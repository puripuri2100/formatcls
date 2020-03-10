const CTX_DOC: &str = "
  % ctx設定
  let ctx-doc = ctx-get-initial-context text-width in\n
  ";

const PAGE: &str = "
  % ページサイズ
  let page = UserDefinedPaper(page-width, page-height) in
";

const BB_MAIN: &str = "
  %メイン
  let bb-main = read-block ctx-doc bt in
";

const PAGE_CONT_F: &str = "
  % コンテンツを表示させる部分の指定
  let pagecontf pbinfo =
    (|
      text-height = text-height;
      text-origin = (left-space, top-space);
    |)
  in
";

pub fn page_parts_f(header_fun: &str, footer_fun: &str) -> String {
  format!(
    "
  % ヘッダーとフッター
  let pagepartsf pbinfo =
    let page-num = pbinfo#page-number in
    let header-origin = (left-space, 0pt) in
    let footer-origin = (left-space, page-height -' bottom-space) in
    (|
      header-content = {} ctx-doc page-num;
      header-origin = header-origin;
      footer-content = {} ctx-doc page-num;
      footer-origin = footer-origin;
    |)
  in
",
    header_fun, footer_fun
  )
}

const LET_DOC_MAIN: &str = "
  % メイン
  let doc-main = page-break page pagecontf pagepartsf (bb-toc-main +++ clear-page +++ bb-main) in
";

const OUTLINE: &str = "  let () = register-outline (List.reverse !outline-lst-ref) in\n";

const DOC: &str = "    doc-main\n";

pub const NAME_HEADER_FUN: &str = "header-fun";
pub const NAME_FOOTER_FUN: &str = "footer-fun";

pub const DEFAULT_HEADER_FUN: &str = "make-header";
pub const DEFAULT_FOOTER_FUN: &str = "make-footer";

pub fn make_document_function(
  function_name: &str,
  toc_fun: String,
  header_fun: &str,
  footer_fun: &str,
) -> String {
  let v = vec![
    CTX_DOC.to_string(),
    BB_MAIN.to_string(),
    toc_fun,
    PAGE.to_string(),
    PAGE_CONT_F.to_string(),
    page_parts_f(header_fun, footer_fun),
    LET_DOC_MAIN.to_string(),
    OUTLINE.to_string(),
    DOC.to_string(),
  ];
  let main_str = vec_to_str(&v);

  format!("let {} config bt ={}", function_name, main_str)
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
