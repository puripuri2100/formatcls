use serde_json::Value;

pub fn set_doc(doc_name: &str, config_type: &Option<&Vec<Value>>) -> String {
  let config_type_str = get_config_type_text(config_type);
  format!(
    "val {} : 'a -> block-text -> document
  constraint 'a :: (|
    title : inline-text;
    author : inline-text;
    {}
  |)\n",
    doc_name, config_type_str
  )
}

fn get_config_type_text(c: &Option<&Vec<Value>>) -> String {
  let st = match c {
    None => String::new(),
    Some(v) => {
      let mut s = String::new();
      let len = v.len();
      for i in 0..len {
        let j = &v[i];
        let name = j["name"].as_str();
        let ty = j["type"].as_str();
        let main_s = match (name, ty) {
          (Some("title"), _) => String::new(),
          (Some("author"), _) => String::new(),
          (Some(n), Some(t)) => format!("{} : {};", n, t),
          (_, _) => String::new(),
        };
        s.push_str(&main_s)
      }
      s
    }
  };
  st
}

pub fn make_val(name: &str, t: &str) -> String {
  format!("val {} : {}\n", name, t)
}

pub fn make_direct(name: &str, t: &str) -> String {
  format!("direct {} : {}\n", name, t)
}

pub const DEF_P_CMD: &str = "
let-block ctx +p inner =
  let indent ctx = inline-skip (get-font-size ctx) in
  let ib-indent = indent ctx in
  let ib-inner = read-inline ctx inner in
    line-break true true ctx (ib-indent ++ ib-inner ++ inline-fil)
";

pub const DIRECT_P_CMD: &str = "direct +p : [inline-text] block-cmd\n";

pub const DIRECT_PN_CMD: &str = "direct +pn : [inline-text] block-cmd\n";

pub const DEF_PN_CMD: &str = "
let-block ctx +pn inner =
  let ib-inner = read-inline ctx inner in
    line-break true true ctx (ib-inner ++ inline-fil)
";
