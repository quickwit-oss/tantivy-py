use crate::to_pyerr;
use pyo3::{exceptions, prelude::*};
use pythonize::pythonize;
use tantivy as tv;

/// Parse a query string into an abstract syntax tree (AST).
///
/// This function parses a query string following Tantivy's query language
/// syntax and returns a Python dictionary representing the parsed AST.
/// Unlike `Index.parse_query()`, this function does not require a schema
/// and returns the raw syntax tree structure.
///
/// Args:
///     query (str): The query string to parse.
///
/// Returns:
///     dict: A dictionary representing the parsed query AST.
///
/// Raises:
///     ValueError: If the query has invalid syntax.
///
/// Example:
///     >>> import tantivy
///     >>> ast = tantivy.parse_query("title:hello AND body:world")
///     >>> print(ast)
#[pyfunction]
pub fn parse_query(py: Python, query: &str) -> PyResult<Py<PyAny>> {
    let ast = tv::query_grammar::parse_query(query).map_err(|e| {
        exceptions::PyValueError::new_err(format!(
            "Query parsing error: {:?}",
            e
        ))
    })?;
    let py_obj = pythonize(py, &ast).map_err(to_pyerr)?;
    Ok(py_obj.unbind())
}

/// Parse a query string leniently, recovering from syntax errors.
///
/// This function attempts to parse a query string even if it contains
/// syntax errors. It returns both the parsed AST and a list of errors
/// encountered during parsing. Unlike `Index.parse_query_lenient()`,
/// this function does not require a schema and returns the raw syntax
/// tree structure.
///
/// Args:
///     query (str): The query string to parse.
///
/// Returns:
///     tuple[dict, list]: A tuple containing:
///         - dict: A dictionary representing the parsed query AST
///         - list: A list of error dictionaries describing syntax errors
///
/// Example:
///     >>> import tantivy
///     >>> ast, errors = tantivy.parse_query_lenient("title:hello AND invalid:")
///     >>> print(f"AST: {ast}")
///     >>> print(f"Errors: {errors}")
#[pyfunction]
pub fn parse_query_lenient(
    py: Python,
    query: &str,
) -> PyResult<(Py<PyAny>, Py<PyAny>)> {
    let (ast, errors) = tv::query_grammar::parse_query_lenient(query);
    let py_ast = pythonize(py, &ast).map_err(to_pyerr)?;
    let py_errors = pythonize(py, &errors).map_err(to_pyerr)?;
    Ok((py_ast.unbind(), py_errors.unbind()))
}
