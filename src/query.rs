use crate::{get_field, make_term, to_pyerr, Schema};
use pyo3::{exceptions, prelude::*, types::PyAny, types::PyString};
use tantivy as tv;

/// Tantivy's Query
#[pyclass(frozen, module = "tantivy.tantivy")]
pub(crate) struct Query {
    pub(crate) inner: Box<dyn tv::query::Query>,
}

impl Query {
    pub(crate) fn get(&self) -> &dyn tv::query::Query {
        &self.inner
    }
}

#[pymethods]
impl Query {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("Query({:?})", self.get()))
    }

    /// Construct a Tantivy's TermQuery
    #[staticmethod]
    #[pyo3(signature = (schema, field_name, field_value, index_option = "position"))]
    pub(crate) fn term_query(
        schema: &Schema,
        field_name: &str,
        field_value: &PyAny,
        index_option: &str,
    ) -> PyResult<Query> {
        let term = make_term(&schema.inner, field_name, field_value)?;
        let index_option = match index_option {
            "position" => tv::schema::IndexRecordOption::WithFreqsAndPositions,
            "freq" => tv::schema::IndexRecordOption::WithFreqs,
            "basic" => tv::schema::IndexRecordOption::Basic,
            _ => return Err(exceptions::PyValueError::new_err(
                "Invalid index option, valid choices are: 'basic', 'freq' and 'position'"
            ))
        };
        let inner = tv::query::TermQuery::new(term, index_option);
        Ok(Query {
            inner: Box::new(inner),
        })
    }

    /// Construct a Tantivy's AllQuery
    #[staticmethod]
    pub(crate) fn all_query() -> PyResult<Query> {
        let inner = tv::query::AllQuery {};
        Ok(Query {
            inner: Box::new(inner),
        })
    }

    /// Construct a Tantivy's FuzzyTermQuery
    ///
    /// # Arguments
    ///
    /// * `schema` - Schema of the target index.
    /// * `field_name` - Field name to be searched.
    /// * `text` - String representation of the query term.
    /// * `distance` - (Optional) Edit distance you are going to alow. When not specified, the default is 1.
    /// * `transposition_cost_one` - (Optional) If true, a transposition (swapping) cost will be 1; otherwise it will be 2. When not specified, the default is true.
    /// * `prefix` - (Optional) If true, prefix levenshtein distance is applied. When not specified, the default is false.
    #[staticmethod]
    #[pyo3(signature = (schema, field_name, text, distance = 1, transposition_cost_one = true, prefix = false))]
    pub(crate) fn fuzzy_term_query(
        schema: &Schema,
        field_name: &str,
        text: &PyString,
        distance: u8,
        transposition_cost_one: bool,
        prefix: bool,
    ) -> PyResult<Query> {
        let term = make_term(&schema.inner, field_name, &text)?;
        let inner = if prefix {
            tv::query::FuzzyTermQuery::new_prefix(
                term,
                distance,
                transposition_cost_one,
            )
        } else {
            tv::query::FuzzyTermQuery::new(
                term,
                distance,
                transposition_cost_one,
            )
        };
        Ok(Query {
            inner: Box::new(inner),
        })
    }

    #[staticmethod]
    #[pyo3(signature = (schema, field_name, regex_pattern))]
    pub(crate) fn regex_query(
        schema: &Schema,
        field_name: &str,
        regex_pattern: &str,
    ) -> PyResult<Query> {
        let field = get_field(&schema.inner, field_name)?;

        let inner_result =
            tv::query::RegexQuery::from_pattern(regex_pattern, field);
        match inner_result {
            Ok(inner) => Ok(Query {
                inner: Box::new(inner),
            }),
            Err(e) => Err(to_pyerr(e)),
        }
    }
}
