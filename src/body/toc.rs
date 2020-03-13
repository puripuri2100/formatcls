pub const NAME_TOC_DEPTH: &str = "toc-depth";
pub const NAME_TOC_FUN: &str = "toc-fun";
pub const NAME_TOC_TITLE_FUN: &str = "toc-title-fun";

pub const DEFAULT_TOC_DEPTH: u64 = 2;
pub const DEFAULT_TOC_FUN: &str = "make-toc-bb";
pub const DEFAULT_TOC_TITLE_FUN: &str = "make-toc-title-bb";

fn make_toc_type(depth: u64) -> String {
  let mut main_str = "type toc-cls =\n".to_string();
  for i in 1..(depth + 1) {
    let s = format!("  | Sec{} of string * inline-text * int list\n", i);
    main_str.push_str(&s);
  }
  main_str
}

fn make_bb_fun() -> String {
  format!(
    "
let make-dots-line ctx w =
  let-rec sub n ib =
    if n <= 0 then
      ib
    else
      ib ++ (sub (n - 1) ib)
  in
  let ib = read-inline ctx {{.}} ++ inline-skip 1pt in
  let wdot = get-natural-width ib in
  let n = (round (w /' wdot)) - 1 in
    inline-fil ++ (sub n ib)

let get-ib-width ib =
  get-natural-metrics ib
  |> (fun (w,_,_) -> w)

let make-toc-bb ctx text-width label count-lst i title =
  let it-page = embed-string (s-get-cross-reference-page label) in
  let ctx-1 =
    ctx
    |> set-cjk-font font-gothic
    |> set-latin-font font-sans
    |> set-font-size (main-font-size *' 1.2)
  in
  let ctx-2 =
    ctx
    |> set-font-size (main-font-size *' 1.1)
  in
    match i with
    | 1 ->
      let ib-num =
        count-lst
        |> List.map (fun n -> (if n <= 0 then ` ` else ((arabic n) ^ `. `#)))
        |> List.fold-left (^) ` `
        |> embed-string
        |> read-inline ctx-1
      in
      let ib-title = read-inline ctx-1 {{#title;}} in
      let ib-page = read-inline ctx-1 {{#it-page;}} in
      inline-skip 3pt ++ ib-num ++ ib-title ++ inline-skip 5pt ++
      make-dots-line ctx-1
        (text-width -'
          (get-ib-width ib-num) -'
          (get-ib-width ib-title) -'
          (get-ib-width ib-page) -'
          13pt
        ) ++ inline-skip 5pt ++ inline-fil ++ ib-page
      |> ib-link-to-location-frame label
    | 2 ->
      inline-skip ((main-font-size *' 1.2) *' 1.0) ++
      (count-lst
        |> List.map (fun n -> (if n <= 0 then ` ` else ((arabic n) ^ `. `#)))
        |> List.fold-left (^) ` `
        |> embed-string
        |> read-inline ctx-2
      ) ++
      read-inline ctx-2 {{#title;}} ++
      inline-fil ++
      read-inline ctx-2 {{#it-page;}}
      |> ib-link-to-location-frame label
    | _ ->
      inline-skip ((main-font-size *' 1.2) *' 2.0) ++
      (count-lst
        |> List.map (fun n -> (if n <= 0 then ` ` else ((arabic n) ^ `. `#)))
        |> List.fold-left (^) ` `
        |> embed-string
        |> read-inline ctx
      ) ++
      read-inline ctx {{#title;}} ++
      inline-fil ++
      read-inline ctx {{#it-page;}}
      |> ib-link-to-location-frame label\n"
  )
}

fn make_title_fun() -> String {
  format!(
    "
let make-toc-title-bb ctx main-bb =
  let title-ctx =
    ctx
    |> set-cjk-font font-gothic
    |> set-latin-font font-sans
    |> set-font-size 16pt
  in
  let title-ib = {{目次}} |> read-inline title-ctx in
  let title-bb = line-break true false title-ctx title-ib in
    title-bb +++ block-skip 5pt +++ main-bb
  "
  )
}

fn make_toc_fun(depth: u64, fun_name: &str) -> String {
  let mut main_str = "let make-toc ctx text-width t =\n  match t with\n".to_string();
  for i in 1..(depth + 1) {
    let s = format!(
      "    | Sec{} (label,title,lst) -> {} ctx text-width label lst {} title\n",
      i, fun_name, i
    );
    main_str.push_str(&s)
  }
  main_str
}

pub fn make_toc_str(depth: &u64, toc_fun: &str) -> String {
  format!(
    "
\n
% Begin Table of Contents ==============
{}\n
{}\n
{}\n
{}
% End Table of Contents ================  
\n",
    make_toc_type(*depth),
    make_bb_fun(),
    make_title_fun(),
    make_toc_fun(*depth, toc_fun)
  )
}

pub fn make_toc_fun_str(toc_title_fun: &str) -> String {
  format!(
    "
  let bb-toc-main =
    !toc-lst-ref
    |> List.reverse
    |> List.map (fun l -> l |> make-toc ctx-doc text-width |> line-break true true ctx-doc)
    |> List.fold-left (+++) block-nil
    |> {} ctx-doc
  in
  ",
    toc_title_fun
  )
}
