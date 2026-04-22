#![allow(clippy::new_ret_no_self)]

use crate::{document::Document, query::Query, to_pyerr};
use pyo3::types::PyDict;
use pyo3::IntoPyObjectExt;
use pyo3::{basic::CompareOp, exceptions::PyValueError, prelude::*};
use pythonize::{depythonize, pythonize};
use serde::{Deserialize, Serialize};
use tantivy as tv;
use tantivy::aggregation::AggregationCollector;
use tantivy::collector::{Count, MultiCollector, TopDocs};
use tantivy::columnar::MonotonicallyMappableToU64;
use tantivy::TantivyDocument;

// Bring the trait into scope. This is required for the `to_named_doc` method.
// However, tantivy-py declares its own `Document` class, so we need to avoid
// introduce the `Document` trait into the namespace.
use tantivy::Document as _;

/// Tantivy's Searcher class
///
/// A Searcher is used to search the index given a prepared Query.
#[pyclass(module = "tantivy.tantivy")]
pub(crate) struct Searcher {
    pub(crate) inner: tv::Searcher,
}

#[derive(
    Clone, Deserialize, PartialEq, Serialize, FromPyObject, IntoPyObject,
)]
enum Fruit {
    #[pyo3(transparent)]
    Score(f32),
    #[pyo3(transparent)]
    OrderU64(Option<u64>),
    #[pyo3(transparent)]
    OrderStr(Option<String>),
}

impl std::fmt::Debug for Fruit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Fruit::Score(s) => f.write_str(&format!("{s}")),
            Fruit::OrderU64(Some(o)) => f.write_str(&format!("{o}")),
            Fruit::OrderU64(None) => f.write_str("None"),
            Fruit::OrderStr(Some(s)) => f.write_str(&format!("{s}")),
            Fruit::OrderStr(None) => f.write_str("None"),
        }
    }
}

/// Open one column per segment, map each DocAddress to its value, and wrap in
/// the given `FastFieldValue` variant.  Used by `fast_field_values()` to avoid
/// repeating the same iterator chain for each numeric type.
macro_rules! read_fast_field_column_values {
    ($readers:expr, $field:expr, $addrs:expr, $method:ident, $variant:path) => {{
        let columns: Vec<Option<_>> = $readers
            .iter()
            .map(|reader| reader.fast_fields().$method($field).ok())
            .collect();
        Ok($addrs
            .iter()
            .map(|addr| {
                columns[addr.segment_ord as usize]
                    .as_ref()
                    .and_then(|col| col.first(addr.doc))
                    .map($variant)
            })
            .collect())
    }};
}

/// A typed value read from a numeric fast field.
///
/// PyO3 converts U64/I64 to Python int and F64 to Python float, so Python
/// callers receive `int | float` without needing to know the underlying type.
#[derive(IntoPyObject)]
enum FastFieldValue {
    #[pyo3(transparent)]
    U64(u64),
    #[pyo3(transparent)]
    I64(i64),
    #[pyo3(transparent)]
    F64(f64),
    #[pyo3(transparent)]
    Bool(bool),
}

#[pyclass(frozen, module = "tantivy.tantivy")]
#[derive(Clone, Copy, Deserialize, PartialEq, Serialize)]
/// Enum representing the direction in which something should be sorted.
pub(crate) enum Order {
    /// Ascending. Smaller values appear first.
    Asc,

    /// Descending. Larger values appear first.
    Desc,
}

impl From<Order> for tv::Order {
    fn from(order: Order) -> Self {
        match order {
            Order::Asc => tv::Order::Asc,
            Order::Desc => tv::Order::Desc,
        }
    }
}

#[pyclass(frozen, module = "tantivy.tantivy")]
#[derive(Clone, Default, Deserialize, PartialEq, Serialize)]
/// Object holding a results successful search.
pub(crate) struct SearchResult {
    hits: Vec<(Fruit, DocAddress)>,
    #[pyo3(get)]
    /// How many documents matched the query. Only available if `count` was set
    /// to true during the search.
    count: Option<usize>,
}

