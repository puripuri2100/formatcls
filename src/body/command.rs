pub fn make_val(name: &str, t: &str) -> String {
  format!("val {} : {}\n", name, t)
}

pub fn make_direct(name: &str, t: &str) -> String {
  format!("direct {} : {}\n", name, t)
}

pub const DIRECT_P_CMD: &str = "direct +p : [inline-text] block-cmd";
pub const DEF_P_CMD: &str = "
let-block ctx +p inner =
  let indent ctx = inline-skip (get-font-size ctx) in
  let ib-indent = indent ctx in
  let ib-inner = read-inline ctx inner in
    line-break true true ctx (ib-indent ++ ib-inner ++ inline-fil)
";

pub const DIRECT_PN_CMD: &str = "direct +pn : [inline-text] block-cmd";
pub const DEF_PN_CMD: &str = "
let-block ctx +pn inner =
  let ib-inner = read-inline ctx inner in
    line-break true true ctx (ib-inner ++ inline-fil)
";

pub const DIRECT_REF_CMD: &str = "direct \\ref : [string] inline-cmd";
pub const DEF_REF_CMD: &str = "
  % \\ref 定義
  let-inline ctx \\ref s-key =
    ib-link-to-location-frame s-key
      (read-inline ctx (embed-string (s-get-cross-reference-num s-key)))
";

pub const DIRECT_REF_PAGE_CMD: &str = "direct \\ref-page : [string] inline-cmd";
pub const DEF_REF_PAGE_CMD: &str = "
  % \\ref-page 定義
  let-inline ctx \\ref-page s-key =
    ib-link-to-location-frame s-key
      (read-inline ctx (embed-string (s-get-cross-reference-page s-key)))
";
