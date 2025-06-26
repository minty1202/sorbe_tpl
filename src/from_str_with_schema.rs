use kernel::{error::Error, value::Value};

use super::cast::cast;
use super::from_schema::from_schema;
use super::from_str;
use super::validation::validate;

pub fn from_str_with_schema(input: &str, schema: &str) -> Result<Value, Error> {
    let value = from_str(input)?;
    let schema = from_schema(schema)?;
    validate(&value, &schema)?;
    let value = cast(&value, &schema);

    Ok(value)
}
