use serde_json::json;
use serde_json::Value;

pub mod command;
pub mod doc;
pub mod font;
pub mod func;
pub mod page;
pub mod sec;
pub mod title;
pub mod toc;

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
  let main_font_ratio_latin = &v[font::NAME_MAIN_FONT][font::NAME_LATIN_RATIO]
    .as_str()
    .unwrap_or(font::DEFAULT_LATIN_RATIO);
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

  let font_data = &v[font::NAME_FONT_DATA].as_array();

  let paragraph_margin_top = &v[func::NAME_PARAGRAPH_MARGIN_TOP]
    .as_str()
    .unwrap_or(func::DEFAULT_PARAGRAPH_MARGIN_TOP);
  let paragraph_margin_bottom = &v[func::NAME_PARAGRAPH_MARGIN_BOTTOM]
    .as_str()
    .unwrap_or(func::DEFAULT_PARAGRAPH_MARGIN_BOTTOM);
  let leading = &v[func::NAME_LEADING]
    .as_str()
    .unwrap_or(func::DEFAULT_LEADING);
  let min_gap_of_lines = &v[func::NAME_MIN_GAP_OF_LINES]
    .as_str()
    .unwrap_or(func::DEFAULT_MIN_GAP_OF_LINES);

  let header_fun = &v[doc::NAME_HEADER_FUN]
    .as_str()
    .unwrap_or(doc::DEFAULT_HEADER_FUN);
  let footer_fun = &v[doc::NAME_FOOTER_FUN]
    .as_str()
    .unwrap_or(doc::DEFAULT_FOOTER_FUN);
  let if_title_page = &v[doc::NAME_IF_TITLE_PAGE]
    .as_bool()
    .unwrap_or(doc::DEFAULT_IF_TITLE_PAGE);
  let if_toc_page = &v[doc::NAME_IF_TOC_PAGE]
    .as_bool()
    .unwrap_or(doc::DEFAULT_IF_TOC_PAGE);

  let title_fun = &v[title::NAME_TITLE_FUN]
    .as_str()
    .unwrap_or(title::DEFAULT_TITLE_FUN);
  let toc_depth = &v[toc::NAME_TOC_DEPTH]
    .as_u64()
    .unwrap_or(toc::DEFAULT_TOC_DEPTH);
  let toc_fun = &v[toc::NAME_TOC_FUN]
    .as_str()
    .unwrap_or(toc::DEFAULT_TOC_FUN);
  let toc_title_fun = &v[toc::NAME_TOC_TITLE_FUN]
    .as_str()
    .unwrap_or(toc::DEFAULT_TOC_TITLE_FUN);
  let sec_depth = &v[sec::NAME_SEC_DEPTH]
    .as_u64()
    .unwrap_or(sec::DEFAULT_SEC_DEPTH);
  let sec_fun_list = json_vec_to_str_vec(
    &v[sec::NAME_SEC_FUN_NAME]
      .as_array()
      .unwrap_or(&vec![json!(null); sec::DEFAULT_SEC_DEPTH as usize]),
    None,
    Some(&sec::default_sec_fun_name()),
  );
  let sec_fun = &v[sec::NAME_SEC_FUN]
    .as_str()
    .unwrap_or(sec::DEFAULT_SEC_FUN);

  let title_str = title::make_title_str(title_fun);
  let toc_str = if toc_depth > sec_depth {
    toc::make_toc_str(sec_depth, toc_fun)
  } else {
    toc::make_toc_str(toc_depth, toc_fun)
  };
  let sec_str = sec::make_sec_str(sec_depth, toc_depth, sec_fun_list, sec_fun);

  let def_value_vec = vec![
    main_font_str,
    font::set_default_font(),
    font::set_font_data(&font_data),
    page_size_str,
    space_str,
    font::set_set_fn(),
    func::set_initial_context(
      paragraph_margin_top,
      paragraph_margin_bottom,
      leading,
      min_gap_of_lines,
    ),
    func::make_register_cross_reference_fun(),
    func::make_get_cross_reference_fun(),
    func::make_register_location_frame_fun(),
    func::make_toc_and_outline_lst(),
    command::DEF_REF_CMD.to_string(),
    command::DEF_REF_PAGE_CMD.to_string(),
    title::title_bb_fun(),
    toc_str,
    func::make_header_footer_fun(),
    sec_str,
    doc::make_document_function(
      &document_function_name,
      title_str,
      toc::make_toc_fun_str(toc_title_fun),
      header_fun,
      footer_fun,
      if_title_page,
      if_toc_page,
    ),
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
    font::NAME_FONT_DATA    : json!(to_font_data_vec(font::make_default_font_vec())),
    doc::NAME_HEADER_FUN    : doc::DEFAULT_HEADER_FUN,
    doc::NAME_FOOTER_FUN    : doc::DEFAULT_FOOTER_FUN,
    doc::NAME_IF_TITLE_PAGE : doc::DEFAULT_IF_TITLE_PAGE,
    doc::NAME_IF_TOC_PAGE   : doc::DEFAULT_IF_TOC_PAGE,
    toc::NAME_TOC_DEPTH     : toc::DEFAULT_TOC_DEPTH,
    toc::NAME_TOC_FUN       : toc::DEFAULT_TOC_FUN,
    toc::NAME_TOC_TITLE_FUN : toc::DEFAULT_TOC_TITLE_FUN,
    sec::NAME_SEC_DEPTH     : sec::DEFAULT_SEC_DEPTH,
    sec::NAME_SEC_FUN_NAME  : sec::default_sec_fun_name(),
    sec::NAME_SEC_FUN       : sec::DEFAULT_SEC_FUN,
    title::NAME_TITLE_FUN   : title::DEFAULT_TITLE_FUN,
    func::NAME_PARAGRAPH_MARGIN_TOP    : func::DEFAULT_PARAGRAPH_MARGIN_TOP,
    func::NAME_PARAGRAPH_MARGIN_BOTTOM : func::DEFAULT_PARAGRAPH_MARGIN_BOTTOM,
    func::NAME_MIN_GAP_OF_LINES        : func::DEFAULT_MIN_GAP_OF_LINES,
    func::NAME_LEADING      : func::DEFAULT_LEADING,
  })
}