#[pymethods]
impl SearchResult {
    #[new]
    fn new(
        py: Python,
        hits: Vec<(Py<PyAny>, DocAddress)>,
        count: Option<usize>,
    ) -> PyResult<Self> {
        let hits = hits
            .iter()
            .map(|(f, d)| Ok((f.extract(py)?, d.clone())))
            .collect::<PyResult<Vec<_>>>()?;
        Ok(Self { hits, count })
    }

    fn __repr__(&self) -> PyResult<String> {
        if let Some(count) = self.count {
            Ok(format!(
                "SearchResult(hits: {:?}, count: {})",
                self.hits, count
            ))
        } else {
            Ok(format!("SearchResult(hits: {:?})", self.hits))
        }
    }

    fn __richcmp__(
        &self,
        other: &Self,
        op: CompareOp,
        py: Python<'_>,
    ) -> PyResult<Py<PyAny>> {
        match op {
            CompareOp::Eq => (self == other).into_py_any(py),
            CompareOp::Ne => (self != other).into_py_any(py),
            _ => Ok(py.NotImplemented()),
        }
    }

    fn __getnewargs__(
        &self,
        py: Python,
    ) -> PyResult<(Vec<(Py<PyAny>, DocAddress)>, Option<usize>)> {
        Ok((self.hits(py)?, self.count))
    }

    #[getter]
    /// The list of tuples that contains the scores and DocAddress of the
    /// search results.
    fn hits(&self, py: Python) -> PyResult<Vec<(Py<PyAny>, DocAddress)>> {
        let ret = self
            .hits
            .iter()
            .map(|(result, address)| -> PyResult<_> {
                Ok((result.clone().into_py_any(py)?, address.clone()))
            })
            .collect::<PyResult<_>>()?;
        Ok(ret)
    }
}

impl Searcher {
    /// Execute an aggregation from an already-deserialized spec.
    /// Shared by `aggregate()` and `cardinality()` so neither needs to
    /// round-trip through JSON or Python when the spec is already a
    /// `serde_json::Value`.
    fn aggregate_value(
        &self,
        py: Python,
        query: &Query,
        aggs: tv::aggregation::agg_req::Aggregations,
    ) -> PyResult<Py<PyDict>> {
        let agg_res = py.detach(move || {
            let agg_collector =
                AggregationCollector::from_aggs(aggs, Default::default());
            self.inner
                .search(query.get(), &agg_collector)
                .map_err(to_pyerr)
        })?;

        pythonize(py, &agg_res)
            .map_err(to_pyerr)?
            .downcast_into::<PyDict>()
            .map(|d| d.unbind())
            .map_err(Into::into)
    }
}

