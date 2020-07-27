use serde_json::json;
use serde_json::Value;

pub const NAME_REQUIRE_PACKAGE: &str = "require-package";
pub const NAME_IMPORT_PACKAGE: &str = "import-package";

pub const DEFAULT_REQUIRE_PACKAGE_VEC_STR: Option<&str> = None;
pub const DEFAULT_IMPORT_PACKAGE_VEC_STR: Option<&str> = None;

pub fn default_json() -> Value {
  json!({
    NAME_REQUIRE_PACKAGE:[],
    NAME_IMPORT_PACKAGE:[]
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
  let mut st = String::new();
  for v_elem in v {
    let s = format!("@require: {}\n", v_elem);
    st.push_str(&s)
  }
  st
}

fn make_import(v: Vec<String>) -> String {
  let mut st = String::new();
  for v_elem in v {
    let s = format!("@import: {}\n", v_elem);
    st.push_str(&s)
  }
  st
}
