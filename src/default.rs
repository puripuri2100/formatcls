use json_patch::merge;
use serde_json::json;
use serde_json::Value;

pub fn merge_default_json(h: Value, m: Value, b: Value) -> Value {
  let mut main = json!(null);
  merge(&mut main, &h);
  merge(&mut main, &m);
  merge(&mut main, &b);
  main
}

#[test]
fn check_make_json() {
  let h = json!({"u":200});
  let m = json!({"bool":true});
  let b = json!({
    "json": {
      "str_array": [
          "hoge",
          "fuga"
      ]
    }
  });
  let r = json!({
    "u":200,
    "bool":true,
    "json": {
      "str_array": [
          "hoge",
          "fuga"
      ]
    }
  });
  assert_eq!(merge_default_json(h, m, b), r);
}
