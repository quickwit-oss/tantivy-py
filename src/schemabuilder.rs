#![allow(clippy::new_ret_no_self)]

use pyo3::{exceptions, prelude::*};

use crate::schema::Schema;
use std::sync::{Arc, RwLock};
use tantivy::schema::{
    self, BytesOptions, DateOptions, IpAddrOptions, INDEXED,
};

/// Tantivy has a very strict schema.
/// You need to specify in advance whether a field is indexed or not,
/// stored or not.
///
/// This is done by creating a schema object, and
/// setting up the fields one by one.
///
/// Examples:
///
///     >>> builder = tantivy.SchemaBuilder()
///
///     >>> title = builder.add_text_field("title", stored=True)
///     >>> body = builder.add_text_field("body")
///
///     >>> schema = builder.build()
#[pyclass(module = "tantivy.tantivy")]
#[derive(Clone)]
pub(crate) struct SchemaBuilder {
    pub(crate) builder: Arc<RwLock<Option<schema::SchemaBuilder>>>,
}

const NO_TOKENIZER_NAME: &str = "raw";
const TOKENIZER: &str = "default";
const RECORD: &str = "position";

#[pymethods]
impl SchemaBuilder {
    #[new]
    fn new() -> Self {
        SchemaBuilder {
            builder: Arc::new(From::from(Some(schema::Schema::builder()))),
        }
    }

    #[staticmethod]
    fn is_valid_field_name(name: &str) -> bool {
        schema::is_valid_field_name(name)
    }

