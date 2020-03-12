pub const NAME_TITLE_FUN: &str = "title-fun";

pub const DEFAULT_TITLE_FUN: &str = "make-title-bb";

pub fn title_bb_fun() -> String {
  format!(
    "
let {} ctx title author other =
  let title-ctx =
    ctx
    |> set-cjk-font font-gothic
    |> set-latin-font font-bold
    |> set-font-size 20pt
  in
  let author-ctx =
    ctx
    |> set-cjk-font font-gothic
    |> set-latin-font font-bold
    |> set-font-size 20pt
  in
  let title-ib = read-inline title-ctx title in
  let author-ib = read-inline author-ctx author in
  let title-bb = line-break true false title-ctx title-ib in
  let author-bb = line-break false true author-ctx author-ib in
  let main-bb = title-bb +++ author-bb in
    main-bb\n",
    DEFAULT_TITLE_FUN
  )
}

pub fn make_title_str(fun: &str) -> String {
  format!(
    "
  let bb-title =
    {} ctx-doc config#title config#author config#other
  in
  ",
    fun
  )
}
