pub const NAME_IF_TITLE_PAGE: &str = "title-page";
pub const NAME_IF_TOC_PAGE: &str = "toc-page";

pub const DEFAULT_IF_TITLE_PAGE: bool = false;
pub const DEFAULT_IF_TOC_PAGE: bool = false;

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

fn if_clear_page(b: bool) -> &'static str {
  if b {
    "clear-page"
  } else {
    "block-nil"
  }
}

fn let_doc_main(if_title_page: &bool, if_toc_page: &bool) -> String {
  format!("
  % メイン
  let doc-main = page-break page pagecontf pagepartsf (bb-title +++ {} +++ bb-toc-main +++ {} +++ bb-main) in
",
  if_clear_page(*if_title_page),
  if_clear_page(*if_toc_page)
)
}

const OUTLINE: &str = "  let () = register-outline (List.reverse !outline-lst-ref) in\n";

const DOC: &str = "    doc-main\n";

pub const NAME_HEADER_FUN: &str = "header-fun";
pub const NAME_FOOTER_FUN: &str = "footer-fun";

pub const DEFAULT_HEADER_FUN: &str = "make-header";
pub const DEFAULT_FOOTER_FUN: &str = "make-footer";

pub fn make_document_function(
  function_name: &str,
  title_str: String,
  toc_fun: String,
  header_fun: &str,
  footer_fun: &str,
  if_title_page: &bool,
  if_toc_page: &bool,
) -> String {
  let v = vec![
    CTX_DOC.to_string(),
    BB_MAIN.to_string(),
    title_str,
    toc_fun,
    PAGE.to_string(),
    PAGE_CONT_F.to_string(),
    page_parts_f(header_fun, footer_fun),
    let_doc_main(if_title_page, if_toc_page),
    OUTLINE.to_string(),
    DOC.to_string(),
  ];
  let main_str = v.concat();

  format!("let {} config bt ={}", function_name, main_str)
}
