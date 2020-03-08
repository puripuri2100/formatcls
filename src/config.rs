use serde_json::Value;
use std::fs;

pub fn parse(path: &str) -> Value {
  let data = fs::read_to_string(path).unwrap();
  serde_json::from_str(&data).unwrap()
}

pub fn require_package(v: Vec<&str>) -> String {
  let len = v.len();
  let mut st = String::new();
  for i in 0..len {
    let s = format!("@require: {}\n", v[i]);
    st.push_str(&s)
  }
  st
}
