use serde::{Deserialize, Serialize};
use crate::datatypes::column::Column;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubTable {
    columns: Vec<Column>,
}

