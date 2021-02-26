#![allow(clippy::new_ret_no_self)]
#![allow(clippy::wrong_self_convention)]

use itertools::Itertools;
use pyo3::{
    prelude::*,
    types::{
        PyAny, PyDateAccess, PyDateTime, PyDict, PyList, PyTimeAccess, PyTuple,
    },
};

use chrono::{offset::TimeZone, Datelike, Timelike, Utc};

use tantivy as tv;

use crate::{facet::Facet, to_pyerr};
use pyo3::{PyMappingProtocol, PyObjectProtocol};
use std::{collections::BTreeMap, fmt};
use tantivy::schema::Value;

fn value_to_py(py: Python, value: &Value) -> PyResult<PyObject> {
    Ok(match value {
        Value::Str(text) => text.into_py(py),
        Value::U64(num) => (*num).into_py(py),
        Value::I64(num) => (*num).into_py(py),
        Value::F64(num) => (*num).into_py(py),
        Value::Bytes(b) => b.to_object(py),
        Value::PreTokStr(_pretoken) => {
            // TODO implement me
            unimplemented!();
        }
        Value::Date(d) => PyDateTime::new(
            py,
            d.year(),
            d.month() as u8,
            d.day() as u8,
            d.hour() as u8,
            d.minute() as u8,
            d.second() as u8,
            d.timestamp_subsec_micros(),
            None,
        )?
        .into_py(py),
        Value::Facet(f) => Facet { inner: f.clone() }.into_py(py),
    })
}

fn value_to_string(value: &Value) -> String {
    match value {
        Value::Str(text) => text.clone(),
        Value::U64(num) => format!("{}", num),
        Value::I64(num) => format!("{}", num),
        Value::F64(num) => format!("{}", num),
        Value::Bytes(bytes) => format!("{:?}", bytes),
        Value::Date(d) => format!("{:?}", d),
        Value::Facet(facet) => facet.to_string(),
        Value::PreTokStr(_pretok) => {
            // TODO implement me
            unimplemented!();
        }
    }
}

/// Tantivy's Document is the object that can be indexed and then searched for.
///
/// Documents are fundamentally a collection of unordered tuples
/// (field_name, value). In this list, one field may appear more than once.
///
/// Example:
///     >>> doc = tantivy.Document()
///     >>> doc.add_text("title", "The Old Man and the Sea")
///     >>> doc.add_text("body", ("He was an old man who fished alone in a "
///                             "skiff in the Gulf Stream and he had gone "
///                             "eighty-four days now without taking a fish."))
///
/// For simplicity, it is also possible to build a `Document` by passing the field
/// values directly as constructor arguments.
///
/// Example:
///     >>> doc = tantivy.Document(title=["The Old Man and the Sea"], body=["..."])
///
/// As syntactic sugar, tantivy also allows the user to pass a single values
/// if there is only one. In other words, the following is also legal.
///
/// Example:
///     >>> doc = tantivy.Document(title="The Old Man and the Sea", body="...")

#[pyclass]
#[derive(Default)]
pub(crate) struct Document {
    pub(crate) field_values: BTreeMap<String, Vec<tv::schema::Value>>,
}

impl fmt::Debug for Document {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let doc_str = self
            .field_values
            .iter()
            .map(|(field_name, field_values)| {
                let values_str: String = field_values
                    .iter()
                    .map(value_to_string)
                    .join(",")
                    .chars()
                    .take(10)
                    .collect();
                format!("{}=[{}]", field_name, values_str)
            })
            .join(",");
        write!(f, "Document({})", doc_str)
    }
}

fn add_value<T>(doc: &mut Document, field_name: String, value: T)
where
    Value: From<T>,
{
    doc.field_values
        .entry(field_name)
        .or_insert_with(Vec::new)
        .push(Value::from(value));
}

pub(crate) fn extract_value(any: &PyAny) -> PyResult<Value> {
    if let Ok(s) = any.extract::<String>() {
        return Ok(Value::Str(s));
    }
    if let Ok(num) = any.extract::<i64>() {
        return Ok(Value::I64(num));
    }
    if let Ok(num) = any.extract::<f64>() {
        return Ok(Value::F64(num));
    }
    if let Ok(py_datetime) = any.downcast::<PyDateTime>() {
        let datetime = Utc
            .ymd(
                py_datetime.get_year(),
                py_datetime.get_month().into(),
                py_datetime.get_day().into(),
            )
            .and_hms_micro(
                py_datetime.get_hour().into(),
                py_datetime.get_minute().into(),
                py_datetime.get_second().into(),
                py_datetime.get_microsecond(),
            );
        return Ok(Value::Date(datetime));
    }
    if let Ok(facet) = any.extract::<Facet>() {
        return Ok(Value::Facet(facet.inner.clone()));
    }
    Err(to_pyerr(format!("Value unsupported {:?}", any)))
}

fn extract_value_single_or_list(any: &PyAny) -> PyResult<Vec<Value>> {
    if let Ok(values) = any.downcast::<PyList>() {
        values.iter().map(extract_value).collect()
    } else {
        Ok(vec![extract_value(any)?])
    }
}

#[pymethods]
impl Document {
    #[new]
    #[args(kwargs = "**")]
    fn new(kwargs: Option<&PyDict>) -> PyResult<Self> {
        let mut document = Document::default();
        if let Some(field_dict) = kwargs {
            document.extend(field_dict)?;
        }
        Ok(document)
    }

