#![allow(clippy::new_ret_no_self)]

use std::collections::BTreeSet;
use std::iter::FromIterator;

use crate::more_collectors::StatsCollector;
use crate::{document::Document, query::Query, to_pyerr};
use pyo3::types::PySet;
use pyo3::{exceptions::PyValueError, prelude::*};
use tantivy as tv;
use tantivy::collector::FilterCollector;

/// Tantivy's Searcher class
///
/// A Searcher is used to search the index given a prepared Query.
#[pyclass]
pub(crate) struct StatSearcher {
    pub(crate) inner: tv::Searcher,
}

#[derive(Clone)]
enum Fruit {
    Score(f32),
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

impl ToPyObject for Fruit {
    fn to_object(&self, py: Python) -> PyObject {
        match self {
            Fruit::Score(s) => s.to_object(py),
            Fruit::Order(o) => o.to_object(py),
        }
    }
}

#[pyclass]
/// Object holding a results successful search.
pub(crate) struct SearchResult {
    #[pyo3(get)]
    hits: Vec<(u64, u64, u64, f32)>,
}

#[pymethods]
impl SearchResult {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("SearchResult({:?} hits)", self.hits.len()))
    }

    #[getter]
    fn unique_docs(&self, py: Python) -> PyResult<BTreeSet<u64>> {
        let s =
            BTreeSet::from_iter(self.hits.iter().map(|(d, f, s, score)| *d));
        Ok(s)
    }

    #[getter]
    fn unique_frames(&self, py: Python) -> PyResult<BTreeSet<u64>> {
        let s =
            BTreeSet::from_iter(self.hits.iter().map(|(d, f, s, score)| *f));
        Ok(s)
    }

    #[getter]
    fn unique_docs_frames(&self, py: Python) -> PyResult<BTreeSet<(u64, u64)>> {
        let s = BTreeSet::from_iter(
            self.hits.iter().map(|(d, f, s, score)| (*d, *f)),
        );
        Ok(s)
    }
}

#[pymethods]
impl StatSearcher {
    /// Search the index with the given query and collect results.
    ///
    /// Args:
    ///     query (Query): The query that will be used for the search.
    ///     allowed_frame_ids: A set of frame ids. Only frame ids that are part of this
    ///         set will be retured as part of the search result. Any other match will be
    ///         discarded.
    ///         To include all results provide an empty set.
    ///
    /// Returns `SearchResult` object.
    ///
    /// Raises a ValueError if there was an error with the search.
    #[pyo3(signature = (query, allowed_frame_ids))]
    fn search(
        &self,
        _py: Python,
        query: &Query,
        allowed_frame_ids: &PySet,
    ) -> PyResult<SearchResult> {
        let frame_id = self.inner.schema().get_field("frame_id__").ok_or(
            PyValueError::new_err("Field frame_id__ not found".to_string()),
        )?;

        let mut filter_active = false;
        let mut analysis_filter: BTreeSet<u64> = BTreeSet::new();
        if !allowed_frame_ids.is_empty() {
            filter_active = true;
            analysis_filter.extend(
                allowed_frame_ids
                    .iter()
                    .map(|v| Ok::<u64, PyErr>(v.extract::<u64>()?))
                    .flatten(),
            );
        }

        let sc = StatsCollector::new();
        let ret = if filter_active {
            self.inner.search(
                query.get(),
                &FilterCollector::new(
                    frame_id,
                    move |value: u64| {
                        filter_active & analysis_filter.contains(&value)
                    },
                    sc,
                ),
            )
        } else {
            self.inner.search(query.get(), &sc)
        };

        let search_result = match ret {
            Ok(result) => {
                if let Some(stats) = result {
                    SearchResult { hits: stats.hits }
                } else {
                    SearchResult { hits: vec![] }
                }
            }
            Err(e) => return Err(PyValueError::new_err(e.to_string())),
        };

        Ok(search_result)
    }

    /// Returns the overall number of documents in the index.
    #[getter]
    fn num_docs(&self) -> u64 {
        self.inner.num_docs()
    }

    /// Fetches a document from Tantivy's store given a DocAddress.
    ///
    /// Args:
    ///     doc_address (DocAddress): The DocAddress that is associated with
    ///         the document that we wish to fetch.
    ///
    /// Returns the Document, raises ValueError if the document can't be found.
    fn doc(&self, doc_address: &DocAddress) -> PyResult<Document> {
        let doc = self.inner.doc(doc_address.into()).map_err(to_pyerr)?;
        let named_doc = self.inner.schema().to_named_doc(&doc);
        Ok(Document {
            field_values: named_doc.0,
        })
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "StatSearcher(num_docs={}, num_segments={})",
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
#[pyclass]
#[derive(Clone, Debug)]
pub(crate) struct DocAddress {
    pub(crate) segment_ord: tv::SegmentOrdinal,
    pub(crate) doc: tv::DocId,
}

#[pymethods]
impl DocAddress {
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
