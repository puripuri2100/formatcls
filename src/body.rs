use serde_json::json;
use serde_json::Value;

pub mod command;
pub mod doc;
pub mod font;
pub mod func;
pub mod setting;

pub fn body(v: &Value) -> String {
  let page_width = &v["page-width"]
    .as_str()
    .unwrap_or(setting::DEFAULT_PAGE_WIDTH);
  let page_height = &v["page-height"]
    .as_str()
    .unwrap_or(setting::DEFAULT_PAGE_HEIGHT);
  let page_size = &v["page-size"].as_str();

  let main_font_size = &v["main-font"]["font-size"]
    .as_str()
    .unwrap_or(setting::DEFAULT_FONT_SIZE);
  let main_font_name_cjk = &v["main-font"]["cjk-name"].as_str().unwrap_or("hoge");
  let main_font_ratio_cjk = &v["main-font"]["cjk-ratio"].as_str().unwrap_or("hoge");
  let main_font_correction_cjk = &v["main-font"]["cjk-correction"].as_str().unwrap_or("hoge");
  let main_font_name_latin = &v["main-font"]["latin-name"].as_str().unwrap_or("hoge");
  let main_font_ratio_latin = &v["main-font"]["latin-ratio"].as_str().unwrap_or("hoge");
  let main_font_correction_latin = &v["main-font"]["latin-correction"]
    .as_str()
    .unwrap_or("hoge");

  let top_space = &v["top-space"]
    .as_str()
    .unwrap_or(setting::DEFAULT_TOP_SPACE);
  let bottom_space = &v["bottom-space"]
    .as_str()
    .unwrap_or(setting::DEFAULT_BOTTOM_SPACE);
  let left_space = &v["left-space"]
    .as_str()
    .unwrap_or(setting::DEFAULT_LEFT_SPACE);
  let right_space = &v["right-space"]
    .as_str()
    .unwrap_or(setting::DEFAULT_RIGHT_SPACE);

  let font_data = &v["font-data"].as_array();

  let document_function_name = &v["doc-fun-name"]
    .as_str()
    .unwrap_or(setting::DEFAULT_DOCUMENT_FUNCTION_NAME);
  let document_config_data = &v["config-data"].as_array();

  let def_value_vec = vec![
    setting::set_main_font(
      &main_font_size,
      &main_font_name_cjk,
      &main_font_ratio_cjk,
      &main_font_correction_cjk,
      &main_font_name_latin,
      &main_font_ratio_latin,
      &main_font_correction_latin,
    ),
    font::set_default_font(),
    font::set_font_data(&font_data),
    setting::set_page_size(&page_size, &page_width, &page_height),
    setting::make_let("top-space", &top_space),
    setting::make_let("bottom-space", &bottom_space),
    setting::make_let("left-space", &left_space),
    setting::make_let("right-space", &right_space),
    setting::make_let("text-height", " page-height -' top-space -' bottom-space"),
    setting::make_let("text-width", " page-width -' left-space -' right-space"),
    setting::set_set_fn(),
    setting::set_initial_context(),
    func::make_header(),
    func::make_footer(),
    doc::make_document_function(&document_function_name),
    command::DEF_P_CMD.to_string(),
    command::DEF_PN_CMD.to_string(),
  ];
  let value_str = vec_to_str(&def_value_vec);
  value_str
}

fn vec_to_str(v: &Vec<String>) -> String {
  let len = v.len();
  let mut s = String::new();
  for i in 0..len {
    let st = format!("{}", v[i]);
    s.push_str(&st)
  }
  s
}

pub fn default_json() -> Value {
  json!({
    "font-size": setting::DEFAULT_FONT_SIZE,
    "page-width" : setting::DEFAULT_PAGE_WIDTH,
    "page-height" : setting::DEFAULT_PAGE_HEIGHT,
    "page-size" : setting::DEFAULT_PAGE_SIZE,
    "top-space" : setting::DEFAULT_TOP_SPACE,
    "bottom-space" : setting::DEFAULT_BOTTOM_SPACE,
    "left-space" : setting::DEFAULT_LEFT_SPACE,
    "right-space" : setting::DEFAULT_RIGHT_SPACE,
  })
}

pub fn make_command_vec() -> Vec<String> {
  let def_module_vec = vec![
    command::DIRECT_P_CMD.to_string(),
    command::DIRECT_PN_CMD.to_string(),
  ];
  def_module_vec
}
