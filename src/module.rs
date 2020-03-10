use serde_json::json;
use serde_json::Value;

pub fn default_json() -> Value {
  json!({
    "doc-fun-name":"document",
    "module":"MyCls"
  })
}
