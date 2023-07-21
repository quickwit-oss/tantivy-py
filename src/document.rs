#![allow(clippy::new_ret_no_self)]
#![allow(clippy::wrong_self_convention)]

use itertools::Itertools;
use pyo3::{
    prelude::*,
    types::{
        PyAny, PyDateAccess, PyDateTime, PyDict, PyList, PyTimeAccess, PyTuple,
    },
};

use chrono::{offset::TimeZone, NaiveDateTime, Utc};

use tantivy as tv;

use crate::{
    facet::Facet, impl_py_copy, impl_py_deepcopy, schema::Schema, to_pyerr,
};
use serde_json::Value as JsonValue;
use std::{
    collections::{BTreeMap, HashMap},
    fmt,
};
use tantivy::schema::Value;

fn value_to_object(val: &JsonValue, py: Python<'_>) -> PyObject {
    match val {
        JsonValue::Null => py.None(),
        JsonValue::Bool(b) => b.to_object(py),
        JsonValue::Number(n) => match n {
            n if n.is_i64() => n.as_i64().to_object(py),
            n if n.is_u64() => n.as_u64().to_object(py),
            n if n.is_f64() => n.as_f64().to_object(py),
            _ => panic!("number too large"),
        },
        JsonValue::String(s) => s.to_object(py),
        JsonValue::Array(v) => {
            let inner: Vec<_> =
                v.iter().map(|x| value_to_object(x, py)).collect();
            inner.to_object(py)
        }
        JsonValue::Object(m) => {
            let inner: HashMap<_, _> =
                m.iter().map(|(k, v)| (k, value_to_object(v, py))).collect();
            inner.to_object(py)
        }
    }
}

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
        Value::Date(d) => {
            let utc = d.into_utc();
            PyDateTime::new(
                py,
                utc.year(),
                utc.month() as u8,
                utc.day(),
                utc.hour(),
                utc.minute(),
                utc.second(),
                utc.microsecond(),
                None,
            )?
            .into_py(py)
        }
        Value::Facet(f) => Facet { inner: f.clone() }.into_py(py),
        Value::JsonObject(json_object) => {
            let inner: HashMap<_, _> = json_object
                .iter()
                .map(|(k, v)| (k, value_to_object(v, py)))
                .collect();
            inner.to_object(py)
        }
        Value::Bool(b) => b.into_py(py),
        Value::IpAddr(i) => (*i).to_string().into_py(py),
    })
}

fn value_to_string(value: &Value) -> String {
    match value {
        Value::Str(text) => text.clone(),
        Value::U64(num) => format!("{num}"),
        Value::I64(num) => format!("{num}"),
        Value::F64(num) => format!("{num}"),
        Value::Bytes(bytes) => format!("{bytes:?}"),
        Value::Date(d) => format!("{d:?}"),
        Value::Facet(facet) => facet.to_string(),
        Value::PreTokStr(_pretok) => {
            // TODO implement me
            unimplemented!();
        }
        Value::JsonObject(json_object) => {
            serde_json::to_string(&json_object).unwrap()
        }
        Value::Bool(b) => format!("{b}"),
        Value::IpAddr(i) => format!("{}", *i),
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
///
/// For numeric fields, the [`Document`] constructor does not have any
/// information about the type and will try to guess the type.
/// Therefore, it is recommended to use the [`Document::from_dict()`],
/// [`Document::extract()`], or `Document::add_*()` functions to provide
/// explicit type information.
///
/// Example:
///     >>> schema = (
///             SchemaBuilder()
///                 .add_unsigned_field("unsigned")
///                 .add_integer_field("signed")
///                 .add_float_field("float")
///                 .build()
///         )
///     >>> doc = tantivy.Document.from_dict(
///             {"unsigned": 1000, "signed": -5, "float": 0.4},
///             schema,
///         )
#[pyclass(module = "tantivy")]
#[derive(Clone, Default)]
pub(crate) struct Document {
    pub(crate) field_values: BTreeMap<String, Vec<tv::schema::Value>>,
}

impl_py_copy!(Document);
impl_py_deepcopy!(Document);

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
                format!("{field_name}=[{values_str}]")
            })
            .join(",");
        write!(f, "Document({doc_str})")
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
    if let Ok(datetime) = any.extract::<NaiveDateTime>() {
        return Ok(Value::Date(tv::DateTime::from_timestamp_secs(
            datetime.timestamp(),
        )));
    }
    if let Ok(facet) = any.extract::<Facet>() {
        return Ok(Value::Facet(facet.inner));
    }
    if let Ok(b) = any.extract::<Vec<u8>>() {
        return Ok(Value::Bytes(b));
    }
    Err(to_pyerr(format!("Value unsupported {any:?}")))
}

