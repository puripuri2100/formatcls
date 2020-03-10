pub const DEFAULT_FONT_SIZE: &str = "12pt";
pub const DEFAULT_PAGE_WIDTH: &str = "210mm";
pub const DEFAULT_PAGE_HEIGHT: &str = "297mm";
pub const DEFAULT_PAGE_SIZE: &str = "a4";
pub const DEFAULT_TOP_SPACE: &str = "20pt";
pub const DEFAULT_BOTTOM_SPACE: &str = "20pt";
pub const DEFAULT_LEFT_SPACE: &str = "20pt";
pub const DEFAULT_RIGHT_SPACE: &str = "20pt";

pub fn make_let(n: &str, v: &str) -> String {
  format!("let {} = {}\n", n, v)
}

pub fn set_page_size(p: &Option<&str>, w: &str, h: &str) -> String {
  match p {
    Some("a0") => make_page_size_let("841mm", "1189mm"),
    Some("a1") => make_page_size_let("594mm", "841mm"),
    Some("a2") => make_page_size_let("420mm", "594mm"),
    Some("a3") => make_page_size_let("297mm", "420mm"),
    Some("a4") => make_page_size_let("210mm", "297mm"),
    Some("a5") => make_page_size_let("148mm", "210mm"),
    Some("a6") => make_page_size_let("105mm", "148mm"),
    Some("b0") => make_page_size_let("1000mm", "1414mm"),
    Some("b1") => make_page_size_let("707mm", "1000mm"),
    Some("b2") => make_page_size_let("500mm", "707mm"),
    Some("b3") => make_page_size_let("353mm", "500mm"),
    Some("b4") => make_page_size_let("250mm", "353mm"),
    Some("b5") => make_page_size_let("176mm", "250mm"),
    Some("b6") => make_page_size_let("125mm", "176mm"),
    Some("c0") => make_page_size_let("917mm", "1297mm"),
    Some("c1") => make_page_size_let("648mm", "917mm"),
    Some("c2") => make_page_size_let("458mm", "648mm"),
    Some("c3") => make_page_size_let("324mm", "458mm"),
    Some("c4") => make_page_size_let("229mm", "324mm"),
    Some("c5") => make_page_size_let("162mm", "2290mm"),
    Some("c6") => make_page_size_let("114mm", "162mm"),
    Some("b0j") => make_page_size_let("1030mm", "1456mm"),
    Some("b1j") => make_page_size_let("728mm", "1030mm"),
    Some("b2j") => make_page_size_let("515mm", "78mm"),
    Some("b3j") => make_page_size_let("364mm", "515mm"),
    Some("b4j") => make_page_size_let("257mm", "362mm"),
    Some("b5j") => make_page_size_let("182mm", "257mm"),
    Some("b6j") => make_page_size_let("128mm", "182mm"),
    Some("ansia") => make_page_size_let("8.5inch", "11inch"),
    Some("ansib") => make_page_size_let("11inch", "17inch"),
    Some("ansic") => make_page_size_let("17inch", "22inch"),
    Some("ansid") => make_page_size_let("22inch", "34inch"),
    Some("ansie") => make_page_size_let("34inch", "44inch"),
    Some("letter") => make_page_size_let("8.5inch", "11inch"),
    Some("legal") => make_page_size_let("8.5inch", "14inch"),
    Some("executive") => make_page_size_let("7.25inch", "10.5inch"),
    Some("screen") => make_page_size_let("225mm", "180mm"),
    None => make_page_size_let(w, h),
    _ => make_page_size_let(w, h),
  }
}

fn make_page_size_let(w: &str, h: &str) -> String {
  format!(
    "{}{}\n",
    make_let("page-width", w),
    make_let("page-height", h)
  )
}

pub fn set_main_font(
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

pub fn set_initial_context() -> String {
  "
% 基本となるctxの設定
let ctx-get-initial-context l-width =
  get-initial-context l-width (command \\math)
    |> set-font-size main-font-size
    |> set-dominant-wide-script Kana
    |> set-language Kana Japanese
    |> set-language HanIdeographic Japanese
    |> set-cjk-font main-font-cjk
    |> set-dominant-narrow-script Latin
    |> set-language Latin English
    |> set-latin-font main-font-latin
    |> set-hyphen-penalty 100
    |> set-math-font `lmodern`
    |> set-manual-rising 0pt % 文字の上下の補正値
    |> set-text-color Color.black

"
  .to_string()
}

#[test]
fn check_set_page_size() {
  assert_eq!(
    set_page_size(&Some("a4"), "240mm", "100mm"),
    "let page-width = 210mm\nlet page-height = 297mm\n\n".to_string()
  );
  assert_eq!(
    set_page_size(&None, "240mm", "100mm"),
    "let page-width = 240mm\nlet page-height = 100mm\n\n".to_string()
  );
}
