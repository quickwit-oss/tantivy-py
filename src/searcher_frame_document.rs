#![allow(clippy::new_ret_no_self)]

use std::collections::BTreeSet;
use std::iter::FromIterator;

use crate::more_collectors::StatsCollector;
use crate::{document::Document, query::Query, to_pyerr};
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

    /// This is an optimization to allow Python callers to obtain vectors
    /// without having to do iteration to get them.
    #[getter]
    fn unique_docs_frames_unzipped(
        &self,
        py: Python,
    ) -> PyResult<(Vec<u64>, Vec<u64>)> {
        let s = BTreeSet::from_iter(
            self.hits.iter().map(|(d, f, s, score)| (*d, *f)),
        );
        let mut v1 = Vec::with_capacity(s.len());
        let mut v2 = Vec::with_capacity(s.len());
        for (d, f) in s.into_iter() {
            v1.push(d);
            v2.push(f);
        }
        Ok((v1, v2))
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
    #[pyo3(signature = (query, filter_fastfield_name=None, filter_fastfield_values=None))]
    fn search(
        &self,
        _py: Python,
        query: &Query,
        filter_fastfield_name: Option<String>,
        filter_fastfield_values: Option<BTreeSet<u64>>,
    ) -> PyResult<SearchResult> {
        if filter_fastfield_values.is_some() {
            if filter_fastfield_name.is_none() {
                let msg = format!(
                    "If filter values are provided, the field name must also be given."
                );
                return Err(PyValueError::new_err(msg));
            }
        };

        let sc = StatsCollector::new();

        let ret = if let Some(members) = filter_fastfield_values {
            let field_name = filter_fastfield_name.unwrap();
            let field = self.inner.schema().get_field(&field_name).or({
                let msg = format!("Field {field_name} not found");
                Err(PyValueError::new_err(msg))
            })?;
            self.inner.search(
                query.get(),
                &FilterCollector::new(
                    field,
                    move |value: u64| members.contains(&value),
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
