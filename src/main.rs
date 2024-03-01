#![allow(non_camel_case_types)]


#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate serde_derive;

pub mod errors;
pub mod streamer;
pub mod instruction;
pub mod graph;
pub mod il;
pub mod loader;

use log::info;


extern crate pretty_env_logger;


fn streamer_from_cfg() -> streamer::Streamer {
  let cfg_toml: toml::Value =
    toml::from_str(&std::fs::read_to_string("cfg.toml").expect("Failed to read cfg.toml"))
      .expect("Failed to parse cfg.toml");
  let micro_trace_path = cfg_toml["run"]["micro_trace_path"]
    .as_str()
    .unwrap_or("tmp/utrace.pb");

  info!("micro_trace_path: {}", micro_trace_path);

  streamer::Streamer::new(micro_trace_path, &["spmv"]).unwrap()
}

fn main() {
  pretty_env_logger::init();
  info!("Hello, world!");

  let cfg = loader::stream_to_cfg(&mut streamer_from_cfg()).unwrap();
  cfg.debug();
}

#[cfg(test)]
mod tests {
  use super::*;

  use toml::Value;

  #[test]
  fn test_streamer() {
    pretty_env_logger::init();

    let cfg_toml: Value =
      toml::from_str(&std::fs::read_to_string("cfg.toml").expect("Failed to read cfg.toml"))
        .expect("Failed to parse cfg.toml");
    let micro_trace_path = cfg_toml["run"]["micro_trace_path"]
      .as_str()
      .unwrap_or("tmp/utrace.pb");

    info!("micro_trace_path: {}", micro_trace_path);

    let streamer = streamer::Streamer::new(micro_trace_path, &[]).unwrap();

    info!("start reading trace file");

    let mut i = 0;
    for _ in streamer {
      i = i + 1;
      if i > 10 {
        break;
      }
    }
  }
}
