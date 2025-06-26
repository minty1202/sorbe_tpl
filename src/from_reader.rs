use kernel::{error::Error, value::Value};

use super::{from_str, from_str_with_schema};

use std::io::Read;

pub fn from_reader<R: Read>(mut reader: R) -> Result<Value, Error> {
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;
    from_str(&contents)
}

pub fn from_reader_with_schema<R1: Read, R2: Read>(
    mut config_reader: R1,
    mut schema_reader: R2,
) -> Result<Value, Error> {
    let mut config_contents = String::new();
    let mut schema_contents = String::new();

    config_reader.read_to_string(&mut config_contents)?;
    schema_reader.read_to_string(&mut schema_contents)?;

    from_str_with_schema(&config_contents, &schema_contents)
}
