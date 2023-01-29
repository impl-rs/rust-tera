use markdown::to_html;
use std::collections::HashMap;
use tera::{try_get_value, Result, Value};

pub fn markdown_to_html(value: &Value, _: &HashMap<String, Value>) -> Result<Value> {
    let markdown_string = try_get_value!("markdown", "value", String, value);

    Ok(to_html(markdown_string.as_str()).into())
}
