pub fn make_header_footer_fun() -> String {
  "

let make-header ctx n =
  block-nil

let make-footer ctx n =
  let ib-num = n |> arabic |> embed-string |> read-inline ctx in
  let main-ib = inline-fil ++ ib-num ++ inline-fil in
  let main-bb = line-break true true ctx main-ib in
    block-skip 0pt +++ main-bb

let empty ctx n = block-nil

  "
  .to_string()
}

pub fn make_register_cross_reference_fun() -> String {
  "

% 相互参照用のコマンド

  % 追加用関数
    % section番号等は、識別のために :num を付け、ページ番号は :page を付ける

    % unit型
    let u-register-cross-reference-num s-label s-title =
      register-cross-reference (s-label ^ `:num`) s-title


    % inline-boxes型
    let ib-register-cross-reference-page s-label s-title =
      hook-page-break (fun pbinfo _ -> register-cross-reference (s-label ^ `:page`) (arabic pbinfo#page-number))

".to_string()
}

pub const VAL_REGISTER_CROSS_REFERENCE_FUN: &str =
  "val u-register-cross-reference-num : string -> string -> unit
val ib-register-cross-reference-page : string -> string -> inline-boxes";

pub fn make_get_cross_reference_fun() -> String {
  "

  % 取得用関数 返り値はどちらもstring型
  let s-get-cross-reference-num s-label =
    match get-cross-reference (s-label ^ `:num`) with
      | None -> `?`
      | Some (label) -> label


  let s-get-cross-reference-page s-label =
    match get-cross-reference (s-label ^ `:page`) with
      | None -> `?`
      | Some (label) -> label

"
  .to_string()
}

pub const VAL_GET_CROSS_REFERENCE_FUN: &str = "val s-get-cross-reference-num : string -> string
val s-get-cross-reference-page : string -> string";

pub fn make_register_location_frame_fun() -> String {
  "

  % ハイパーリンクのキー登録とキー取得

  % 登録 string -> inline-boxes -> inline-boxes
  let ib-register-location-frame s-key ib =
    inline-frame-breakable (0pt, 0pt, 0pt, 0pt)
      (Annot.register-location-frame s-key) ib


  % 取得 string -> inline-boxes -> inline-boxes
  let ib-link-to-location-frame s-key ib =
    inline-frame-breakable (0pt, 0pt, 0pt, 0pt)
      (Annot.link-to-location-frame s-key None) ib

"
  .to_string()
}

pub const VAL_REGISTER_LOCATION_FRAME_FUN: &str =
  "val ib-register-location-frame : string -> inline-boxes -> inline-boxes
val ib-link-to-location-frame : string -> inline-boxes -> inline-boxes";

pub fn make_toc_and_outline_lst() -> String {
  "
  let-mutable toc-lst-ref <- []
  let-mutable fig-lst-ref <- []
  let-mutable table-lst-ref <- []
  let-mutable outline-lst-ref <- []
"
  .to_string()
}

pub const NAME_PARAGRAPH_MARGIN_TOP: &str = "paragraph-margin-top";
pub const NAME_PARAGRAPH_MARGIN_BOTTOM: &str = "paragraph-margin-bottom";
pub const NAME_LEADING: &str = "leading";
pub const NAME_MIN_GAP_OF_LINES: &str = "min-gap-of-lines";

pub const DEFAULT_PARAGRAPH_MARGIN_TOP: &str = "10pt";
pub const DEFAULT_PARAGRAPH_MARGIN_BOTTOM: &str = "10pt";
pub const DEFAULT_LEADING: &str = "17pt";
pub const DEFAULT_MIN_GAP_OF_LINES: &str = "5pt";

pub fn set_initial_context(
  paragraph_margin_top: &str,
  paragraph_margin_bottom: &str,
  leading: &str,
  min_gap_of_lines: &str,
) -> String {
  format!(
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
    |> set-paragraph-margin {} {}
    |> set-leading {}
    |> set-min-gap-of-lines {}

",
    paragraph_margin_top, paragraph_margin_bottom, leading, min_gap_of_lines
  )
}
