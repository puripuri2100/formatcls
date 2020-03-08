pub fn make_header() -> String {
  "

let make-header ctx n =
  block-nil

  "
  .to_string()
}

pub fn make_footer() -> String {
  "

let make-footer ctx n =
  let ib-num = n |> arabic |> embed-string |> read-inline ctx in
  let main-ib = inline-fil ++ ib-num ++ inline-fil in
  let main-bb = line-break true true ctx main-ib in
    block-skip 0pt +++ main-bb

"
  .to_string()
}
