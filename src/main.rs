pub mod streamer;
use log::info;
use prost::Message;
use std::{fs::File, io::BufWriter, io::Write};
use streamer::proto::{self, mt_msg::Timestamp};

use toml::Value;

extern crate pretty_env_logger;

fn write_trace(file_path: &str) {
  info!("write trace to {}", file_path);
  let file = File::create(file_path).unwrap();
  let messages = vec![
    proto::MtMsg {
      inst: proto::mt_msg::Inst {
        binary: vec![0 as u8, 0 as u8, 0 as u8, 0 as u8],
        pc: 1,
        seq_num: 1,
        op_class: "IntALU".to_string(),
      },
      mem_access: None,
      reg_access: None,
      timestamp: Timestamp { commit: 1 },
      symptom: None,
    },
    proto::MtMsg {
      inst: proto::mt_msg::Inst {
        binary: vec![0 as u8, 0 as u8, 0 as u8, 0 as u8],
        pc: 2,
        seq_num: 2,
        op_class: "IntALU".to_string(),
      },
      mem_access: None,
      reg_access: None,
      timestamp: Timestamp { commit: 3 },
      symptom: None,
    },
  ];

  let mut writer = BufWriter::new(&file);

  for msg in messages {
    let buf = msg.encode_length_delimited_to_vec();
    writer.write_all(&buf).unwrap();
  }
}

fn main() {
  pretty_env_logger::init();
  info!("Hello, world!");


  let cfg_toml: Value =
    toml::from_str(&std::fs::read_to_string("cfg.toml").expect("Failed to read cfg.toml"))
      .expect("Failed to parse cfg.toml");
  let micro_trace_path = cfg_toml["run"]["micro_trace_path"]
    .as_str()
    .unwrap_or("tmp/utrace.pb");
  let generate_trace = cfg_toml["run"]["generate_trace"]
    .as_bool()
    .unwrap_or(false);

  if generate_trace {
    write_trace(micro_trace_path);
  }

  info!("micro_trace_path: {}", micro_trace_path);

  let streamer = streamer::Streamer::new(micro_trace_path).unwrap();

  info!("start reading trace file");

  let mut i = 0;
  for _ in streamer {
    // println!("{:?}", msg);
    i = i + 1;
    if i > 10 {
      break;
    }
  }
}
