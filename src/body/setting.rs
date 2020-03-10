pub const DEFAULT_FONT_SIZE: &str = "12pt";
pub const DEFAULT_TOP_SPACE: &str = "20pt";
pub const DEFAULT_BOTTOM_SPACE: &str = "20pt";
pub const DEFAULT_LEFT_SPACE: &str = "20pt";
pub const DEFAULT_RIGHT_SPACE: &str = "20pt";

pub fn make_let(n: &str, v: &str) -> String {
  format!("let {} = {}\n", n, v)
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
