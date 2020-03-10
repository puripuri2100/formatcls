use serde_json::json;
use serde_json::Value;

pub mod command;
pub mod doc;
pub mod font;
pub mod func;
pub mod page;

pub fn body(v: &Value, document_function_name: &str) -> String {
  let main_font_size = &v[font::NAME_MAIN_FONT][font::NAME_SIZE]
    .as_str()
    .unwrap_or(font::DEFAULT_SIZE);
  let main_font_name_cjk = &v[font::NAME_MAIN_FONT][font::NAME_CJK_NAME]
    .as_str()
    .unwrap_or(font::DEFAULT_CJK_NAME);
  let main_font_ratio_cjk = &v[font::NAME_MAIN_FONT][font::NAME_CJK_RATIO]
    .as_str()
    .unwrap_or(font::DEFAULT_CJK_RATIO);
  let main_font_correction_cjk = &v[font::NAME_MAIN_FONT][font::NAME_CJK_CORRECTION]
    .as_str()
    .unwrap_or(font::DEFAULT_CJK_CORRECTION);
  let main_font_name_latin = &v[font::NAME_MAIN_FONT][font::NAME_LATIN_NAME]
    .as_str()
    .unwrap_or(font::DEFAULT_LATIN_NAME);
  let main_font_ratio_latin = &v[font::NAME_MAIN_FONT][font::NAME_LATIN_CORRECTION]
    .as_str()
    .unwrap_or(font::DEFAULT_LATIN_CORRECTION);
  let main_font_correction_latin = &v[font::NAME_MAIN_FONT][font::NAME_LATIN_CORRECTION]
    .as_str()
    .unwrap_or(font::DEFAULT_LATIN_CORRECTION);
  let main_font_str = font::make_main_font_str(
    &main_font_size,
    &main_font_name_cjk,
    &main_font_ratio_cjk,
    &main_font_correction_cjk,
    &main_font_name_latin,
    &main_font_ratio_latin,
    &main_font_correction_latin,
  );

  let top_space = &v[page::NAME_TOP_SPACE]
    .as_str()
    .unwrap_or(page::DEFAULT_TOP_SPACE);
  let bottom_space = &v[page::NAME_BOTTOM_SPACE]
    .as_str()
    .unwrap_or(page::DEFAULT_BOTTOM_SPACE);
  let left_space = &v[page::NAME_LEFT_SPACE]
    .as_str()
    .unwrap_or(page::DEFAULT_LEFT_SPACE);
  let right_space = &v[page::NAME_RIGHT_SPACE]
    .as_str()
    .unwrap_or(page::DEFAULT_RIGHT_SPACE);
  let space_str = page::make_space_str(top_space, bottom_space, left_space, right_space);

  let page_width = &v[page::NAME_PAGE_WIDTH]
    .as_str()
    .unwrap_or(page::DEFAULT_PAGE_WIDTH);
  let page_height = &v[page::NAME_PAGE_HEIGHT]
    .as_str()
    .unwrap_or(page::DEFAULT_PAGE_HEIGHT);
  let page_size = &v[page::NAME_PAGE_SIZE].as_str();
  let page_size_str = page::make_page_size_str(page_size, page_width, page_height);

  let font_data = &v["font-data"].as_array();

  let def_value_vec = vec![
    main_font_str,
    font::set_default_font(),
    font::set_font_data(&font_data),
    page_size_str,
    space_str,
    font::set_set_fn(),
    font::set_initial_context(),
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
    page::NAME_PAGE_WIDTH   : page::DEFAULT_PAGE_WIDTH,
    page::NAME_PAGE_HEIGHT  : page::DEFAULT_PAGE_HEIGHT,
    page::NAME_PAGE_SIZE    : page::DEFAULT_PAGE_SIZE,
    page::NAME_TOP_SPACE    : page::DEFAULT_TOP_SPACE,
    page::NAME_BOTTOM_SPACE : page::DEFAULT_BOTTOM_SPACE,
    page::NAME_LEFT_SPACE   : page::DEFAULT_LEFT_SPACE,
    page::NAME_RIGHT_SPACE  : page::DEFAULT_RIGHT_SPACE,
    font::NAME_MAIN_FONT    : json!({
      font::NAME_SIZE             : font::DEFAULT_SIZE,
      font::NAME_CJK_NAME         : font::DEFAULT_CJK_NAME,
      font::NAME_CJK_RATIO        : font::DEFAULT_CJK_RATIO,
      font::NAME_CJK_CORRECTION   : font::DEFAULT_CJK_CORRECTION,
      font::NAME_LATIN_NAME       : font::DEFAULT_LATIN_NAME,
      font::NAME_LATIN_RATIO      : font::DEFAULT_LATIN_RATIO,
      font::NAME_LATIN_CORRECTION : font::DEFAULT_LATIN_CORRECTION,
    }),
  })
}

pub fn make_command_vec() -> Vec<String> {
  let def_module_vec = vec![
    command::DIRECT_P_CMD.to_string(),
    command::DIRECT_PN_CMD.to_string(),
  ];
  def_module_vec
}
