use super::{Field, Metadata};

#[cfg(feature = "serde_types")]
use serde_derive::{Deserialize, Serialize};

/// An ordered sequence of [`Field`]s with associated [`Metadata`].
///
/// [`Schema`] is an abstration used to read from, and write to, Arrow IPC format,
/// Apache Parquet, and Apache Avro. All these formats have a concept of a schema
/// with fields and metadata.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde_types", derive(Serialize, Deserialize))]
pub struct Schema {
    /// The fields composing this schema.
    pub fields: Vec<Field>,
    /// Optional metadata.
    pub metadata: Metadata,
}

impl Schema {
    /// Attaches a [`Metadata`] to [`Schema`]
    #[inline]
    pub fn with_metadata(self, metadata: Metadata) -> Self {
        Self {
            fields: self.fields,
            metadata,
        }
    }

    /// Returns a new [`Schema`] with a subset of all fields whose `predicate`
    /// evaluates to true.
    pub fn filter<F: Fn(usize, &Field) -> bool>(self, predicate: F) -> Self {
        let fields = self
            .fields
            .into_iter()
            .enumerate()
            .filter_map(|(index, f)| {
                if (predicate)(index, &f) {
                    Some(f)
                } else {
                    None
                }
            })
            .collect();

        Schema {
            fields,
            metadata: self.metadata,
        }
    }
}

impl From<Vec<Field>> for Schema {
    fn from(fields: Vec<Field>) -> Self {
        Self {
            fields,
            ..Default::default()
        }
    }
}

#[cfg(feature = "arrow")]
impl From<arrow_schema::Schema> for Schema {
    fn from(schema: arrow_schema::Schema) -> Self {
        let arrow_schema::Schema { fields, metadata } = schema;
        Self {
            fields: fields.iter().map(Into::into).collect(),
            metadata: metadata.into_iter().collect(),
        }
    }
}

#[cfg(feature = "arrow")]
impl From<Schema> for arrow_schema::Schema {
    fn from(schema: Schema) -> Self {
        let Schema { fields, metadata } = schema;
        let fields: arrow_schema::Fields =
            fields.into_iter().map(arrow_schema::Field::from).collect();
        Self::new_with_metadata(fields, metadata.into_iter().collect())
    }
}
