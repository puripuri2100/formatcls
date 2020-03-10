use serde_json::json;
use serde_json::Value;

pub const DEFAULT_DOCUMENT_FUNCTION_NAME: &str = "document";
pub const DEFAULT_MODULE_NAME: &str = "MyCls";

pub fn default_json() -> Value {
  json!({
    "doc-fun-name":DEFAULT_DOCUMENT_FUNCTION_NAME,
    "module":DEFAULT_MODULE_NAME
  })
}
