use serde_json::json;
use serde_json::Value;

pub const DEFAULT_REQUIRE_PACKAGE_VEC_STR: Option<&str> = None;
pub const DEFAULT_IMPORT_PACKAGE_VEC_STR: Option<&str> = None;

pub fn default_json() -> Value {
  json!({
    "require-package":[],
    "import-package":[]
  })
}

pub fn package(default: Vec<String>, require: Vec<String>, import: Vec<String>) -> String {
  let default_str = make_require(default);
  let require_vec = require;
  let require_str = make_require(require_vec);
  let import_vec = import;
  let import_str = make_import(import_vec);
  format!("{}\n{}\n{}", default_str, require_str, import_str)
}

fn make_require(v: Vec<String>) -> String {
  let len = v.len();
  let mut st = String::new();
  for i in 0..len {
    let s = format!("@require: {}\n", v[i]);
    st.push_str(&s)
  }
  st
}

fn make_import(v: Vec<String>) -> String {
  let len = v.len();
  let mut st = String::new();
  for i in 0..len {
    let s = format!("@import: {}\n", v[i]);
    st.push_str(&s)
  }
  st
}
