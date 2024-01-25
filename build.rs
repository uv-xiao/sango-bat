// build.rs
extern crate prost_build;
extern crate toml;

use std::{fs, path::Path};
use toml::Value;

fn main() {
  // Read the TOML file
  let toml_content = fs::read_to_string("cfg.toml").expect("Failed to read cfg.toml");
  let toml_value: Value = toml::from_str(&toml_content).expect("Failed to parse TOML");

  // Access configuration values
  let proto_file_path = toml_value["build"]["proto_file_path"]
    .as_str()
    .unwrap_or("src/mtmsg.proto");
  let proto_included_path = Path::new(proto_file_path)
    .parent()
    .unwrap()
    .to_str()
    .unwrap();

  prost_build::compile_protos(&[proto_file_path], &[proto_included_path]).unwrap();
}
