#![recursion_limit = "1024"]

#[cfg(test)]
#[macro_use]
extern crate indexmap;

#[macro_use]
mod prelude;

mod cli;
mod commands;
mod context;
mod data;
mod env;
mod evaluate;
mod format;
mod git;
mod parser;
mod plugin;
mod shell;
mod stream;
mod utils;

pub use crate::cli::cli;
pub use crate::data::base::property_get::ValueExt;
pub use crate::data::config::{config_path, APP_INFO};
pub use crate::data::dict::{TaggedDictBuilder, TaggedListBuilder};
pub use crate::data::value;
pub use crate::data::primitive;
pub use crate::env::host::BasicHost;
pub use crate::parser::parse::token_tree_builder::TokenTreeBuilder;
pub use crate::plugin::{serve_plugin, Plugin};
pub use crate::utils::{did_you_mean, AbsoluteFile, AbsolutePath, RelativePath};
pub use num_traits::cast::ToPrimitive;
