use crate::value::Value;
use indexmap::IndexMap;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Scope {
    it: Value,
    vars: IndexMap<String, Value>,
}

#[typetag::serde(tag = "type")]
pub trait Evaluate: Debug + Send {
    fn evaluate(&self, scope: &Scope) -> Value;
    fn clone_box(&self) -> Box<dyn Evaluate>;
}

impl Clone for Box<dyn Evaluate> {
    fn clone(&self) -> Box<dyn Evaluate> {
        self.clone_box()
    }
}
