pub mod streamer;
use log::info;

use toml::Value;

extern crate pretty_env_logger;


fn main() {
  pretty_env_logger::init();
  info!("Hello, world!");


  let cfg_toml: Value =
    toml::from_str(&std::fs::read_to_string("cfg.toml").expect("Failed to read cfg.toml"))
      .expect("Failed to parse cfg.toml");
  let micro_trace_path = cfg_toml["run"]["micro_trace_path"]
    .as_str()
    .unwrap_or("tmp/utrace.pb");

  info!("micro_trace_path: {}", micro_trace_path);

  let streamer = streamer::Streamer::new(micro_trace_path).unwrap();

  info!("start reading trace file");

  let mut i = 0;
  for _ in streamer {
    i = i + 1;
    if i > 10 {
      break;
    }
  }
}