pub fn to_font_data_vec(vec: Vec<(&str, &str, &str, &str)>) -> Vec<Value> {
  let mut stack: Vec<Value> = vec![];
  for (tag, name, ratio, correction) in vec.iter() {
    let j = json!({
      "tag" : tag,
      "name" : name,
      "ratio" : ratio,
      "correction" : correction,
    });
    stack.push(j)
  }
  stack
}

pub fn make_command_vec(v: &Value) -> Vec<String> {
  let mut def_module_vec = vec![
    func::VAL_REGISTER_CROSS_REFERENCE_FUN.to_string(),
    func::VAL_GET_CROSS_REFERENCE_FUN.to_string(),
    func::VAL_REGISTER_LOCATION_FRAME_FUN.to_string(),
    command::DIRECT_REF_CMD.to_string(),
    command::DIRECT_REF_PAGE_CMD.to_string(),
    command::DIRECT_P_CMD.to_string(),
    command::DIRECT_PN_CMD.to_string(),
  ];
  let sec_depth = &v[sec::NAME_SEC_DEPTH]
    .as_u64()
    .unwrap_or(sec::DEFAULT_SEC_DEPTH);
  let sec_fun_list = json_vec_to_str_vec(
    &v[sec::NAME_SEC_FUN_NAME]
      .as_array()
      .unwrap_or(&vec![json!(null); sec::DEFAULT_SEC_DEPTH as usize]),
    None,
    Some(&sec::default_sec_fun_name()),
  );
  let mut sec_command_vec = sec::command_vec(sec_depth, sec_fun_list);
  let () = def_module_vec.append(&mut sec_command_vec);
  def_module_vec
}

pub fn json_vec_to_str_vec(
  j_vec: &Vec<Value>,
  default: Option<&str>,
  default_vec: Option<&Vec<&str>>,
) -> Vec<String> {
  let mut s_vec = vec![];
  let len = j_vec.len();
  for i in 0..len {
    let v = &j_vec[i];
    let s_op = v.as_str();
    match s_op {
      None => match default {
        None => match default_vec {
          None => {}
          Some(ve) => s_vec.push(format!("{}", ve[i])),
        },
        Some(s) => s_vec.push(format!("{}", s)),
      },
      Some(s) => s_vec.push(format!("{}", s)),
    }
  }
  s_vec
}

#[test]
fn check_json_vec_to_str_vec() {
  assert_eq!(
    json_vec_to_str_vec(
      &vec![json!(null); 3],
      None,
      Some(&vec!["hoge", "fuga", "piyo"])
    ),
    vec!["hoge".to_string(), "fuga".to_string(), "piyo".to_string()]
  );
}
