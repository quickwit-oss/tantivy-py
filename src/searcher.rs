#![allow(clippy::new_ret_no_self)]

use crate::document::Document;
use crate::query::Query;
use crate::{to_pyerr, get_field};
use pyo3::prelude::*;
use pyo3::PyObjectProtocol;
use tantivy as tv;
use tantivy::collector::{MultiCollector, Count, TopDocs};

/// Tantivy's Searcher class
///
/// A Searcher is used to search the index given a prepared Query.
#[pyclass]
pub(crate) struct Searcher {
    pub(crate) inner: tv::LeasedItem<tv::Searcher>,
    pub(crate) schema: tv::schema::Schema,
}

const SORT_BY: &str = "";

#[pyclass]
pub(crate) struct SearchResult {
    pub(crate) hits: Vec<(PyObject, DocAddress)>,
    pub(crate) count: Option<usize>
}

#[pymethods]
impl Searcher {
    /// Search the index with the given query and collect results.
    ///
    /// Args:
    ///     query (Query): The query that will be used for the search.
    ///     collector (Collector): A collector that determines how the search
    ///         results will be collected. Only the TopDocs collector is
    ///         supported for now.
    ///
    /// Returns a list of tuples that contains the scores and DocAddress of the
    /// search results.
    ///
    /// Raises a ValueError if there was an error with the search.
    #[args(limit = 10, sort_by = "SORT_BY", count = true)]
    fn search(
        &self,
        py: Python,
        query: &Query,
        limit: usize,
        count: bool,
        sort_by: &str,
    ) -> PyResult<SearchResult> {
        let field = match sort_by {
            "" => None,
            field_name => Some(get_field(&self.schema, field_name)?)
        };

        let mut multicollector = tv::collector::MultiCollector::new();

        let count_handle = if count {
            Some(multicollector.add_collector(Count))
        } else {
            None
        };


        let (mut multifruit, hits) = match field {
            Some(f) => {
                let collector = tv::collector::TopDocs::with_limit(limit).order_by_u64_field(f);
                let top_docs_handle = multicollector.add_collector(collector);
                let ret = self.inner.search(&query.inner, &multicollector);

                match ret {
                    Ok(mut r) => {
                        let top_docs = top_docs_handle.extract(&mut r);
                        let result: Vec<(PyObject, DocAddress)> =
                            top_docs.iter().map(|(f, d)| ((*f).into_py(py), DocAddress::from(d))).collect();
                        (r, result)
                    }
                    Err(e) => return Err(exceptions::ValueError::py_err(e.to_string())),
                }

            },
            None => {
                let collector = tv::collector::TopDocs::with_limit(limit);
                let top_docs_handle = multicollector.add_collector(collector);
                let ret = self.inner.search(&query.inner, &multicollector);

                match ret {
                    Ok(mut r) => {
                        let top_docs = top_docs_handle.extract(&mut r);
                        let result: Vec<(PyObject, DocAddress)> =
                            top_docs.iter().map(|(f, d)| ((*f).into_py(py), DocAddress::from(d))).collect();
                        (r, result)
                    }
                    Err(e) => return Err(exceptions::ValueError::py_err(e.to_string())),
                }
            }
        };

        let count = match count_handle {
            Some(h) => Some(h.extract(&mut multifruit)),
            None => None
        };

        Ok(SearchResult { hits, count })
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
}

/// DocAddress contains all the necessary information to identify a document
/// given a Searcher object.
///
/// It consists in an id identifying its segment, and its segment-local DocId.
/// The id used for the segment is actually an ordinal in the list of segment
/// hold by a Searcher.
#[pyclass]
pub(crate) struct DocAddress {
    pub(crate) segment_ord: tv::SegmentLocalId,
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
            segment_ord: doc_address.segment_ord(),
            doc: doc_address.doc(),
        }
    }
}

impl Into<tv::DocAddress> for &DocAddress {
    fn into(self) -> tv::DocAddress {
        tv::DocAddress(self.segment_ord(), self.doc())
    }
}

#[pyproto]
impl PyObjectProtocol for Searcher {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "Searcher(num_docs={}, num_segments={})",
            self.inner.num_docs(),
            self.inner.segment_readers().len()
        ))
    }
}
