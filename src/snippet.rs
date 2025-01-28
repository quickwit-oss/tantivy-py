use crate::to_pyerr;
use pyo3::prelude::*;
use tantivy as tv;
// Bring the trait into scope to use methods like `as_str()` on `OwnedValue`.
use tantivy::schema::Value;

/// Tantivy Snippet
///
/// Snippet contains a fragment of a document, and some highlighted
/// parts inside it.
#[pyclass(module = "tantivy.tantivy")]
pub(crate) struct Snippet {
    pub(crate) inner: tv::Snippet,
}

#[pyclass(module = "tantivy.tantivy")]
pub(crate) struct Range {
    #[pyo3(get)]
    start: usize,
    #[pyo3(get)]
    end: usize,
}

#[pymethods]
impl Snippet {
    pub fn to_html(&self) -> PyResult<String> {
        Ok(self.inner.to_html())
    }

    pub fn highlighted(&self) -> Vec<Range> {
        let highlighted = self.inner.highlighted();
        let results = highlighted
            .iter()
            .map(|r| Range {
                start: r.start,
                end: r.end,
            })
            .collect::<Vec<_>>();
        results
    }

    pub fn fragment(&self) -> PyResult<String> {
        Ok(self.inner.fragment().to_string())
    }
}

#[pyclass(module = "tantivy.tantivy")]
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
        field_name: &str,
    ) -> PyResult<SnippetGenerator> {
        let field = schema
            .inner
            .get_field(field_name)
            .or(Err("field not found"))
            .map_err(to_pyerr)?;
        let generator =
            tv::SnippetGenerator::create(&searcher.inner, query.get(), field)
                .map_err(to_pyerr)?;

        Ok(SnippetGenerator {
            field_name: field_name.to_string(),
            inner: generator,
        })
    }

    pub fn snippet_from_doc(&self, doc: &crate::Document) -> crate::Snippet {
        let text: String = doc
            .iter_values_for_field(&self.field_name)
            .flat_map(|ov| ov.as_str())
            .collect::<Vec<&str>>()
            .join(" ");

        let result = self.inner.snippet(&text);
        Snippet { inner: result }
    }

    pub fn set_max_num_chars(&mut self, max_num_chars: usize) {
        self.inner.set_max_num_chars(max_num_chars);
    }
}
