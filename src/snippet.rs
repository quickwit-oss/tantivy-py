use pyo3::prelude::*;
use tantivy as tv;
use crate::{
    to_pyerr,
};

/// Tantivy schema.
///
/// The schema is very strict. To build the schema the `SchemaBuilder` class is
/// provided.
#[pyclass]
pub(crate) struct Snippet {
    pub(crate) inner: tv::Snippet,
}

#[pyclass]
pub(crate) struct Range {
    #[pyo3(get)]
    start: usize,
    #[pyo3(get)]
    end: usize
}

#[pymethods]
impl Snippet {
    pub fn to_html(&self) -> PyResult<String> {
        Ok(self.inner.to_html())
    }

    pub fn highlighted(&self) -> Vec<Range> {
        let highlighted =  self.inner.highlighted();
        let results = highlighted.iter().map(|r| Range { start: r.start, end: r.end }).collect::<Vec<_>>();
        results
    }
}


#[pyclass]
pub(crate) struct SnippetGenerator {
    pub(crate) field_name: String,
    pub(crate) inner: tv::SnippetGenerator,
}

#[pymethods]
impl SnippetGenerator {
    #[staticmethod]
    pub fn create(
        searcher: &crate::Searcher,
        query: &crate::Query,
        schema: &crate::Schema,
        field_name: &str
    ) -> PyResult<SnippetGenerator> {
        let field = schema.inner.get_field(field_name).ok_or("field not found").map_err(to_pyerr)?;
        let generator = tv::SnippetGenerator::create(&*searcher.inner, query.get(), field).map_err(to_pyerr)?;

        return Ok(SnippetGenerator { field_name: field_name.to_string(), inner: generator });
    }

    pub fn snippet_from_doc(&self, doc: &crate::Document) -> crate::Snippet {
        let text: String = doc
            .iter_values_for_field(&self.field_name)
            .flat_map(tv::schema::Value::text)
            .collect::<Vec<&str>>()
            .join(" ");

        let result = self.inner.snippet(&text);
        Snippet { inner: result }
    }
}
