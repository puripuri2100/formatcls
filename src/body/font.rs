use serde_json::json;
use serde_json::Value;

pub const NAME_MAIN_FONT: &str = "main-font";
pub const NAME_FONT_DATA: &str = "font-data";

pub const NAME_SIZE: &str = "size";
pub const NAME_CJK_NAME: &str = "cjk-name";
pub const NAME_CJK_RATIO: &str = "cjk-ratio";
pub const NAME_CJK_CORRECTION: &str = "latin-correction";
pub const NAME_LATIN_NAME: &str = "latin-name";
pub const NAME_LATIN_RATIO: &str = "latin-ratio";
pub const NAME_LATIN_CORRECTION: &str = "latin-correction";

pub const DEFAULT_SIZE: &str = "12pt";
pub const DEFAULT_CJK_NAME: &str = "ipaexm";
pub const DEFAULT_CJK_RATIO: &str = "0.88";
pub const DEFAULT_CJK_CORRECTION: &str = "0.0";
pub const DEFAULT_LATIN_NAME: &str = "Junicode";
pub const DEFAULT_LATIN_RATIO: &str = "1.0";
pub const DEFAULT_LATIN_CORRECTION: &str = "0.0";

pub fn make_default_font_vec() -> Vec<(&'static str, &'static str, &'static str, &'static str)> {
  let v = vec![
    ("roman", "Junicode", "1.0", "0.0"),
    ("bold", "Junicode-b", "1.0", "0.0"),
    ("italic", "Junicode-it", "1.0", "0.0"),
    ("sans", "lmsans", "1.0", "0.0"),
    ("mono", "lmmono", "1.0", "0.0"),
    ("mincho", "ipaexm", "0.88", "0.0"),
    ("gothic", "ipaexg", "0.88", "0.0"),
  ];
  v
}

pub fn set_default_font() -> String {
  let default_vec = make_default_font_vec();
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
  for font in font_vec {
    let (tag, name, ratio, correction) = &font;
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

fn set_font_vec(font_data: &[Value]) -> Vec<(String, Option<String>, String, String)> {
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

pub fn make_let(n: &str, v: &str) -> String {
  format!("let {} = {}\n", n, v)
}

pub fn make_main_font_str(
  main_font_size: &str,
  main_font_name_cjk: &str,
  main_font_ratio_cjk: &str,
  main_font_correction_cjk: &str,
  main_font_name_latin: &str,
  main_font_ratio_latin: &str,
  main_font_correction_latin: &str,
) -> String {
  format!(
    "

let main-font-size = {}
let main-font-cjk = (```{}```, {}, {})
let main-font-latin = (```{}```, {}, {})

",
    main_font_size,
    main_font_name_cjk,
    main_font_ratio_cjk,
    main_font_correction_cjk,
    main_font_name_latin,
    main_font_ratio_latin,
    main_font_correction_latin
  )
}

pub fn set_set_fn() -> String {
  "

% フォント変更用関数
let set-cjk-font font-name ctx =
  ctx |> set-font HanIdeographic font-name
      |> set-font Kana font-name


let set-latin-font font-name ctx =
  ctx |> set-font Latin font-name

  "
  .to_string()
}
