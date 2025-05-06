#![allow(clippy::new_ret_no_self)]
#![allow(clippy::wrong_self_convention)]

use itertools::Itertools;
use pyo3::{
    basic::CompareOp,
    prelude::*,
    types::{
        PyAny, PyBool, PyDateAccess, PyDateTime, PyDict, PyInt, PyList,
        PyTimeAccess, PyTuple,
    },
    Python,
};

use chrono::{offset::TimeZone, NaiveDateTime, Utc};

use tantivy::{self as tv, schema::document::OwnedValue as Value};

use crate::{facet::Facet, schema::Schema, to_pyerr};
use serde::{
    ser::SerializeMap, Deserialize, Deserializer, Serialize, Serializer,
};
use std::{
    collections::BTreeMap,
    fmt,
    net::{IpAddr, Ipv6Addr},
    str::FromStr,
};

pub(crate) fn extract_value(any: &Bound<PyAny>) -> PyResult<Value> {
    if let Ok(s) = any.extract::<String>() {
        return Ok(Value::Str(s));
    }
    if any.is_exact_instance_of::<PyBool>() {
        return Ok(Value::Bool(any.extract::<bool>()?));
    }
    if let Ok(num) = any.extract::<i64>() {
        return Ok(Value::I64(num));
    }
    if let Ok(num) = any.extract::<f64>() {
        return Ok(Value::F64(num));
    }
    if let Ok(datetime) = any.extract::<NaiveDateTime>() {
        return Ok(Value::Date(tv::DateTime::from_timestamp_secs(
            datetime.and_utc().timestamp(),
        )));
    }
    if let Ok(facet) = any.extract::<Facet>() {
        return Ok(Value::Facet(facet.inner));
    }
    if let Ok(b) = any.extract::<Vec<u8>>() {
        return Ok(Value::Bytes(b));
    }
    if let Ok(dict) = any.downcast::<PyDict>() {
        if let Ok(json_dict) =
            pythonize::depythonize::<BTreeMap<String, Value>>(&dict.as_ref())
        {
            return Ok(Value::Object(json_dict.into_iter().collect()));
        } else {
            return Err(to_pyerr(
                "Invalid JSON object. Expected valid JSON string or Dict[str, Any].",
            ));
        }
    }
    Err(to_pyerr(format!("Value unsupported {any:?}")))
}

pub(crate) fn extract_value_for_type(
    any: &Bound<PyAny>,
    tv_type: tv::schema::Type,
    field_name: &str,
) -> PyResult<Value> {
    // Helper function to create `PyErr`s returned by this function.
    fn to_pyerr_for_type<'a, E: std::error::Error>(
        type_name: &'a str,
        field_name: &'a str,
        any: &'a Bound<PyAny>,
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
        tv::schema::Type::Bool => Value::Bool(
            any.extract::<bool>()
                .map_err(to_pyerr_for_type("Bool", field_name, any))?,
        ),
        tv::schema::Type::F64 => Value::F64(
            any.extract::<f64>()
                .map_err(to_pyerr_for_type("F64", field_name, any))?,
        ),
        tv::schema::Type::Date => {
            let datetime = any
                .extract::<NaiveDateTime>()
                .map_err(to_pyerr_for_type("DateTime", field_name, any))?;

            Value::Date(tv::DateTime::from_timestamp_secs(
                datetime.and_utc().timestamp(),
            ))
        }
        tv::schema::Type::Facet => Value::Facet(
            any.extract::<Facet>()
                .map_err(to_pyerr_for_type("Facet", field_name, any))?
                .inner,
        ),
        tv::schema::Type::Bytes => Value::Bytes(
            any.extract::<Vec<u8>>()
                .map_err(to_pyerr_for_type("Bytes", field_name, any))?,
        ),
        tv::schema::Type::Json => {
            if let Ok(json_str) = any.extract::<&str>() {
                return serde_json::from_str::<BTreeMap<String, Value>>(
                    json_str,
                )
                .map(|json_map| Value::Object(json_map.into_iter().collect()))
                .map_err(to_pyerr_for_type("Json", field_name, any));
            }

            let dict = any
                .downcast::<PyDict>()
                .map_err(to_pyerr_for_type("Json", field_name, any))?;
            let map = pythonize::depythonize::<BTreeMap<String, Value>>(
                &dict.as_ref(),
            )?;
            Value::Object(map.into_iter().collect())
        }
        tv::schema::Type::IpAddr => {
            let val = any
                .extract::<&str>()
                .map_err(to_pyerr_for_type("IpAddr", field_name, any))?;

            IpAddr::from_str(val)
                .map(|addr| match addr {
                    IpAddr::V4(addr) => addr.to_ipv6_mapped(),
                    IpAddr::V6(addr) => addr,
                })
                .map(Value::IpAddr)
                .map_err(to_pyerr_for_type("IpAddr", field_name, any))?
        }
    };

    Ok(value)
}

