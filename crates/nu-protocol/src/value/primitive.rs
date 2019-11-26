use crate::value::column_path::ColumnPath;
use crate::value::{serde_bigdecimal, serde_bigint};
use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use num_bigint::BigInt;
use num_traits::cast::FromPrimitive;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
pub enum Primitive {
    Nothing,
    #[serde(with = "serde_bigint")]
    Int(BigInt),
    #[serde(with = "serde_bigdecimal")]
    Decimal(BigDecimal),
    Bytes(u64),
    String(String),
    ColumnPath(ColumnPath),
    Pattern(String),
    Boolean(bool),
    Date(DateTime<Utc>),
    Duration(u64), // Duration in seconds
    Path(PathBuf),
    #[serde(with = "serde_bytes")]
    Binary(Vec<u8>),

    // Stream markers (used as bookend markers rather than actual values)
    BeginningOfStream,
    EndOfStream,
}

impl From<BigDecimal> for Primitive {
    fn from(decimal: BigDecimal) -> Primitive {
        Primitive::Decimal(decimal)
    }
}

impl From<f64> for Primitive {
    fn from(float: f64) -> Primitive {
        Primitive::Decimal(BigDecimal::from_f64(float).unwrap())
    }
}
