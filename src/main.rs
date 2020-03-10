extern crate clap;

use clap::{App, Arg};
use serde_json::json;
use serde_json::Value;
use std::fs;
use std::fs::File;
use std::io::Write;

pub mod body;
pub mod default;
pub mod header;
pub mod module;

fn write_file(file_name: String, text: String) -> () {
  let mut file = File::create(file_name).unwrap();
  file.write_all(text.as_bytes()).unwrap();
}

fn parse(path: &str) -> Value {
  let data = fs::read_to_string(path).unwrap();
  serde_json::from_str(&data).unwrap()
}

fn main() {
  let app = App::new("clapex")
    .version("0.0.1")
    .arg(
      Arg::with_name("output")
        .help("Specify output file")
        .short("o")
        .long("output")
        .takes_value(true),
    )
    .arg(
      Arg::with_name("config")
        .help("Specify config file")
        .short("c")
        .long("config")
        .takes_value(true),
    )
    .arg(
      Arg::with_name("default")
        .help("Output default setting file")
        .short("d")
        .long("default")
        .takes_value(true),
    );

  let matches = app.get_matches();

  let mut output_name: Option<String> = None;
  let mut config_name: Option<String> = None;
  let mut default_name: Option<String> = None;

  if let Some(output) = matches.value_of("output") {
    output_name = Some(output.to_string());
    print!("Value for output: {}\n", output);
  }

  if let Some(config) = matches.value_of("config") {
    config_name = Some(config.to_string());
    print!("Value for config: {}\n", config);
  }

  if let Some(default) = matches.value_of("default") {
    default_name = Some(default.to_string());
    print!("Output for default: {}\n", default);
  }

  let package_data = vec![
    "annot".to_string(),
    "option".to_string(),
    "list".to_string(),
    "math".to_string(),
    "gr".to_string(),
    "color".to_string(),
    "vdecoset".to_string(),
  ];

  let _ = match (&mut config_name, &mut output_name) {
    (Some(c), Some(o)) => {
      let config_data = parse(c);
      let json_null_vec = vec![json!(null)];
      let require_list = config_data[header::NAME_REQUIRE_PACKAGE]
        .as_array()
        .unwrap_or(&json_null_vec);
      let import_list = config_data[header::NAME_IMPORT_PACKAGE]
        .as_array()
        .unwrap_or(&json_null_vec);
      let header = header::package(
        package_data,
        json_vec_to_str_vec(require_list, header::DEFAULT_REQUIRE_PACKAGE_VEC_STR),
        json_vec_to_str_vec(import_list, header::DEFAULT_IMPORT_PACKAGE_VEC_STR),
      );
      let doc_fun_name = &config_data[module::NAME_DOC_FUNCTION_NAME]
        .as_str()
        .unwrap_or(module::DEFAULT_DOC_FUNCTION_NAME);
      let module_name = &config_data[module::NAME_MODULE_NAME]
        .as_str()
        .unwrap_or(module::DEFAULT_MODULE_NAME);
      let page_width = &config_data[body::NAME_PAGE_WIDTH]
        .as_str()
        .unwrap_or(body::DEFAULT_PAGE_WIDTH);
      let page_height = &config_data[body::NAME_PAGE_HEIGHT]
        .as_str()
        .unwrap_or(body::DEFAULT_PAGE_HEIGHT);
      let page_size = &config_data[body::NAME_PAGE_SIZE].as_str();
      let page_size_str = body::set_page_size(page_size, page_width, page_height);
      let command_vec = body::make_command_vec();
      let sig = module::make_sig(doc_fun_name, command_vec);
      let body = body::body(&config_data, doc_fun_name, page_size_str);
      let text = format!(
        "{}\nmodule {} : sig\n{}\nend = struct\n{}\nend\n let {} = {}.{}\n",
        header, module_name, sig, body, doc_fun_name, module_name, doc_fun_name
      );
      write_file(o.to_string(), text);
    }
    (None, Some(_)) => {
      panic!("!!!");
    }
    (_, _) => (),
  };

  let default_json = default::merge_default_json(
    header::default_json(),
    module::default_json(),
    body::default_json(),
  );

  let _ = match default_name {
    None => {}
    Some(s) => write_file(
      s,
      serde_json::to_string_pretty(&default_json).unwrap_or_default(),
    ),
  };
}

fn json_vec_to_str_vec(j_vec: &Vec<Value>, default: Option<&str>) -> Vec<String> {
  let mut s_vec = vec![];
  let len = j_vec.len();
  for i in 0..len {
    let v = &j_vec[i];
    let s_op = v.as_str();
    match s_op {
      None => match default {
        None => {}
        Some(s) => s_vec.push(format!("{}", s)),
      },
      Some(s) => s_vec.push(format!("{}", s)),
    }
  }
  s_vec
}
