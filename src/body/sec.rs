pub const NAME_SEC_DEPTH: &str = "sec-depth";
pub const NAME_SEC_FUN_NAME: &str = "sec-name-list";
pub const NAME_SEC_FUN: &str = "sec-fun";

pub const DEFAULT_SEC_DEPTH: u64 = 3;
pub const DEFAULT_SEC_FUN: &str = "make-sec-bb";

pub fn default_sec_fun_name() -> Vec<&'static str> {
  vec!["section", "subsection", "subsubsection"]
}

fn set_sec_fun_vec(depth: usize, sec_fun_list: &Vec<String>) -> Vec<String> {
  let mut stack = vec![];
  let len = sec_fun_list.len();
  for i in 1..(depth + 1) {
    let s = if i <= len {
      format!("{}", &sec_fun_list[(i - 1)])
    } else {
      format!("sec{}", i)
    };
    stack.push(s)
  }
  stack
}

fn make_sec_num_ref(sec_depth: &u64) -> String {
  let mut main_str = String::new();
  for i in 1..(sec_depth + 1) {
    let s = format!("let-mutable sec{}-count <- 0\n", i);
    main_str.push_str(&s)
  }
  main_str
}

fn add_toc_lst (i:usize, toc_depth_64:&u64) -> String {
  let toc_depth = *toc_depth_64 as usize;
  if i > toc_depth {
    format!("()")
  }
  else{
  format!("toc-lst-ref <- (Sec{}(label, title, lst)) :: !toc-lst-ref",i)
  }
}

fn make_cout(n: usize, depth_64: &u64) -> String {
  let depth: usize = *depth_64 as usize;
  let mut main_str = format!("let () = sec{}-count <- (!sec{}-count + 1) in\n", n, n);
  for i in (n + 1)..(depth + 1) {
    let s = format!("let () = sec{}-count <- 0 in\n", i);
    main_str.push_str(&s)
  }
  main_str
}

fn make_num_lst(n: usize) -> String {
  let mut main_str = String::new();
  for i in 1..(n + 1) {
    let s = format!("!sec{}-count; ", i);
    main_str.push_str(&s)
  }
  format!("[{}]", main_str)
}

fn make_sec_bb_fun() -> String {
  format!(
    "
let make-sec-bb ctx label count-lst i title outline-lst outline-title-opt main =
  let sec-ctx =
    match i with
    | 1 -> 
      ctx
      |> set-cjk-font font-gothic
      |> set-latin-font font-sans
      |> set-font-size (main-font-size *' 1.5)
    | 2 -> 
      ctx
      |> set-cjk-font font-gothic
      |> set-latin-font font-sans
      |> set-font-size (main-font-size *' 1.3)
    | _ ->
      ctx
      |> set-cjk-font font-gothic
      |> set-latin-font font-sans
      |> set-font-size (main-font-size *' 1.2)
  in
  let s-num =
    count-lst
    |> List.map (fun n -> (if n <= 0 then ` ` else ((arabic n) ^ `. `#)))
    |> List.fold-left (^) ` `
  in
  let ib-title =
    ib-register-location-frame label (read-inline sec-ctx title)
  in
  let s-title = extract-string ib-title in
  let ib-title-link = ib-register-cross-reference-page label s-title in
  let () = u-register-cross-reference-num label s-num in
  let () =
    match i with
    | 1 -> (outline-lst <- (i, s-num ^ s-title, label, true) :: !outline-lst)
    | 2 -> (outline-lst <- (i, s-num ^ s-title, label, true) :: !outline-lst)
    | _ -> (outline-lst <- (i, s-num ^ s-title, label, false) :: !outline-lst)
  in
  let ib-num = read-inline sec-ctx (embed-string s-num) in

  let bb-title =
    line-break true false sec-ctx (ib-num ++ ib-title ++ ib-title-link)
  in
  let bb-inner = read-block ctx main in
    bb-title +++ bb-inner
  \n"
  )
}

fn make_sec_cmd(sec_fun_name: &Vec<String>, sec_fun: &str, depth: &u64, toc_depth: &u64) -> String {
  let mut main_str = String::new();
  let len = sec_fun_name.len();
  for i in 1..(len+1) {
    let s = format!(
      "
let-block ctx +{} ?:labelopt ?:outline-title-opt title inner =
  {}
  let lst =
    {}
  in
  let join i s1 s2 =
    if i <= 0 then
      s2
    else
      s1 ^ `-` ^ s2
  in
  let label =
    match labelopt with
    | None        -> lst |> List.map arabic |> List.fold-lefti join ` `
    | Some(label) -> label
  in
  let () = {} in
    {} ctx label lst {} title outline-lst-ref outline-title-opt inner\n\n",
      sec_fun_name[(i - 1)],
      make_cout(i, depth),
      make_num_lst(i),
      add_toc_lst(i,toc_depth),
      sec_fun,
      i
    );
    main_str.push_str(&s)
  }
  main_str
}

pub fn make_sec_str(
  sec_depth: &u64,
  toc_depth: &u64,
  sec_fun_list: Vec<String>,
  sec_fun: &str,
) -> String {
  let sec_vec = set_sec_fun_vec(*sec_depth as usize, &sec_fun_list);
  format!(
    "{}\n{}\n{}",
    make_sec_num_ref(sec_depth),
    make_sec_bb_fun(),
    make_sec_cmd(&sec_vec, sec_fun, sec_depth, toc_depth)
  )
}

pub fn command_vec(sec_depth: &u64, sec_fun_list: Vec<String>) -> Vec<String> {
  let sec_vec = set_sec_fun_vec(*sec_depth as usize, &sec_fun_list);
  let mut stack = vec![];
  for v in sec_vec.iter() {
    let s = format!(
      "direct +{} : [string?; inline-text?; inline-text; block-text] block-cmd",
      v
    );
    stack.push(s)
  }
  stack
}

#[test]
fn check_set_sec_fun_vec() {
  assert_eq!(
    set_sec_fun_vec(
      3,
      &vec![
        "hoge1".to_string(),
        "hoge2".to_string(),
        "hoge3".to_string()
      ]
    ),
    vec![
      "hoge1".to_string(),
      "hoge2".to_string(),
      "hoge3".to_string()
    ]
  );
  assert_eq!(
    set_sec_fun_vec(
      4,
      &vec![
        "hoge1".to_string(),
        "hoge2".to_string(),
        "hoge3".to_string()
      ]
    ),
    vec![
      "hoge1".to_string(),
      "hoge2".to_string(),
      "hoge3".to_string(),
      "sec4".to_string()
    ]
  );
  assert_eq!(
    set_sec_fun_vec(
      2,
      &vec![
        "hoge1".to_string(),
        "hoge2".to_string(),
        "hoge3".to_string()
      ]
    ),
    vec!["hoge1".to_string(), "hoge2".to_string()]
  );
}
