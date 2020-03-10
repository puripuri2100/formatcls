use serde_json::json;
use serde_json::Value;

pub fn make_default_json(
  header: Vec<(String, String)>,
  module: Vec<(String, String)>,
  body: Vec<(String, String)>,
) -> Value {
  let mut main_str = String::new();
  for (tag, v) in header.iter() {
    main_str.push_str(&format!("\"{}\":{}", tag, v))
  }
  for (tag, v) in module.iter() {
    main_str.push_str(&format!("\"{}\":{}", tag, v))
  }
  for (tag, v) in body.iter() {
    main_str.push_str(&format!("\"{}\":{}", tag, v))
  }
  let json_date = format!("{}{}{}", "{", main_str, "}");
  let v: Value = serde_json::from_str(&json_date).unwrap_or(json!(null));
  v
}