    /// Add a new text field to the schema.
    ///
    /// Args:
    ///     name (str): The name of the field.
    ///     stored (bool, optional): If true sets the field as stored, the
    ///         content of the field can be later restored from a Searcher.
    ///         Defaults to False.
    ///     fast (bool, optional): Set the text options as a fast field. A
    ///         fast field is a column-oriented fashion storage for tantivy.
    ///         Text fast fields will have the term ids stored in the fast
    ///         field. The fast field will be a multivalued fast field.
    ///         It is recommended to use the "raw" tokenizer, since it will
    ///         store the original text unchanged. The "default" tokenizer will
    ///         store the terms as lower case and this will be reflected in the
    ///         dictionary.
    ///     tokenizer_name (str, optional): The name of the tokenizer that
    ///         should be used to process the field. Defaults to 'default'
    ///     index_option (str, optional): Sets which information should be
    ///         indexed with the tokens. Can be one of 'position', 'freq' or
    ///         'basic'. Defaults to 'position'. The 'basic' index_option
    ///         records only the document ID, the 'freq' option records the
    ///         document id and the term frequency, while the 'position' option
    ///         records the document id, term frequency and the positions of
    ///         the term occurrences in the document.
    ///
    /// Returns the associated field handle.
    /// Raises a ValueError if there was an error with the field creation.
    #[pyo3(signature = (
        name,
        stored = false,
        fast = false,
        tokenizer_name = TOKENIZER,
        index_option = RECORD
    ))]
    fn add_text_field(
        &mut self,
        name: &str,
        stored: bool,
        fast: bool,
        tokenizer_name: &str,
        index_option: &str,
    ) -> PyResult<Self> {
        let builder = &mut self.builder;
        let options = SchemaBuilder::build_text_option(
            stored,
            fast,
            tokenizer_name,
            index_option,
        )?;

        if let Some(builder) = builder.write().unwrap().as_mut() {
            builder.add_text_field(name, options);
        } else {
            return Err(exceptions::PyValueError::new_err(
                "Schema builder object isn't valid anymore.",
            ));
        }
        Ok(self.clone())
    }

    /// Add a new signed integer field to the schema.
    ///
    /// Args:
    ///     name (str): The name of the field.
    ///     stored (bool, optional): If true sets the field as stored, the
    ///         content of the field can be later restored from a Searcher.
    ///         Defaults to False.
    ///     indexed (bool, optional): If true sets the field to be indexed.
    ///     fast (bool, optional): Set the numeric options as a fast field. A
    ///         fast field is a column-oriented fashion storage for tantivy.
    ///         It is designed for the fast random access of some document
    ///         fields given a document id.
    ///
    /// Returns the associated field handle.
    /// Raises a ValueError if there was an error with the field creation.
    #[pyo3(signature = (name, stored = false, indexed = false, fast = false))]
    fn add_integer_field(
        &mut self,
        name: &str,
        stored: bool,
        indexed: bool,
        fast: bool,
    ) -> PyResult<Self> {
        let builder = &mut self.builder;

        let opts = SchemaBuilder::build_numeric_option(stored, indexed, fast)?;

        if let Some(builder) = builder.write().unwrap().as_mut() {
            builder.add_i64_field(name, opts);
        } else {
            return Err(exceptions::PyValueError::new_err(
                "Schema builder object isn't valid anymore.",
            ));
        }
        Ok(self.clone())
    }

    /// Add a new float field to the schema.
    ///
    /// Args:
    ///     name (str): The name of the field.
    ///     stored (bool, optional): If true sets the field as stored, the
    ///         content of the field can be later restored from a Searcher.
    ///         Defaults to False.
    ///     indexed (bool, optional): If true sets the field to be indexed.
    ///     fast (bool, optional): Set the numeric options as a fast field. A
    ///         fast field is a column-oriented fashion storage for tantivy.
    ///         It is designed for the fast random access of some document
    ///         fields given a document id.
    ///
    /// Returns the associated field handle.
    /// Raises a ValueError if there was an error with the field creation.
    #[pyo3(signature = (name, stored = false, indexed = false, fast = false))]
    fn add_float_field(
        &mut self,
        name: &str,
        stored: bool,
        indexed: bool,
        fast: bool,
    ) -> PyResult<Self> {
        let builder = &mut self.builder;

        let opts = SchemaBuilder::build_numeric_option(stored, indexed, fast)?;

        if let Some(builder) = builder.write().unwrap().as_mut() {
            builder.add_f64_field(name, opts);
        } else {
            return Err(exceptions::PyValueError::new_err(
                "Schema builder object isn't valid anymore.",
            ));
        }
        Ok(self.clone())
    }

    /// Add a new unsigned integer field to the schema.
    ///
    /// Args:
    ///     name (str): The name of the field.
    ///     stored (bool, optional): If true sets the field as stored, the
    ///         content of the field can be later restored from a Searcher.
    ///         Defaults to False.
    ///     indexed (bool, optional): If true sets the field to be indexed.
    ///     fast (bool, optional): Set the numeric options as a fast field. A
    ///         fast field is a column-oriented fashion storage for tantivy.
    ///         It is designed for the fast random access of some document
    ///         fields given a document id.
    ///
    /// Returns the associated field handle.
    /// Raises a ValueError if there was an error with the field creation.
    #[pyo3(signature = (name, stored = false, indexed = false, fast = false))]
    fn add_unsigned_field(
        &mut self,
        name: &str,
        stored: bool,
        indexed: bool,
        fast: bool,
    ) -> PyResult<Self> {
        let builder = &mut self.builder;

        let opts = SchemaBuilder::build_numeric_option(stored, indexed, fast)?;

        if let Some(builder) = builder.write().unwrap().as_mut() {
            builder.add_u64_field(name, opts);
        } else {
            return Err(exceptions::PyValueError::new_err(
                "Schema builder object isn't valid anymore.",
            ));
        }
        Ok(self.clone())
    }

    /// Add a new boolean field to the schema.
    ///
    /// Args:
    ///     name (str): The name of the field.
    ///     stored (bool, optional): If true sets the field as stored, the
    ///         content of the field can be later restored from a Searcher.
    ///         Defaults to False.
    ///     indexed (bool, optional): If true sets the field to be indexed.
    ///     fast (bool, optional): Set the numeric options as a fast field. A
    ///         fast field is a column-oriented fashion storage for tantivy.
    ///         It is designed for the fast random access of some document
    ///         fields given a document id.
    ///
    /// Returns the associated field handle.
    /// Raises a ValueError if there was an error with the field creation.
    #[pyo3(signature = (name, stored = false, indexed = false, fast = false))]
    fn add_boolean_field(
        &mut self,
        name: &str,
        stored: bool,
        indexed: bool,
        fast: bool,
    ) -> PyResult<Self> {
        let builder = &mut self.builder;

        let opts = SchemaBuilder::build_numeric_option(stored, indexed, fast)?;

        if let Some(builder) = builder.write().unwrap().as_mut() {
            builder.add_bool_field(name, opts);
        } else {
            return Err(exceptions::PyValueError::new_err(
                "Schema builder object isn't valid anymore.",
            ));
        }
        Ok(self.clone())
    }

    /// Add a new date field to the schema.
    ///
    /// Args:
    ///     name (str): The name of the field.
    ///     stored (bool, optional): If true sets the field as stored, the
    ///         content of the field can be later restored from a Searcher.
    ///         Defaults to False.
    ///     indexed (bool, optional): If true sets the field to be indexed.
    ///     fast (bool, optional): Set the date options as a fast field. A fast
    ///         field is a column-oriented fashion storage for tantivy. It is
    ///         designed for the fast random access of some document fields
    ///         given a document id.
    ///
    /// Returns the associated field handle.
    /// Raises a ValueError if there was an error with the field creation.
    #[pyo3(signature = (name, stored = false, indexed = false, fast = false))]
    fn add_date_field(
        &mut self,
        name: &str,
        stored: bool,
        indexed: bool,
        fast: bool,
    ) -> PyResult<Self> {
        let builder = &mut self.builder;

        let mut opts = DateOptions::default();
        if stored {
            opts = opts.set_stored();
        }
        if indexed {
            opts = opts.set_indexed();
        }
        if fast {
            opts = opts.set_fast();
        }

        if let Some(builder) = builder.write().unwrap().as_mut() {
            builder.add_date_field(name, opts);
        } else {
            return Err(exceptions::PyValueError::new_err(
                "Schema builder object isn't valid anymore.",
            ));
        }
        Ok(self.clone())
    }

    /// Add a new json field to the schema.
    ///
    /// Args:
    ///     name (str): the name of the field.
    ///     stored (bool, optional): If true sets the field as stored, the
    ///         content of the field can be later restored from a Searcher.
    ///         Defaults to False.
    ///     fast (bool, optional): Set the text options as a fast field. A
    ///         fast field is a column-oriented fashion storage for tantivy.
    ///         Text fast fields will have the term ids stored in the fast
    ///         field. The fast field will be a multivalued fast field.
    ///         It is recommended to use the "raw" tokenizer, since it will
    ///         store the original text unchanged. The "default" tokenizer will
    ///         store the terms as lower case and this will be reflected in the
    ///         dictionary.
    ///     tokenizer_name (str, optional): The name of the tokenizer that
    ///         should be used to process the field. Defaults to 'default'
    ///     index_option (str, optional): Sets which information should be
    ///         indexed with the tokens. Can be one of 'position', 'freq' or
    ///         'basic'. Defaults to 'position'. The 'basic' index_option
    ///         records only the document ID, the 'freq' option records the
    ///         document id and the term frequency, while the 'position' option
    ///         records the document id, term frequency and the positions of
    ///         the term occurrences in the document.
    ///
    /// Returns the associated field handle.
    /// Raises a ValueError if there was an error with the field creation.
    #[pyo3(signature = (
        name,
        stored = false,
        fast = false,
        tokenizer_name = TOKENIZER,
        index_option = RECORD
    ))]
    fn add_json_field(
        &mut self,
        name: &str,
        stored: bool,
        fast: bool,
        tokenizer_name: &str,
        index_option: &str,
    ) -> PyResult<Self> {
        let builder = &mut self.builder;
        let options = SchemaBuilder::build_text_option(
            stored,
            fast,
            tokenizer_name,
            index_option,
        )?;

        if let Some(builder) = builder.write().unwrap().as_mut() {
            builder.add_json_field(name, options);
        } else {
            return Err(exceptions::PyValueError::new_err(
                "Schema builder object isn't valid anymore.",
            ));
        }

        Ok(self.clone())
    }

    /// Add a Facet field to the schema.
    /// Args:
    ///     name (str): The name of the field.
    fn add_facet_field(&mut self, name: &str) -> PyResult<Self> {
        let builder = &mut self.builder;

        if let Some(builder) = builder.write().unwrap().as_mut() {
            builder.add_facet_field(name, INDEXED);
        } else {
            return Err(exceptions::PyValueError::new_err(
                "Schema builder object isn't valid anymore.",
            ));
        }
        Ok(self.clone())
    }

    /// Add a fast bytes field to the schema.
    ///
    /// Args:
    ///     name (str): The name of the field.
    ///     stored (bool, optional): If true sets the field as stored, the
    ///         content of the field can be later restored from a Searcher.
    ///         Defaults to False.
    ///     indexed (bool, optional): If true sets the field to be indexed.
    ///     fast (bool, optional): Set the bytes options as a fast field. A fast
    ///         field is a column-oriented fashion storage for tantivy. It is
    ///         designed for the fast random access of some document fields
    ///         given a document id.
    #[pyo3(signature = (
        name,
        stored = false,
        indexed = false,
        fast = false
    ))]
    fn add_bytes_field(
        &mut self,
        name: &str,
        stored: bool,
        indexed: bool,
        fast: bool,
    ) -> PyResult<Self> {
        let builder = &mut self.builder;
        let mut opts = BytesOptions::default();
        if stored {
            opts = opts.set_stored();
        }
        if indexed {
            opts = opts.set_indexed();
        }
        if fast {
            opts = opts.set_fast();
        }

        if let Some(builder) = builder.write().unwrap().as_mut() {
            builder.add_bytes_field(name, opts);
        } else {
            return Err(exceptions::PyValueError::new_err(
                "Schema builder object isn't valid anymore.",
            ));
        }
        Ok(self.clone())
    }

    /// Add an IP address field to the schema.
    ///
    /// Args:
    ///     name (str): The name of the field.
    ///     stored (bool, optional): If true sets the field as stored, the
    ///         content of the field can be later restored from a Searcher.
    ///         Defaults to False.
    ///     indexed (bool, optional): If true sets the field to be indexed.
    ///     fast (bool, optional): Set the IP address options as a fast field. A
    ///         fast field is a column-oriented fashion storage for tantivy. It
    ///         is designed for the fast random access of some document fields
    ///         given a document id.
    #[pyo3(signature = (
        name,
        stored = false,
        indexed = false,
        fast = false
    ))]
    fn add_ip_addr_field(
        &mut self,
        name: &str,
        stored: bool,
        indexed: bool,
        fast: bool,
    ) -> PyResult<Self> {
        let builder = &mut self.builder;
        let mut opts = IpAddrOptions::default();
        if stored {
            opts = opts.set_stored();
        }
        if indexed {
            opts = opts.set_indexed();
        }
        if fast {
            opts = opts.set_fast();
        }

        if let Some(builder) = builder.write().unwrap().as_mut() {
            builder.add_ip_addr_field(name, opts);
        } else {
            return Err(exceptions::PyValueError::new_err(
                "Schema builder object isn't valid anymore.",
            ));
        }

        Ok(self.clone())
    }

    /// Finalize the creation of a Schema.
    ///
    /// Returns a Schema object. After this is called the SchemaBuilder cannot
    /// be used anymore.
    fn build(&mut self) -> PyResult<Schema> {
        let builder = self.builder.write().unwrap().take();
        if let Some(builder) = builder {
            let schema = builder.build();
            Ok(Schema { inner: schema })
        } else {
            Err(exceptions::PyValueError::new_err(
                "Schema builder object isn't valid anymore.",
            ))
        }
    }
}

