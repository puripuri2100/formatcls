use serde_json::json;
use serde_json::Value;

pub fn set_default_font() -> String {
  let default_vec = vec![
    ("roman", "Junicode", "1.0", "0.0"),
    ("bold", "Junicode-b", "1.0", "0.0"),
    ("italic", "Junicode-it", "1.0", "0.0"),
    ("sans", "lmsans", "1.0", "0.0"),
    ("mono", "lmmono", "1.0", "0.0"),
    ("mincho", "ipaexm", "0.88", "0.0"),
    ("gothic", "ipaexg", "0.88", "0.0"),
  ];
  let mut s = String::new();
  for (tag, name, ratio, correction) in default_vec.iter() {
    let st = format!(
      "let font-{} = (```{}```,{},{})\n",
      tag, name, ratio, correction
    );
    s.push_str(&st)
  }
  s
}

pub fn set_font_data(font_data_opt: &&Option<&Vec<Value>>) -> String {
  let font_data = match font_data_opt {
    None => vec![json!(null)],
    Some(v) => v.to_vec(),
  };
  let font_vec = set_font_vec(&font_data);
  let mut s = String::new();
  let vec_len = font_vec.len();
  for i in 0..vec_len {
    let (tag, name, ratio, correction) = &font_vec[i];
    let st = match name {
      None => String::new(),
      Some(n) => format!(
        "let font-{} = (```{}```,{},{})\n",
        tag, n, ratio, correction
      ),
    };
    s.push_str(&st)
  }
  s
}

fn set_font_vec(font_data: &Vec<Value>) -> Vec<(String, Option<String>, String, String)> {
  let mut stack = vec![];
  let mut data_vec = vec![];
  for j in font_data.iter() {
    let tag = j["tag"].as_str();
    let name = j["name"].as_str();
    let ratio = j["ratio"].as_str().unwrap_or("1.0");
    let correction = j["correction"].as_str().unwrap_or("0.0");
    data_vec.push((tag, name, ratio, correction))
  }
  for (tag_opt, name_opt, ratio, correction) in data_vec.iter() {
    match (tag_opt, name_opt) {
      (Some(tag), Some(name)) => stack.push((
        tag.to_string(),
        Some(name.to_string()),
        ratio.to_string(),
        correction.to_string(),
      )),
      _ => stack.push((String::new(), None, String::new(), String::new())),
    }
  }
  stack.to_vec()
}