pub(crate) fn extract_value_for_type(
    any: &PyAny,
    tv_type: tv::schema::Type,
    field_name: &str,
) -> PyResult<Value> {
    // Helper function to create `PyErr`s returned by this function.
    fn to_pyerr_for_type<'a, E: std::error::Error>(
        type_name: &'a str,
        field_name: &'a str,
        any: &'a PyAny,
    ) -> impl Fn(E) -> PyErr + 'a {
        move |_| {
            to_pyerr(format!(
                "Expected {} type for field {}, got {:?}",
                type_name, field_name, any
            ))
        }
    }

    let value = match tv_type {
        tv::schema::Type::Str => Value::Str(
            any.extract::<String>()
                .map_err(to_pyerr_for_type("Str", field_name, any))?,
        ),
        tv::schema::Type::U64 => Value::U64(
            any.extract::<u64>()
                .map_err(to_pyerr_for_type("U64", field_name, any))?,
        ),
        tv::schema::Type::I64 => Value::I64(
            any.extract::<i64>()
                .map_err(to_pyerr_for_type("I64", field_name, any))?,
        ),
        tv::schema::Type::F64 => Value::F64(
            any.extract::<f64>()
                .map_err(to_pyerr_for_type("F64", field_name, any))?,
        ),
        tv::schema::Type::Date => {
            let datetime = any
                .extract::<NaiveDateTime>()
                .map_err(to_pyerr_for_type("DateTime", field_name, any))?;

            Value::Date(tv::DateTime::from_timestamp_secs(datetime.timestamp()))
        }
        tv::schema::Type::Facet => Value::Facet(
            any.extract::<Facet>()
                .map_err(to_pyerr_for_type("Facet", field_name, any))?
                .inner,
        ),
        _ => return Err(to_pyerr(format!("Value unsupported {:?}", any))),
    };

    Ok(value)
}

fn extract_value_single_or_list(any: &PyAny) -> PyResult<Vec<Value>> {
    if let Ok(values) = any.downcast::<PyList>() {
        values.iter().map(extract_value).collect()
    } else {
        Ok(vec![extract_value(any)?])
    }
}

fn extract_value_single_or_list_for_type(
    any: &PyAny,
    field_type: &tv::schema::FieldType,
    field_name: &str,
) -> PyResult<Vec<Value>> {
    // Check if a numeric fast field supports multivalues.
    if let Ok(values) = any.downcast::<PyList>() {
        values
            .iter()
            .map(|any| {
                extract_value_for_type(any, field_type.value_type(), field_name)
            })
            .collect()
    } else {
        Ok(vec![extract_value_for_type(
            any,
            field_type.value_type(),
            field_name,
        )?])
    }
}