#[pymethods]
impl Searcher {
    /// Search the index with the given query and collect results.
    ///
    /// Args:
    ///     query (Query): The query that will be used for the search.
    ///     limit (int, optional): The maximum number of search results to
    ///         return. Defaults to 10.
    ///     count (bool, optional): Should the number of documents that match
    ///         the query be returned as well. Defaults to true.
    ///     order_by_field (Field, optional): A schema field that the results
    ///         should be ordered by. The field must be declared as a fast field
    ///         when building the schema. Note, this only works for unsigned
    ///         fields.
    ///     offset (Field, optional): The offset from which the results have
    ///         to be returned.
    ///     order (Order, optional): The order in which the results
    ///         should be sorted. If not specified, defaults to descending.
    ///     weight_by_field (Field, optional): A schema field that the results
    ///         should be weighted by. The field must be declared as a fast
    ///         field when building the schema. Note, this only works for
    ///         f64, i64 and u64 fields. The given field value is first
    ///         transformed using the formula `log2(2.0 + value)` and then
    ///         multiplied with the original score. This means that a weight field
    ///         value of 0.0 results in no change to the original score.
    ///         If the weight value is negative, it is treated as 0.0.
    ///
    /// Returns `SearchResult` object.
    ///
    /// Raises a ValueError if there was an error with the search.
    #[pyo3(signature = (query, limit = 10, count = true, order_by_field = None, offset = 0, order = Order::Desc,
            weight_by_field = None))]
    #[allow(clippy::too_many_arguments)]
    fn search(
        &self,
        py: Python,
        query: &Query,
        limit: usize,
        count: bool,
        order_by_field: Option<&str>,
        offset: usize,
        order: Order,
        weight_by_field: Option<&str>,
    ) -> PyResult<SearchResult> {
        py.detach(move || {
            let mut multicollector = MultiCollector::new();

            let count_handle = if count {
                Some(multicollector.add_collector(Count))
            } else {
                None
            };

            let (mut multifruit, hits) = {
                let collector = TopDocs::with_limit(limit).and_offset(offset);
                if let Some(weight_by_field) = weight_by_field {
                    let weight_by_field = weight_by_field.to_string();

                    // Get field type from schema
                    let schema = self.inner.schema();
                    let field = crate::get_field(&schema, &weight_by_field)
                        .map_err(|e| PyValueError::new_err(e.to_string()))?;
                    let field_entry = schema.get_field_entry(field);
                    let field_type = field_entry.field_type().value_type();

                    if !field_entry.is_fast() {
                        return Err(PyValueError::new_err(format!(
                            "Field '{}' is not a fast field. The field must be declared with fast=True in the schema.",
                            weight_by_field
                        )));
                    }

                    // Check if field type is supported
                    if !matches!(field_type, tv::schema::Type::F64 | tv::schema::Type::I64 | tv::schema::Type::U64) {
                        return Err(PyValueError::new_err(format!(
                            "Unsupported field type for weighting: {:?}. Only f64, i64, and u64 fastfields are supported.",
                            field_type
                        )));
                    }

                    let collector = collector.tweak_score(
                        move |segment_reader: &tv::SegmentReader| {
                            // Create all three readers upfront. Only one will succeed based on
                            // the actual field type, but we must create all three because:
                            // 1. Rust closures have a single concrete type - we can't return
                            //    different closure types from different match arms
                            // 2. The alternative (Box<dyn Fn>) adds heap allocation per segment
                            //    and virtual dispatch overhead per document
                            // 3. This approach enables monomorphization: the inner closure has
                            //    a concrete type, allowing LLVM to inline get_val() calls
                            let f64_reader = segment_reader
                                .fast_fields()
                                .f64(&weight_by_field)
                                .ok()
                                .map(|r| r.first_or_default_col(0.0));
                            let i64_reader = segment_reader
                                .fast_fields()
                                .i64(&weight_by_field)
                                .ok()
                                .map(|r| r.first_or_default_col(0));
                            let u64_reader = segment_reader
                                .fast_fields()
                                .u64(&weight_by_field)
                                .ok()
                                .map(|r| r.first_or_default_col(0));

                            move |doc: tv::DocId, original_score: tv::Score| {
                                let value: f64 = match field_type {
                                    // Runtime type dispatch is required here even though field_type
                                    // was checked earlier because:
                                    // 1. field_type is moved into this closure and can't be matched
                                    //    at compile time to select which reader to use
                                    // 2. All three readers must exist at this point for the closure
                                    //    to have a single concrete type
                                    //
                                    // Use map_or(0.0, ...) instead of unwrap() because segments
                                    // created before a schema change may lack this fast field.
                                    // Default value 0.0 results in neutral scoring:
                                    // boost = log2(2.0 + 0.0) = 1.0, so score * 1.0 = score
                                    tv::schema::Type::F64 => f64_reader.as_ref().map_or(0.0, |r| r.get_val(doc)),
                                    tv::schema::Type::I64 => i64_reader.as_ref().map_or(0.0, |r| r.get_val(doc) as f64),
                                    tv::schema::Type::U64 => u64_reader.as_ref().map_or(0.0, |r| r.get_val(doc) as f64),
                                    _ => unreachable!(),
                                };
                                let value = value.max(0.0); // Negative values are not allowed
                                let value_boost_score = ((2f64 + value) as tv::Score).log2();
                                value_boost_score * original_score
                            }
                        },
                    );
                    let top_docs_handle =
                        multicollector.add_collector(collector);
                    let ret = self.inner.search(query.get(), &multicollector);
                    match ret {
                        Ok(mut r) => {
                            let top_docs = top_docs_handle.extract(&mut r);
                            let result: Vec<(Fruit, DocAddress)> = top_docs
                                .iter()
                                .map(|(f, d)| {
                                    (Fruit::Score(*f), DocAddress::from(d))
                                })
                                .collect();
                            (r, result)
                        }
                        Err(e) => {
                            return Err(PyValueError::new_err(e.to_string()))
                        }
                    }
                } else if let Some(order_by) = order_by_field {
                    let schema = self.inner.schema();
                    let field = crate::get_field(&schema, order_by)
                        .map_err(|e| PyValueError::new_err(e.to_string()))?;
                    let field_type =
                        schema.get_field_entry(field).field_type().value_type();
                    macro_rules! run_order_by_fast {
                        ($t:ty) => {{
                            let top_docs_handle = multicollector.add_collector(
                                collector.order_by_fast_field::<$t>(order_by, order.into()),
                            );
                            let ret =
                                self.inner.search(query.get(), &multicollector);
                            match ret {
                                Ok(mut r) => {
                                    let top_docs = top_docs_handle.extract(&mut r);
                                    let result: Vec<(Fruit, DocAddress)> = top_docs
                                        .into_iter()
                                        .map(|(f, d)| {
                                            (
                                                Fruit::OrderU64(f.map(|v| v.to_u64())),
                                                DocAddress::from(&d),
                                            )
                                        })
                                        .collect();
                                    (r, result)
                                }
                                Err(e) => {
                                    return Err(PyValueError::new_err(e.to_string()))
                                }
                            }
                        }};
                    }
                    match field_type {
                        tv::schema::Type::U64  => run_order_by_fast!(u64),
                        tv::schema::Type::I64  => run_order_by_fast!(i64),
                        tv::schema::Type::F64  => run_order_by_fast!(f64),
                        tv::schema::Type::Bool => run_order_by_fast!(bool),
                        tv::schema::Type::Date => run_order_by_fast!(tv::DateTime),
                        tv::schema::Type::Str  => {
                            let top_docs_handle = multicollector.add_collector(
                                collector.order_by_string_fast_field(order_by, order.into()),
                            );
                            let ret =
                                self.inner.search(query.get(), &multicollector);
                            match ret {
                                Ok(mut r) => {
                                    let top_docs = top_docs_handle.extract(&mut r);
                                    let result: Vec<(Fruit, DocAddress)> = top_docs
                                        .into_iter()
                                        .map(|(f, d)| {
                                            (
                                                Fruit::OrderStr(f),
                                                DocAddress::from(&d),
                                            )
                                        })
                                        .collect();
                                    (r, result)
                                }
                                Err(e) => {
                                    return Err(PyValueError::new_err(e.to_string()))
                                }
                            }
                        }
                        other => {
                            return Err(PyValueError::new_err(format!(
                                "Field '{}' has type {:?}; order_by_field only supports \
                                 U64, I64, F64, Bool, Date, and Str fast fields.",
                                order_by, other
                            )));
                        }
                    }
                } else {
                    let top_docs_handle = multicollector
                        .add_collector(collector.order_by_score());
                    let ret = self.inner.search(query.get(), &multicollector);

                    match ret {
                        Ok(mut r) => {
                            let top_docs = top_docs_handle.extract(&mut r);
                            let result: Vec<(Fruit, DocAddress)> = top_docs
                                .iter()
                                .map(|(f, d)| {
                                    (Fruit::Score(*f), DocAddress::from(d))
                                })
                                .collect();
                            (r, result)
                        }
                        Err(e) => {
                            return Err(PyValueError::new_err(e.to_string()))
                        }
                    }
                }
            };

            let count = count_handle.map(|h| h.extract(&mut multifruit));

            Ok(SearchResult { hits, count })
        })
    }

    /// Execute an aggregation query and return the results as a dict.
    ///
    /// Args:
    ///     query (Query): The query that filters the documents to aggregate over.
    ///     agg (dict): The aggregation specification as a Python dict.
    ///
    /// Returns a dict containing the aggregation results.
    #[pyo3(signature = (query, agg))]
    fn aggregate(
        &self,
        py: Python,
        query: &Query,
        agg: Py<PyDict>,
    ) -> PyResult<Py<PyDict>> {
        let aggs: tv::aggregation::agg_req::Aggregations =
            depythonize(agg.bind(py)).map_err(to_pyerr)?;
        self.aggregate_value(py, query, aggs)
    }

    /// Returns the cardinality of a query.
    ///
    /// Args:
    ///     query (Query): The query that will be used for the search.
    ///     field_name (str): The field for which to compute the cardinality.
    ///
    /// Returns the cardinality.
    #[pyo3(signature = (query, field_name))]
    fn cardinality(
        &self,
        py: Python,
        query: &Query,
        field_name: &str,
    ) -> PyResult<f64> {
        let agg_spec = serde_json::json!({
            "cardinality": {
                "cardinality": {
                    "field": field_name,
                }
            }
        });

        let aggs = serde_json::from_value(agg_spec).map_err(to_pyerr)?;
        let agg_res = self.aggregate_value(py, query, aggs)?;
        let agg_res: &Bound<PyDict> = agg_res.bind(py);

        let res = agg_res.get_item("cardinality")?.ok_or_else(|| {
            PyValueError::new_err("Unexpected aggregation result")
        })?;
        let res_dict: &Bound<PyDict> = res.downcast()?;
        let value = res_dict.get_item("value")?.ok_or_else(|| {
            PyValueError::new_err("Unexpected aggregation result")
        })?;
        value.extract::<f64>()
    }

    /// Returns the overall number of documents in the index.
    #[getter]
    fn num_docs(&self) -> u64 {
        self.inner.num_docs()
    }

    /// Returns the number of segments in the index.
    #[getter]
    fn num_segments(&self) -> usize {
        self.inner.segment_readers().len()
    }

    /// Return the overall number of documents containing
    /// the given term.
    #[pyo3(signature = (field_name, field_value))]
    fn doc_freq(
        &self,
        py: Python,
        field_name: &str,
        field_value: &Bound<PyAny>,
    ) -> PyResult<u64> {
        // make_term() needs the GIL (Python type extraction); doc_freq() does not.
        let schema = self.inner.schema();
        let term = crate::make_term(schema, field_name, field_value)?;
        py.detach(move || self.inner.doc_freq(&term).map_err(to_pyerr))
    }

    /// Fetches a document from Tantivy's store given a DocAddress.
    ///
    /// Args:
    ///     doc_address (DocAddress): The DocAddress that is associated with
    ///         the document that we wish to fetch.
    ///
    /// Returns the Document, raises ValueError if the document can't be found.
    fn doc(&self, py: Python, doc_address: &DocAddress) -> PyResult<Document> {
        let addr: tv::DocAddress = doc_address.into();
        py.detach(move || {
            let doc: TantivyDocument =
                self.inner.doc(addr).map_err(to_pyerr)?;
            let named_doc = doc.to_named_doc(self.inner.schema());
            Ok(crate::document::Document {
                field_values: named_doc.0,
            })
        })
    }

    /// Read a numeric fast field for a batch of DocAddresses without fetching
    /// stored documents.
    ///
    /// Fast fields are column-oriented and support O(1) random access by
    /// segment-local DocId.  Use this instead of doc().to_dict()[field] when
    /// you only need a single numeric field for many documents.
    ///
    /// The field type is resolved from the schema automatically: u64 and i64
    /// fields return Python int; f64 fields return Python float; bool fields
    /// return Python bool.
    ///
    /// Args:
    ///     field_name: Name of a u64, i64, f64, or bool field declared with fast=True.
    ///     doc_addresses: List of DocAddress objects (e.g. from search().hits).
    ///
    /// Returns:
    ///     A list of values in the same order as doc_addresses.
    ///     None is returned for any address where the column is absent
    ///     (e.g. a segment written before the field was added to the schema).
    ///
    /// Raises:
    ///     ValueError: if the field does not exist, is not a fast field, or
    ///         has an unsupported type (only u64, i64, f64, and bool are supported).
    #[pyo3(signature = (field_name, doc_addresses))]
    fn fast_field_values(
        &self,
        field_name: &str,
        doc_addresses: Vec<DocAddress>,
    ) -> PyResult<Vec<Option<FastFieldValue>>> {
        let schema = self.inner.schema();

        let field = schema.get_field(field_name).map_err(|_| {
            PyValueError::new_err(format!("Unknown field: '{field_name}'"))
        })?;
        let field_entry = schema.get_field_entry(field);
        if !field_entry.is_fast() {
            return Err(PyValueError::new_err(format!(
                "Field '{field_name}' is not a fast field."
            )));
        }

        let field_type = field_entry.field_type().value_type();
        let segment_readers = self.inner.segment_readers();
        let num_segments = segment_readers.len();

        // Validate all segment_ords before reading so we don't produce a
        // partial result on error.
        for doc_address in &doc_addresses {
            if doc_address.segment_ord as usize >= num_segments {
                return Err(PyValueError::new_err(format!(
                    "Invalid segment_ord: {}",
                    doc_address.segment_ord
                )));
            }
        }

        // Pre-open one Column per segment so it is not reopened per document.
        // Column::first() returns Option<T>, so no sentinel value is needed.
        match field_type {
            tv::schema::Type::U64 => read_fast_field_column_values!(
                segment_readers,
                field_name,
                doc_addresses,
                u64,
                FastFieldValue::U64
            ),
            tv::schema::Type::I64 => read_fast_field_column_values!(
                segment_readers,
                field_name,
                doc_addresses,
                i64,
                FastFieldValue::I64
            ),
            tv::schema::Type::F64 => read_fast_field_column_values!(
                segment_readers,
                field_name,
                doc_addresses,
                f64,
                FastFieldValue::F64
            ),
            tv::schema::Type::Bool => read_fast_field_column_values!(
                segment_readers,
                field_name,
                doc_addresses,
                bool,
                FastFieldValue::Bool
            ),
            _ => Err(PyValueError::new_err(format!(
                "Field '{field_name}' has unsupported type for fast field access. \
                 Only u64, i64, f64, and bool fast fields are supported."
            ))),
        }
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "Searcher(num_docs={}, num_segments={})",
            self.inner.num_docs(),
            self.inner.segment_readers().len()
        ))
    }
}

