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

const PAGE_PARTS_F: &str = "

  % ヘッダーとフッター
  let pagepartsf pbinfo =
    let page-num = pbinfo#page-number in
    let header-origin = (left-space, 0pt) in
    let footer-origin = (left-space, page-height -' bottom-space) in
    (|
      header-content = make-header ctx-doc page-num;
      header-origin = header-origin;
      footer-content = make-footer ctx-doc page-num;
      footer-origin = footer-origin;
    |)
  in

";

const LET_DOC_MAIN: &str = "
  % メイン
  let doc-main = page-break page pagecontf pagepartsf bb-main in
";

const OUTLINE: &str = "  let () = register-outline (List.reverse !outline-ref) in\n";

const DOC: &str = "    doc-main\n";

pub fn make_document_function(function_name: &str) -> String {
  let v = vec![
    CTX_DOC.to_string(),
    BB_MAIN.to_string(),
    PAGE.to_string(),
    PAGE_CONT_F.to_string(),
    PAGE_PARTS_F.to_string(),
    LET_DOC_MAIN.to_string(),
    //    OUTLINE.to_string(),
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