    fn extend(&mut self, py_dict: &PyDict) -> PyResult<()> {
        let mut field_values: BTreeMap<String, Vec<tv::schema::Value>> =
            BTreeMap::new();
        for key_value_any in py_dict.items() {
            if let Ok(key_value) = key_value_any.downcast::<PyTuple>() {
                if key_value.len() != 2 {
                    continue;
                }
                let key: String = key_value.get_item(0).extract()?;
                let value_list =
                    extract_value_single_or_list(key_value.get_item(1))?;
                field_values.insert(key, value_list);
            }
        }
        self.field_values.extend(field_values.into_iter());
        Ok(())
    }

    #[staticmethod]
    fn from_dict(py_dict: &PyDict) -> PyResult<Document> {
        let mut field_values: BTreeMap<String, Vec<tv::schema::Value>> =
            BTreeMap::new();
        for key_value_any in py_dict.items() {
            if let Ok(key_value) = key_value_any.downcast::<PyTuple>() {
                if key_value.len() != 2 {
                    continue;
                }
                let key: String = key_value.get_item(0).extract()?;
                let value_list =
                    extract_value_single_or_list(key_value.get_item(1))?;
                field_values.insert(key, value_list);
            }
        }
        Ok(Document { field_values })
    }

    /// Returns a dictionary with the different
    /// field values.
    ///
    /// In tantivy, `Document` can be hold multiple
    /// values for a single field.
    ///
    /// For this reason, the dictionary, will associate
    /// a list of value for every field.
    fn to_dict(&self, py: Python) -> PyResult<PyObject> {
        let dict = PyDict::new(py);
        for (key, values) in &self.field_values {
            let values_py: Vec<PyObject> = values
                .iter()
                .map(|v| value_to_py(py, v))
                .collect::<PyResult<_>>()?;
            dict.set_item(key, values_py)?;
        }
        Ok(dict.into())
    }

    /// Add a text value to the document.
    ///
    /// Args:
    ///     field_name (str): The field name for which we are adding the text.
    ///     text (str): The text that will be added to the document.
    fn add_text(&mut self, field_name: String, text: &str) {
        add_value(self, field_name, text);
    }

    /// Add an unsigned integer value to the document.
    ///
    /// Args:
    ///     field_name (str): The field name for which we are adding the unsigned integer.
    ///     value (int): The integer that will be added to the document.
    fn add_unsigned(&mut self, field_name: String, value: u64) {
        add_value(self, field_name, value);
    }

    /// Add a signed integer value to the document.
    ///
    /// Args:
    ///     field_name (str): The field name for which we are adding the integer.
    ///     value (int): The integer that will be added to the document.
    fn add_integer(&mut self, field_name: String, value: i64) {
        add_value(self, field_name, value);
    }

    /// Add a date value to the document.
    ///
    /// Args:
    ///     field_name (str): The field name for which we are adding the date.
    ///     value (datetime): The date that will be added to the document.
    fn add_date(&mut self, field_name: String, value: &PyDateTime) {
        let datetime = Utc
            .ymd(
                value.get_year(),
                value.get_month().into(),
                value.get_day().into(),
            )
            .and_hms_micro(
                value.get_hour().into(),
                value.get_minute().into(),
                value.get_second().into(),
                value.get_microsecond(),
            );
        add_value(self, field_name, datetime);
    }

    /// Add a facet value to the document.
    /// Args:
    ///     field_name (str): The field name for which we are adding the facet.
    ///     value (Facet): The Facet that will be added to the document.
    fn add_facet(&mut self, field_name: String, facet: &Facet) {
        add_value(self, field_name, facet.inner.clone());
    }

    /// Add a bytes value to the document.
    ///
    /// Args:
    ///     field_name (str): The field for which we are adding the bytes.
    ///     value (bytes): The bytes that will be added to the document.
    fn add_bytes(&mut self, field_name: String, bytes: Vec<u8>) {
        add_value(self, field_name, bytes);
    }

    /// Returns the number of added fields that have been added to the document
    #[getter]
    fn num_fields(&self) -> usize {
        self.field_values.len()
    }

    /// True if the document is empty, False otherwise.
    #[getter]
    fn is_empty(&self) -> bool {
        self.field_values.is_empty()
    }

    /// Get the first value associated with the given field.
    ///
    /// Args:
    ///     field (Field): The field for which we would like to get the value.
    ///
    /// Returns the value if one is found, otherwise None.
    /// The type of the value depends on the field.
    fn get_first(
        &self,
        py: Python,
        fieldname: &str,
    ) -> PyResult<Option<PyObject>> {
        if let Some(value) = self.iter_values_for_field(fieldname).next() {
            let py_value = value_to_py(py, value)?;
            Ok(Some(py_value))
        } else {
            Ok(None)
        }
    }

    /// Get the all values associated with the given field.
    ///
    /// Args:
    ///     field (Field): The field for which we would like to get the values.
    ///
    /// Returns a list of values.
    /// The type of the value depends on the field.
    fn get_all(&self, py: Python, field_name: &str) -> PyResult<Vec<PyObject>> {
        self.iter_values_for_field(field_name)
            .map(|value| value_to_py(py, value))
            .collect::<PyResult<Vec<_>>>()
    }
}

impl Document {
    fn iter_values_for_field<'a>(
        &'a self,
        field: &str,
    ) -> impl Iterator<Item = &'a Value> + 'a {
        self.field_values
            .get(field)
            .into_iter()
            .flat_map(|values| values.iter())
    }
}

#[pyproto]
impl PyMappingProtocol for Document {
    fn __getitem__(&self, field_name: &str) -> PyResult<Vec<PyObject>> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        self.get_all(py, field_name)
    }
}

#[pyproto]
impl PyObjectProtocol for Document {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{:?}", self))
    }
}
