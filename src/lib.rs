use ::tantivy as tv;
use ::tantivy::schema::{OwnedValue as Value, Term};
use pyo3::{exceptions, prelude::*, wrap_pymodule};

mod document;
mod explanation;
mod facet;
mod index;
mod parser_error;
mod query;
mod schema;
mod schemabuilder;
mod searcher;
mod snippet;
mod tokenizer;

use document::{extract_value, extract_value_for_type, Document};
use explanation::Explanation;
use facet::Facet;
use index::Index;
use query::{Occur, Query};
use schema::{FieldType, Schema};
use schemabuilder::SchemaBuilder;
use searcher::{DocAddress, Order, SearchResult, Searcher};
use snippet::{Snippet, SnippetGenerator};
use tokenizer::{Filter, TextAnalyzer, TextAnalyzerBuilder, Tokenizer};

/// Python bindings for the search engine library Tantivy.
///
/// Tantivy is a full text search engine library written in rust.
///
/// It is closer to Apache Lucene than to Elasticsearch and Apache Solr in
/// the sense it is not an off-the-shelf search engine server, but rather
/// a library that can be used to build such a search engine.
/// Tantivy is, in fact, strongly inspired by Lucene's design.
///
/// Example:
///     >>> import json
///     >>> import tantivy
///
///     >>> builder = tantivy.SchemaBuilder()
///
///     >>> title = builder.add_text_field("title", stored=True)
///     >>> body = builder.add_text_field("body")
///
///     >>> schema = builder.build()
///     >>> index = tantivy.Index(schema)
///     >>> doc = tantivy.Document()
///     >>> doc.add_text(title, "The Old Man and the Sea")
///     >>> doc.add_text(body, ("He was an old man who fished alone in a "
///                             "skiff in the Gulf Stream and he had gone "
///                             "eighty-four days now without taking a fish."))
///
///     >>> writer.add_document(doc)
///
///     >>> doc = schema.parse_document(json.dumps({
///            "title": ["Frankenstein", "The Modern Prometheus"],
///            "body": ("You will rejoice to hear that no disaster has "
///                     "accompanied the commencement of an enterprise which "
///                     "you have regarded with such evil forebodings.  "
///                     "I arrived here yesterday, and my first task is to "
///                     "assure my dear sister of my welfare and increasing "
///                     "confidence in the success of my undertaking.")
///     }))
///
///     >>> writer.add_document(doc)
///     >>> writer.commit()
///
///     >>> reader = index.reader()
///     >>> searcher = reader.searcher()
///
///     >>> query = index.parse_query("sea whale", [title, body])
///
///     >>> result = searcher.search(query, 10)
///
///     >>> assert len(result) == 1
///
#[pymodule]
fn tantivy(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    m.add_class::<Order>()?;
    m.add_class::<Schema>()?;
    m.add_class::<SchemaBuilder>()?;
    m.add_class::<Searcher>()?;
    m.add_class::<SearchResult>()?;
    m.add_class::<Document>()?;
    m.add_class::<Index>()?;
    m.add_class::<DocAddress>()?;
    m.add_class::<Facet>()?;
    m.add_class::<Query>()?;
    m.add_class::<Explanation>()?;
    m.add_class::<Snippet>()?;
    m.add_class::<SnippetGenerator>()?;
    m.add_class::<Occur>()?;
    m.add_class::<FieldType>()?;
    m.add_class::<Tokenizer>()?;
    m.add_class::<TextAnalyzerBuilder>()?;
    m.add_class::<Filter>()?;
    m.add_class::<TextAnalyzer>()?;

    m.add_wrapped(wrap_pymodule!(query_parser_error))?;

    m.add("__version__", tv::version_string())?;

    Ok(())
}

