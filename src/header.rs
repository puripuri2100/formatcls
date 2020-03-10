use serde_json::Value;

pub fn default_vec() -> Vec<(String, String)> {
  vec![]
}

pub fn package(default: Vec<&str>, require: &Value, import: &Value) -> String {
  let default_str = make_require(default);
  let require_vec = value_to_str_vec(require);
  let require_str = make_require(require_vec);
  let import_vec = value_to_str_vec(import);
  let import_str = make_import(import_vec);
  format!("{}\n{}\n{}", default_str, require_str, import_str)
}

fn make_require(v: Vec<&str>) -> String {
  let len = v.len();
  let mut st = String::new();
  for i in 0..len {
    let s = format!("@require: {}\n", v[i]);
    st.push_str(&s)
  }
  st
}

fn make_import(v: Vec<&str>) -> String {
  let len = v.len();
  let mut st = String::new();
  for i in 0..len {
    let s = format!("@import: {}\n", v[i]);
    st.push_str(&s)
  }
  st
}

fn value_to_str_vec(j: &Value) -> Vec<&str> {
  let j_vec_op = j.as_array();
  let mut s_vec = vec![];
  match j_vec_op {
    None => {}
    Some(j_vec) => {
      let len = j_vec.len();
      for i in 0..len {
        let v = &j_vec[i];
        let s_op = v.as_str();
        match s_op {
          None => {}
          Some(s) => s_vec.push(s),
        }
      }
    }
  }
  s_vec.to_vec()
}