/// DocAddress contains all the necessary information to identify a document
/// given a Searcher object.
///
/// It consists in an id identifying its segment, and its segment-local DocId.
/// The id used for the segment is actually an ordinal in the list of segment
/// hold by a Searcher.
#[pyclass(frozen, module = "tantivy.tantivy")]
#[derive(
    Clone, Debug, Deserialize, PartialEq, PartialOrd, Eq, Ord, Serialize,
)]
pub(crate) struct DocAddress {
    pub(crate) segment_ord: tv::SegmentOrdinal,
    pub(crate) doc: tv::DocId,
}

#[pymethods]
impl DocAddress {
    #[new]
    fn new(segment_ord: tv::SegmentOrdinal, doc: tv::DocId) -> Self {
        DocAddress { segment_ord, doc }
    }

    /// The segment ordinal is an id identifying the segment hosting the
    /// document. It is only meaningful, in the context of a searcher.
    #[getter]
    fn segment_ord(&self) -> u32 {
        self.segment_ord
    }

    /// The segment local DocId
    #[getter]
    fn doc(&self) -> u32 {
        self.doc
    }

    fn __richcmp__(
        &self,
        other: &Self,
        op: CompareOp,
        _py: Python<'_>,
    ) -> bool {
        op.matches(self.cmp(other))
    }

    fn __getnewargs__(&self) -> PyResult<(tv::SegmentOrdinal, tv::DocId)> {
        Ok((self.segment_ord, self.doc))
    }
}

impl From<&tv::DocAddress> for DocAddress {
    fn from(doc_address: &tv::DocAddress) -> Self {
        DocAddress {
            segment_ord: doc_address.segment_ord,
            doc: doc_address.doc_id,
        }
    }
}

impl From<&DocAddress> for tv::DocAddress {
    fn from(val: &DocAddress) -> Self {
        tv::DocAddress {
            segment_ord: val.segment_ord(),
            doc_id: val.doc(),
        }
    }
}