/// Submodule containing all the possible errors that can be raised during
/// query parsing.
///
/// Example:
///     >>> import tantivy
///     >>> from tantivy import query_parser_error
///
///     >>> builder = tantivy.SchemaBuilder()
///
///     >>> title = builder.add_text_field("title", stored=True)
///     >>> body = builder.add_text_field("body")
///     >>> id = builder.add_unsigned_field("id")
///     >>> rating = builder.add_float_field("rating")
///
///     >>> schema = builder.build()
///     >>> index = tantivy.Index(schema)
///
///     >>> query, errors = index.parse_query_lenient(
///             "bod:'world' AND id:<3.5 AND rating:5.0"
///         )
///
///     >>> assert len(errors) == 2
///     >>> assert isinstance(errors[0], query_parser_error.FieldDoesNotExistError)
///     >>> assert isinstance(errors[1], query_parser_error.ExpectedIntError)
#[pymodule]
fn query_parser_error(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    m.add_class::<parser_error::SyntaxError>()?;
    m.add_class::<parser_error::UnsupportedQueryError>()?;
    m.add_class::<parser_error::FieldDoesNotExistError>()?;
    m.add_class::<parser_error::ExpectedIntError>()?;
    m.add_class::<parser_error::ExpectedBase64Error>()?;
    m.add_class::<parser_error::ExpectedFloatError>()?;
    m.add_class::<parser_error::ExpectedBoolError>()?;
    m.add_class::<parser_error::AllButQueryForbiddenError>()?;
    m.add_class::<parser_error::NoDefaultFieldDeclaredError>()?;
    m.add_class::<parser_error::FieldNotIndexedError>()?;
    m.add_class::<parser_error::FieldDoesNotHavePositionsIndexedError>()?;
    m.add_class::<parser_error::PhrasePrefixRequiresAtLeastTwoTermsError>()?;
    m.add_class::<parser_error::UnknownTokenizerError>()?;
    m.add_class::<parser_error::RangeMustNotHavePhraseError>()?;
    m.add_class::<parser_error::DateFormatError>()?;
    m.add_class::<parser_error::FacetFormatError>()?;
    m.add_class::<parser_error::IpFormatError>()?;

    Ok(())
}

pub(crate) fn to_pyerr<E: ToString>(err: E) -> PyErr {
    exceptions::PyValueError::new_err(err.to_string())
}

pub(crate) fn get_field(
    schema: &tv::schema::Schema,
    field_name: &str,
) -> PyResult<tv::schema::Field> {
    let field = schema.get_field(field_name).map_err(|_err| {
        exceptions::PyValueError::new_err(format!(
            "Field `{field_name}` is not defined in the schema."
        ))
    })?;

    Ok(field)
}

pub(crate) fn make_term(
    schema: &tv::schema::Schema,
    field_name: &str,
    field_value: &Bound<PyAny>,
) -> PyResult<tv::Term> {
    let field = get_field(schema, field_name)?;
    let value = extract_value(field_value)?;
    let term = match value {
        Value::Str(text) => Term::from_field_text(field, &text),
        Value::U64(num) => Term::from_field_u64(field, num),
        Value::I64(num) => Term::from_field_i64(field, num),
        Value::F64(num) => Term::from_field_f64(field, num),
        Value::Date(d) => Term::from_field_date(field, d),
        Value::Facet(facet) => Term::from_facet(field, &facet),
        Value::Bool(b) => Term::from_field_bool(field, b),
        Value::IpAddr(i) => Term::from_field_ip_addr(field, i),
        _ => {
            return Err(exceptions::PyValueError::new_err(format!(
                "Can't create a term for Field `{field_name}` with value `{field_value}`."
            )))
        }
    };

    Ok(term)
}

pub(crate) fn make_term_for_type(
    schema: &tv::schema::Schema,
    field_name: &str,
    field_type: FieldType,
    field_value: &Bound<PyAny>,
) -> PyResult<tv::Term> {
    let field = get_field(schema, field_name)?;
    let value =
        extract_value_for_type(field_value, field_type.into(), field_name)?;
    let term = match value {
        Value::Str(text) => Term::from_field_text(field, &text),
        Value::U64(num) => Term::from_field_u64(field, num),
        Value::I64(num) => Term::from_field_i64(field, num),
        Value::F64(num) => Term::from_field_f64(field, num),
        Value::Date(d) => Term::from_field_date(field, d),
        Value::Facet(facet) => Term::from_facet(field, &facet),
        Value::Bool(b) => Term::from_field_bool(field, b),
        Value::IpAddr(i) => Term::from_field_ip_addr(field, i),
        _ => {
            return Err(exceptions::PyValueError::new_err(format!(
                "Can't create a term for Field `{field_name}` with value `{field_value}`."
            )))
        }
    };

    Ok(term)
}
