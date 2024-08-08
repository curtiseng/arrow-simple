use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum DataType {
    Int32(int32_type::Int32Type)
}

impl<'de> Deserialize<'de> for DataType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        Ok(DataType::Int32(int32_type::Int32Type))
    }
}

pub mod int32_type;