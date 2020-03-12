use serde_json::json;
use serde_json::Value;

pub const NAME_DOC_FUNCTION_NAME: &str = "doc-fun-name";
pub const NAME_MODULE_NAME: &str = "module";

pub const DEFAULT_DOC_FUNCTION_NAME: &str = "document";
pub const DEFAULT_MODULE_NAME: &str = "MyCls";

pub fn default_json() -> Value {
  json!({
    NAME_DOC_FUNCTION_NAME:DEFAULT_DOC_FUNCTION_NAME,
    NAME_MODULE_NAME:DEFAULT_MODULE_NAME
  })
}

pub fn make_sig(doc_name: &str, command_vec: Vec<String>) -> String {
  let doc_str = format!(
    "val {} : 'a -> block-text -> document
  constraint 'a :: (|
    title : inline-text;
    author : inline-text;
    other : 'b;
  |)\n",
    doc_name
  );
  let mut main_str = doc_str;
  for s in command_vec.iter() {
    main_str.push_str(&format!("{}\n", s))
  }
  main_str
}