impl SchemaBuilder {
    fn build_numeric_option(
        stored: bool,
        indexed: bool,
        fast: bool,
    ) -> PyResult<schema::NumericOptions> {
        let opts = schema::NumericOptions::default();
        let opts = if stored { opts.set_stored() } else { opts };
        let opts = if indexed { opts.set_indexed() } else { opts };
        let opts = if fast { opts.set_fast() } else { opts };
        Ok(opts)
    }

    fn build_text_option(
        stored: bool,
        fast: bool,
        tokenizer_name: &str,
        index_option: &str,
    ) -> PyResult<schema::TextOptions> {
        let index_option = match index_option {
            "position" => schema::IndexRecordOption::WithFreqsAndPositions,
            "freq" => schema::IndexRecordOption::WithFreqs,
            "basic" => schema::IndexRecordOption::Basic,
            _ => return Err(exceptions::PyValueError::new_err(
                "Invalid index option, valid choices are: 'basic', 'freq' and 'position'"
            ))
        };

        let indexing = schema::TextFieldIndexing::default()
            .set_tokenizer(tokenizer_name)
            .set_index_option(index_option);

        let options =
            schema::TextOptions::default().set_indexing_options(indexing);
        let options = if stored {
            options.set_stored()
        } else {
            options
        };

        let options = if fast {
            let text_tokenizer = if tokenizer_name != NO_TOKENIZER_NAME {
                Some(tokenizer_name)
            } else {
                None
            };
            options.set_fast(text_tokenizer)
        } else {
            options
        };

        Ok(options)
    }
}