impl Document {
    fn extract_py_values_from_dict(
        py_dict: &PyDict,
        schema: Option<&Schema>,
        out_field_values: &mut BTreeMap<String, Vec<tv::schema::Value>>,
    ) -> PyResult<()> {
        // TODO: Reserve when https://github.com/rust-lang/rust/issues/72631 is stable.
        // out_field_values.reserve(py_dict.len());

        for key_value_any in py_dict.items() {
            if let Ok(key_value) = key_value_any.downcast::<PyTuple>() {
                if key_value.len() != 2 {
                    continue;
                }
                let key = key_value.get_item(0)?.extract::<String>()?;

                let field_type = if let Some(schema) = schema {
                    let field_type = schema
                        .inner
                        .get_field(key.as_str())
                        .map(|field| {
                            schema.inner.get_field_entry(field).field_type()
                        })
                        .ok();

                    if let Some(field_type) = field_type {
                        // A field type was found, so validate it after the values are extracted.
                        Some(field_type)
                    } else {
                        // The field does not exist in the schema, so skip over it.
                        continue;
                    }
                } else {
                    // No schema was provided, so do not validate anything.
                    None
                };

                let value_list = if let Some(field_type) = field_type {
                    extract_value_single_or_list_for_type(
                        key_value.get_item(1)?,
                        field_type,
                        key.as_str(),
                    )?
                } else {
                    extract_value_single_or_list(key_value.get_item(1)?)?
                };

                out_field_values.insert(key, value_list);
            }
        }

        Ok(())
    }
}

#[pymethods]
impl Document {
    /// Creates a new document with optional fields from `**kwargs`.
    ///
    /// Note that the types of numeric fields are unknown here. To
    /// provide explicit type information, use the [`from_dict()`],
    /// [`extend()`], or `add_<type>()` functions.
    #[new]
    #[pyo3(signature = (**kwargs))]
    fn new(kwargs: Option<&PyDict>) -> PyResult<Self> {
        let mut document = Document::default();
        if let Some(field_dict) = kwargs {
            document.extend(field_dict, None)?;
        }
        Ok(document)
    }

    fn extend(
        &mut self,
        py_dict: &PyDict,
        schema: Option<&Schema>,
    ) -> PyResult<()> {
        Document::extract_py_values_from_dict(
            py_dict,
            schema,
            &mut self.field_values,
        )
    }

    #[staticmethod]
    fn from_dict(
        py_dict: &PyDict,
        schema: Option<&Schema>,
    ) -> PyResult<Document> {
        let mut field_values: BTreeMap<String, Vec<tv::schema::Value>> =
            BTreeMap::new();
        Document::extract_py_values_from_dict(
            py_dict,
            schema,
            &mut field_values,
        )?;
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

    /// Add a float value to the document.
    ///
    /// Args:
    ///     field_name (str): The field name for which we are adding the value.
    ///     value (f64): The float that will be added to the document.
    fn add_float(&mut self, field_name: String, value: f64) {
        add_value(self, field_name, value);
    }

    /// Add a date value to the document.
    ///
    /// Args:
    ///     field_name (str): The field name for which we are adding the date.
    ///     value (datetime): The date that will be added to the document.
    fn add_date(&mut self, field_name: String, value: &PyDateTime) {
        let datetime = Utc
            .with_ymd_and_hms(
                value.get_year(),
                value.get_month().into(),
                value.get_day().into(),
                value.get_hour().into(),
                value.get_minute().into(),
                value.get_second().into(),
            )
            .single()
            .unwrap();
        add_value(
            self,
            field_name,
            tv::DateTime::from_timestamp_secs(datetime.timestamp()),
        );
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

    /// Add a bytes value to the document.
    ///
    /// Args:
    ///     field_name (str): The field for which we are adding the bytes.
    ///     value (str): The json object that will be added to the document.
    fn add_json(&mut self, field_name: String, json: &str) {
        let json_object: serde_json::Value =
            serde_json::from_str(json).unwrap();
        add_value(self, field_name, json_object);
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

    fn __getitem__(&self, field_name: &str) -> PyResult<Vec<PyObject>> {
        Python::with_gil(|py| -> PyResult<Vec<PyObject>> {
            self.get_all(py, field_name)
        })
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{self:?}"))
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
