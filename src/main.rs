extern crate clap;

use clap::{App, Arg};
use std::fs::File;
use std::io::Write;

pub mod body;
pub mod config;

fn write_file(file_name: String, text: String) -> () {
  let mut file = File::create(file_name).unwrap();
  file.write_all(text.as_bytes()).unwrap();
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

  let package_data: Vec<&str> = vec!["annot", "option", "list", "math", "gr", "color", "vdecoset"];

  let _ = match (&mut config_name, &mut output_name) {
    (Some(c), Some(o)) => {
      let config_data = config::parse(c);
      let require_list = &config_data["require-list"];
      let import_list = &config_data["import-list"];
      let header = config::package(package_data, &require_list, &import_list);
      let module = body::module_name(&config_data);
      let (sig, body) = body::body(&config_data);
      let text = format!(
        "{}\nmodule {} : sig\n{}\nend = struct\n{}\nend",
        header, module, sig, body
      );
      write_file(o.to_string(), text);
    }
    (None, Some(_)) => {
      panic!("!!!");
    }
    (_, _) => (),
  };

  let _ = match default_name {
    None => {}
    Some(s) => write_file(s, body::default_json_setting()),
  };
}
