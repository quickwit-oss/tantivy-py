use crate::to_pyerr;
use pyo3::prelude::*;
use tantivy as tv;
<<<<<<< HEAD

/// Tantivy schema.
///
/// The schema is very strict. To build the schema the `SchemaBuilder` class is
/// provided.
#[pyclass]
=======
// Bring the trait into scope to use methods like `as_str()` on `OwnedValue`.
use tantivy::schema::Value;

/// Tantivy Snippet
///
/// Snippet contains a fragment of a document, and some highlighted
/// parts inside it.
#[pyclass(module = "tantivy.tantivy")]
>>>>>>> upstream/master
pub(crate) struct Snippet {
    pub(crate) inner: tv::Snippet,
}

<<<<<<< HEAD
#[pyclass]
=======
#[pyclass(module = "tantivy.tantivy")]
>>>>>>> upstream/master
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
}

<<<<<<< HEAD
#[pyclass]
=======
#[pyclass(module = "tantivy.tantivy")]
>>>>>>> upstream/master
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

<<<<<<< HEAD
        return Ok(SnippetGenerator {
            field_name: field_name.to_string(),
            inner: generator,
        });
=======
        Ok(SnippetGenerator {
            field_name: field_name.to_string(),
            inner: generator,
        })
>>>>>>> upstream/master
    }

    pub fn snippet_from_doc(&self, doc: &crate::Document) -> crate::Snippet {
        let text: String = doc
            .iter_values_for_field(&self.field_name)
<<<<<<< HEAD
            .flat_map(tv::schema::Value::as_text)
=======
            .flat_map(|ov| ov.as_str())
>>>>>>> upstream/master
            .collect::<Vec<&str>>()
            .join(" ");

        let result = self.inner.snippet(&text);
        Snippet { inner: result }
    }
<<<<<<< HEAD
=======

    pub fn set_max_num_chars(&mut self, max_num_chars: usize) {
        self.inner.set_max_num_chars(max_num_chars);
    }
>>>>>>> upstream/master
}