fn extract_value_single_or_list(any: &Bound<PyAny>) -> PyResult<Vec<Value>> {
    if let Ok(values) = any.downcast::<PyList>() {
        values.iter().map(|v| extract_value(&v)).collect()
    } else {
        Ok(vec![extract_value(any)?])
    }
}

fn extract_value_single_or_list_for_type(
    any: &Bound<PyAny>,
    field_type: &tv::schema::FieldType,
    field_name: &str,
) -> PyResult<Vec<Value>> {
    // Check if a numeric fast field supports multivalues.
    if let Ok(values) = any.downcast::<PyList>() {
        // Process an array of integers as a single entry if it is a bytes field.
        if field_type.value_type() == tv::schema::Type::Bytes
            && values
                .get_item(0)
                .map(|v| v.is_instance_of::<PyInt>())
                .unwrap_or(false)
        {
            return Ok(vec![extract_value_for_type(
                values,
                field_type.value_type(),
                field_name,
            )?]);
        }

        values
            .iter()
            .map(|any| {
                extract_value_for_type(
                    &any,
                    field_type.value_type(),
                    field_name,
                )
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

fn object_to_py(py: Python, obj: &Vec<(String, Value)>) -> PyResult<PyObject> {
    let dict = PyDict::new_bound(py);
    for (k, v) in obj.iter() {
        dict.set_item(k, value_to_py(py, v)?)?;
    }
    Ok(dict.into())
}

fn value_to_py(py: Python, value: &Value) -> PyResult<PyObject> {
    Ok(match value {
        Value::Null => py.None(),
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
            PyDateTime::new_bound(
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
        Value::Array(arr) => {
            let mut list = PyList::empty_bound(py);
            // Because `value_to_py` can return an error, we need to be able
            // to handle those errors on demand. Also, we want to avoid
            // collecting all the values into an intermediate `Vec` before
            // creating the `PyList`. So, the loop below is the simplest
            // solution. Another option might have been
            // `arr.iter().try_for_each(...)` but it just looks more complex.
            for v in arr {
                list.append(value_to_py(py, v)?)?;
            }
            list.into()
        }
        Value::Object(obj) => object_to_py(py, obj)?,
        Value::Bool(b) => b.into_py(py),
        Value::IpAddr(i) => (*i).to_string().into_py(py),
    })
}

fn value_to_string(value: &Value) -> String {
    match value {
        Value::Null => format!("{:?}", value),
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
        Value::Array(arr) => {
            let inner: Vec<_> = arr.iter().map(value_to_string).collect();
            format!("{inner:?}")
        }
        Value::Object(json_object) => {
            serde_json::to_string(&json_object).unwrap()
        }
        Value::Bool(b) => format!("{b}"),
        Value::IpAddr(i) => format!("{}", *i),
    }
}

/// Serializes a [`tv::DateTime`] object.
///
/// Since tantivy stores it as a single `i64` nanosecond timestamp, it is serialized and
/// deserialized as one.
fn serialize_datetime<S: Serializer>(
    dt: &tv::DateTime,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    dt.into_timestamp_nanos().serialize(serializer)
}

/// Deserializes a [`tv::DateTime`] object.
///
/// Since tantivy stores it as a single `i64` nanosecond timestamp, it is serialized and
/// deserialized as one.
fn deserialize_datetime<'de, D>(
    deserializer: D,
) -> Result<tv::DateTime, D::Error>
where
    D: Deserializer<'de>,
{
    i64::deserialize(deserializer).map(tv::DateTime::from_timestamp_nanos)
}

fn deserialize_json_object_as_i64<'de, D>(
    deserializer: D,
) -> Result<Vec<(String, Value)>, D::Error>
where
    D: Deserializer<'de>,
{
    let raw_object = Vec::deserialize(deserializer)?;
    let converted_object = raw_object
        .into_iter()
        .map(|(key, value)| {
            let converted_value = match value {
                serde_json::Value::Number(num) => {
                    if let Some(i) = num.as_i64() {
                        Value::I64(i)
                    } else {
                        Value::F64(num.as_f64().unwrap())
                    }
                }
                serde_json::Value::Object(obj) => {
                    Value::Object(deserialize_json_object_as_i64_inner(obj))
                }
                _ => Value::from(value),
            };
            (key, converted_value)
        })
        .collect();

    Ok(converted_object)
}

fn deserialize_json_object_as_i64_inner(
    raw_object: serde_json::Map<String, serde_json::Value>,
) -> Vec<(String, Value)> {
    raw_object
        .into_iter()
        .map(|(key, value)| {
            let converted_value = match value {
                serde_json::Value::Number(num) => {
                    if let Some(i) = num.as_i64() {
                        Value::I64(i)
                    } else {
                        Value::F64(num.as_f64().unwrap())
                    }
                }
                serde_json::Value::Object(obj) => {
                    Value::Object(deserialize_json_object_as_i64_inner(obj))
                }
                _ => Value::from(value),
            };
            (key, converted_value)
        })
        .collect()
}

/// An equivalent type to [`tantivy::schema::Value`], but unlike the tantivy crate's serialization
/// implementation, it uses tagging in its serialization and deserialization to differentiate
/// between different integer types.
///
/// [`BorrowedSerdeValue`] is often used for the serialization path, as owning the data is not
/// necessary for serialization.
#[derive(Deserialize, Serialize)]
enum SerdeValue {
    /// Null
    Null,
    /// The str type is used for any text information.
    Str(String),
    /// Pre-tokenized str type,
    PreTokStr(tv::tokenizer::PreTokenizedString),
    /// Unsigned 64-bits Integer `u64`
    U64(u64),
    /// Signed 64-bits Integer `i64`
    I64(i64),
    /// 64-bits Float `f64`
    F64(f64),
    /// Bool value
    Bool(bool),
    #[serde(
        deserialize_with = "deserialize_datetime",
        serialize_with = "serialize_datetime"
    )]
    /// Date/time with microseconds precision
    Date(tv::DateTime),
    /// Facet
    Facet(tv::schema::Facet),
    /// Arbitrarily sized byte array
    Bytes(Vec<u8>),
    /// Array
    Array(Vec<Value>),
    /// Object value.
    #[serde(deserialize_with = "deserialize_json_object_as_i64")]
    Object(Vec<(String, Value)>),
    /// IpV6 Address. Internally there is no IpV4, it needs to be converted to `Ipv6Addr`.
    IpAddr(Ipv6Addr),
}

impl From<SerdeValue> for Value {
    fn from(value: SerdeValue) -> Self {
        match value {
            SerdeValue::Null => Self::Null,
            SerdeValue::Str(v) => Self::Str(v),
            SerdeValue::PreTokStr(v) => Self::PreTokStr(v),
            SerdeValue::U64(v) => Self::U64(v),
            SerdeValue::I64(v) => Self::I64(v),
            SerdeValue::F64(v) => Self::F64(v),
            SerdeValue::Date(v) => Self::Date(v),
            SerdeValue::Facet(v) => Self::Facet(v),
            SerdeValue::Bytes(v) => Self::Bytes(v),
            SerdeValue::Array(v) => Self::Array(v),
            SerdeValue::Object(v) => Self::Object(v),
            SerdeValue::Bool(v) => Self::Bool(v),
            SerdeValue::IpAddr(v) => Self::IpAddr(v),
        }
    }
}

impl From<Value> for SerdeValue {
    fn from(value: Value) -> Self {
        match value {
            Value::Null => Self::Null,
            Value::Str(v) => Self::Str(v),
            Value::PreTokStr(v) => Self::PreTokStr(v),
            Value::U64(v) => Self::U64(v),
            Value::I64(v) => Self::I64(v),
            Value::F64(v) => Self::F64(v),
            Value::Date(v) => Self::Date(v),
            Value::Facet(v) => Self::Facet(v),
            Value::Bytes(v) => Self::Bytes(v),
            Value::Array(v) => Self::Array(v),
            Value::Object(v) => Self::Object(v),
            Value::Bool(v) => Self::Bool(v),
            Value::IpAddr(v) => Self::IpAddr(v),
        }
    }
}

/// A non-owning version of [`SerdeValue`]. This is used in serialization to avoid unnecessary
/// cloning.
#[derive(Serialize)]
enum BorrowedSerdeValue<'a> {
    /// Null
    Null,
    /// The str type is used for any text information.
    Str(&'a str),
    /// Pre-tokenized str type,
    PreTokStr(&'a tv::tokenizer::PreTokenizedString),
    /// Unsigned 64-bits Integer `u64`
    U64(&'a u64),
    /// Signed 64-bits Integer `i64`
    I64(&'a i64),
    /// 64-bits Float `f64`
    F64(&'a f64),
    /// Bool value
    Bool(&'a bool),
    #[serde(serialize_with = "serialize_datetime")]
    /// Date/time with microseconds precision
    Date(&'a tv::DateTime),
    /// Facet
    Facet(&'a tv::schema::Facet),
    /// Arbitrarily sized byte array
    Bytes(&'a [u8]),
    /// Array
    Array(&'a Vec<Value>),
    /// Json object value.
    Object(&'a Vec<(String, Value)>),
    /// IpV6 Address. Internally there is no IpV4, it needs to be converted to `Ipv6Addr`.
    IpAddr(&'a Ipv6Addr),
}

impl<'a> From<&'a Value> for BorrowedSerdeValue<'a> {
    fn from(value: &'a Value) -> Self {
        match value {
            Value::Null => Self::Null,
            Value::Str(v) => Self::Str(v),
            Value::PreTokStr(v) => Self::PreTokStr(v),
            Value::U64(v) => Self::U64(v),
            Value::I64(v) => Self::I64(v),
            Value::F64(v) => Self::F64(v),
            Value::Date(v) => Self::Date(v),
            Value::Facet(v) => Self::Facet(v),
            Value::Bytes(v) => Self::Bytes(v),
            Value::Array(v) => Self::Array(v),
            Value::Object(v) => Self::Object(v),
            Value::Bool(v) => Self::Bool(v),
            Value::IpAddr(v) => Self::IpAddr(v),
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
///     ...                       "skiff in the Gulf Stream and he had gone "
///     ...                       "eighty-four days now without taking a fish."))
///     >>> doc
///     Document(body=[He was an ],title=[The Old Ma])
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
///     ...     SchemaBuilder()
///     ...         .add_unsigned_field("unsigned")
///     ...         .add_integer_field("signed")
///     ...         .add_float_field("float")
///     ...         .build()
///     ... )
///     >>> doc = tantivy.Document.from_dict(
///     ...     {"unsigned": 1000, "signed": -5, "float": 0.4},
///     ...     schema,
///     ... )
#[pyclass(module = "tantivy.tantivy")]
#[derive(Clone, Default, PartialEq)]
pub(crate) struct Document {
    pub(crate) field_values: BTreeMap<String, Vec<Value>>,
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
                format!("{field_name}=[{values_str}]")
            })
            .join(",");
        write!(f, "Document({doc_str})")
    }
}

impl Serialize for Document {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map =
            serializer.serialize_map(Some(self.field_values.len()))?;
        for (k, v) in &self.field_values {
            let ser_v: Vec<_> =
                v.iter().map(BorrowedSerdeValue::from).collect();
            map.serialize_entry(&k, &ser_v)?;
        }
        map.end()
    }
}

impl<'de> Deserialize<'de> for Document {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        BTreeMap::<String, Vec<SerdeValue>>::deserialize(deserializer).map(
            |field_map| Document {
                field_values: field_map
                    .into_iter()
                    .map(|(k, v)| {
                        let v: Vec<_> =
                            v.into_iter().map(Value::from).collect();
                        (k, v)
                    })
                    .collect(),
            },
        )
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
    fn new(kwargs: Option<&Bound<PyDict>>) -> PyResult<Self> {
        let mut document = Document::default();
        if let Some(field_dict) = kwargs {
            document.extend(field_dict, None)?;
        }
        Ok(document)
    }

    fn extend(
        &mut self,
        py_dict: &Bound<PyDict>,
        schema: Option<&Schema>,
    ) -> PyResult<()> {
        Document::extract_py_values_from_dict(
            py_dict,
            schema,
            &mut self.field_values,
        )
    }

    #[staticmethod]
    #[pyo3(signature = (py_dict, schema=None))]
    fn from_dict(
        py_dict: &Bound<PyDict>,
        schema: Option<&Schema>,
    ) -> PyResult<Document> {
        let mut field_values: BTreeMap<String, Vec<Value>> = BTreeMap::new();
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
        let dict = PyDict::new_bound(py);
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
        self.add_value(field_name, text);
    }

    /// Add an unsigned integer value to the document.
    ///
    /// Args:
    ///     field_name (str): The field name for which we are adding the unsigned integer.
    ///     value (int): The integer that will be added to the document.
    fn add_unsigned(&mut self, field_name: String, value: u64) {
        self.add_value(field_name, value);
    }

    /// Add a signed integer value to the document.
    ///
    /// Args:
    ///     field_name (str): The field name for which we are adding the integer.
    ///     value (int): The integer that will be added to the document.
    fn add_integer(&mut self, field_name: String, value: i64) {
        self.add_value(field_name, value);
    }

    /// Add a float value to the document.
    ///
    /// Args:
    ///     field_name (str): The field name for which we are adding the value.
    ///     value (f64): The float that will be added to the document.
    fn add_float(&mut self, field_name: String, value: f64) {
        self.add_value(field_name, value);
    }

    /// Add a boolean value to the document.
    ///
    /// Args:
    ///     field_name (str): The field name for which we are adding the value.
    ///     value (bool): The boolean that will be added to the document.
    fn add_boolean(&mut self, field_name: String, value: bool) {
        self.add_value(field_name, value);
    }

    /// Add a date value to the document.
    ///
    /// Args:
    ///     field_name (str): The field name for which we are adding the date.
    ///     value (datetime): The date that will be added to the document.
    fn add_date(&mut self, field_name: String, value: &Bound<PyDateTime>) {
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
        self.add_value(
            field_name,
            tv::DateTime::from_timestamp_secs(datetime.timestamp()),
        );
    }

    /// Add a facet value to the document.
    /// Args:
    ///     field_name (str): The field name for which we are adding the facet.
    ///     value (Facet): The Facet that will be added to the document.
    fn add_facet(&mut self, field_name: String, facet: &Facet) {
        self.add_value(field_name, facet.inner.clone());
    }

    /// Add a bytes value to the document.
    ///
    /// Args:
    ///     field_name (str): The field for which we are adding the bytes.
    ///     value (bytes): The bytes that will be added to the document.
    fn add_bytes(&mut self, field_name: String, bytes: Vec<u8>) {
        self.add_value(field_name, bytes);
    }

    /// Add a JSON value to the document.
    ///
    /// Args:
    ///     field_name (str): The field for which we are adding the JSON.
    ///     value (str | Dict[str, Any]): The JSON object that will be added
    ///         to the document.
    ///
    /// Raises a ValueError if the JSON is invalid.
    fn add_json(
        &mut self,
        field_name: String,
        value: &Bound<PyAny>,
    ) -> PyResult<()> {
        type JsonMap = serde_json::Map<String, serde_json::Value>;

        if let Ok(json_str) = value.extract::<&str>() {
            let json_map: JsonMap =
                serde_json::from_str(json_str).map_err(to_pyerr)?;
            self.add_value(field_name, json_map);
            Ok(())
        } else if let Ok(json_map) = pythonize::depythonize::<JsonMap>(value) {
            self.add_value(field_name, json_map);
            Ok(())
        } else {
            Err(to_pyerr("Invalid JSON object. Expected valid JSON string or Dict[str, Any]."))
        }
    }

    /// Add an IP address value to the document.
    ///
    /// Args:
    ///     field_name (str): The field for which we are adding the IP address.
    ///     value (str): The IP address object that will be added
    ///         to the document.
    ///
    /// Raises a ValueError if the IP address is invalid.
    fn add_ip_addr(&mut self, field_name: String, value: &str) -> PyResult<()> {
        let ip_addr = IpAddr::from_str(value).map_err(to_pyerr)?;
        match ip_addr {
            IpAddr::V4(addr) => {
                self.add_value(field_name, addr.to_ipv6_mapped())
            }
            IpAddr::V6(addr) => self.add_value(field_name, addr),
        }
        Ok(())
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

    fn __copy__(&self) -> Self {
        self.clone()
    }

    fn __deepcopy__(&self, _memo: &Bound<PyDict>) -> Self {
        self.clone()
    }

    fn __richcmp__(
        &self,
        other: &Self,
        op: CompareOp,
        py: Python<'_>,
    ) -> PyObject {
        match op {
            CompareOp::Eq => (self == other).into_py(py),
            CompareOp::Ne => (self != other).into_py(py),
            _ => py.NotImplemented(),
        }
    }

    #[staticmethod]
    fn _internal_from_pythonized(serialized: &Bound<PyAny>) -> PyResult<Self> {
        pythonize::depythonize(&serialized).map_err(to_pyerr)
    }

    fn __reduce__<'a>(
        slf: PyRef<'a, Self>,
        py: Python<'a>,
    ) -> PyResult<Bound<'a, PyTuple>> {
        let serialized = pythonize::pythonize(py, &*slf).map_err(to_pyerr)?;

        Ok(PyTuple::new_bound(
            py,
            [
                slf.into_py(py).getattr(py, "_internal_from_pythonized")?,
                PyTuple::new_bound(py, [serialized]).to_object(py),
            ],
        ))
    }
}

impl Document {
    fn add_value<T>(&mut self, field_name: String, value: T)
    where
        Value: From<T>,
    {
        self.field_values
            .entry(field_name)
            .or_default()
            .push(Value::from(value));
    }

    fn extract_py_values_from_dict(
        py_dict: &Bound<PyDict>,
        schema: Option<&Schema>,
        out_field_values: &mut BTreeMap<String, Vec<Value>>,
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
                        &key_value.get_item(1)?,
                        field_type,
                        key.as_str(),
                    )?
                } else {
                    extract_value_single_or_list(&key_value.get_item(1)?)?
                };

                out_field_values.insert(key, value_list);
            }
        }

        Ok(())
    }

    pub fn iter_values_for_field<'a>(
        &'a self,
        field: &str,
    ) -> impl Iterator<Item = &'a Value> + 'a {
        self.field_values
            .get(field)
            .into_iter()
            .flat_map(|values| values.iter())
    }
}
