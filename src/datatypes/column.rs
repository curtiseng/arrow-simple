use serde::{Deserialize, Serialize};

use super::types::DataType;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Column {
    pub name: String,
    pub data_type: DataType,
}