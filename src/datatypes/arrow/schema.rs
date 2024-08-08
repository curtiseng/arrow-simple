use arrow::datatypes::Fields;

pub struct Schema {
    fields: Vec<arrow::datatypes::Field>,
    fields_ref: Fields,
}