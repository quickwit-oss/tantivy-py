#![allow(clippy::new_ret_no_self)]

use crate::{document::Document, query::Query, to_pyerr};
use pyo3::types::PyDict;
use pyo3::IntoPyObjectExt;
use pyo3::{basic::CompareOp, exceptions::PyValueError, prelude::*};
use serde::{Deserialize, Serialize};
use tantivy as tv;
use tantivy::aggregation::AggregationCollector;
use tantivy::collector::{Count, MultiCollector, TopDocs};
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
    Order(u64),
}

impl std::fmt::Debug for Fruit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Fruit::Score(s) => f.write_str(&format!("{s}")),
            Fruit::Order(o) => f.write_str(&format!("{o}")),
        }
    }
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
        // TODO: supported fastfield types: u64, i64, f64, bytes, ip and text.
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
                    let collector =
                        collector.order_by_u64_field(order_by, order.into());
                    let top_docs_handle =
                        multicollector.add_collector(collector);
                    let ret = self.inner.search(query.get(), &multicollector);

                    match ret {
                        Ok(mut r) => {
                            let top_docs = top_docs_handle.extract(&mut r);
                            let result: Vec<(Fruit, DocAddress)> = top_docs
                                .iter()
                                .map(|(f, d)| {
                                    (Fruit::Order(*f), DocAddress::from(d))
                                })
                                .collect();
                            (r, result)
                        }
                        Err(e) => {
                            return Err(PyValueError::new_err(e.to_string()))
                        }
                    }
                } else {
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
                }
            };

            let count = count_handle.map(|h| h.extract(&mut multifruit));

            Ok(SearchResult { hits, count })
        })
    }

    #[pyo3(signature = (query, agg))]
    fn aggregate(
        &self,
        py: Python,
        query: &Query,
        agg: Py<PyDict>,
    ) -> PyResult<Py<PyDict>> {
        let py_json = py.import("json")?;
        let agg_query_str = py_json.call_method1("dumps", (agg,))?.to_string();

        let agg_str = py.detach(move || {
            let agg_collector = AggregationCollector::from_aggs(
                serde_json::from_str(&agg_query_str).map_err(to_pyerr)?,
                Default::default(),
            );
            let agg_res = self
                .inner
                .search(query.get(), &agg_collector)
                .map_err(to_pyerr)?;

            serde_json::to_string(&agg_res).map_err(to_pyerr)
        })?;

        let agg_dict = py_json.call_method1("loads", (agg_str,))?;
        let agg_dict = agg_dict.downcast::<PyDict>()?;

        Ok(agg_dict.clone().unbind())
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
        let py_json = py.import("json")?;
        let agg_query = serde_json::json!({
            "cardinality": {
                "cardinality": {
                    "field": field_name,
                }
            }
        });
        let agg_query_str =
            serde_json::to_string(&agg_query).map_err(to_pyerr)?;
        let agg_query_dict: Py<PyDict> =
            py_json.call_method1("loads", (agg_query_str,))?.extract()?;

        let agg_res = self.aggregate(py, query, agg_query_dict)?;
        let agg_res: &Bound<PyDict> = agg_res.bind(py);

        let res = agg_res.get_item("cardinality")?.ok_or_else(|| {
            PyValueError::new_err("Unexpected aggregation result")
        })?;
        let res_dict: &Bound<PyDict> = res.downcast()?;
        let value = res_dict.get_item("value")?.ok_or_else(|| {
            PyValueError::new_err("Unexpected aggregation result")
        })?;
        let res = value.extract::<f64>()?;

        Ok(res)
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
        field_name: &str,
        field_value: &Bound<PyAny>,
    ) -> PyResult<u64> {
        // Wrap the tantivy Searcher `doc_freq` method to return a PyResult.
        let schema = self.inner.schema();
        let term = crate::make_term(schema, field_name, field_value)?;
        self.inner.doc_freq(&term).map_err(to_pyerr)
    }

    /// Fetches a document from Tantivy's store given a DocAddress.
    ///
    /// Args:
    ///     doc_address (DocAddress): The DocAddress that is associated with
    ///         the document that we wish to fetch.
    ///
    /// Returns the Document, raises ValueError if the document can't be found.
    fn doc(&self, doc_address: &DocAddress) -> PyResult<Document> {
        let doc: TantivyDocument =
            self.inner.doc(doc_address.into()).map_err(to_pyerr)?;
        let named_doc = doc.to_named_doc(self.inner.schema());
        Ok(crate::document::Document {
            field_values: named_doc.0,
        })
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
