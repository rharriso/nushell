use crate::value::Value;
use derive_new::new;
use indexmap::IndexMap;
use nu_source::Tag;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CallInfo {
    pub args: EvaluatedArgs,
    pub name_tag: Tag,
}

#[derive(Debug, Default, new, Serialize, Deserialize, Clone)]
pub struct EvaluatedArgs {
    pub positional: Option<Vec<Value>>,
    pub named: Option<IndexMap<String, Value>>,
}
