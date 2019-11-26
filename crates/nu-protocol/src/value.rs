pub mod column_path;
mod debug;
pub mod dict;
pub mod evaluate;
pub mod primitive;
mod serde_bigdecimal;
mod serde_bigint;

use crate::errors::ShellError;
use crate::value::dict::Dictionary;
use crate::value::evaluate::Evaluate;
use crate::value::primitive::Primitive;
use nu_source::Tag;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UntaggedValue {
    Primitive(Primitive),
    Row(Dictionary),
    Table(Vec<Value>),

    // Errors are a type of value too
    Error(ShellError),

    Block(Box<dyn Evaluate>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Value {
    pub value: UntaggedValue,
    pub tag: Tag,
}

impl std::ops::Deref for Value {
    type Target = UntaggedValue;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl UntaggedValue {
    pub fn into_value(self, tag: impl Into<Tag>) -> Value {
        Value {
            value: self,
            tag: tag.into(),
        }
    }

    pub fn into_untagged_value(self) -> Value {
        Value {
            value: self,
            tag: Tag::unknown(),
        }
    }
}
