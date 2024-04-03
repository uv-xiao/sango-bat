use std::{
  collections::VecDeque,
  fs::File,
  io::{self, BufReader, Read},
  path::Path,
};

use log::{info, error};
use prost::Message;

use self::proto::UtMsg;

pub mod proto {
  include!(concat!(env!("OUT_DIR"), "/utproto.rs"));
}

pub struct Streamer {
  reader: BufReader<File>,
  buf: VecDeque<u8>,
  end: bool,
  symbol_of_interest: Vec<String>,
}

impl Streamer {
  pub fn new<P: AsRef<Path>>(file_path: P, symbol_of_interest: &[&str]) -> io::Result<Self> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let buf = VecDeque::new();
    Ok(Self {
      reader,
      buf,
      end: false,
      symbol_of_interest: symbol_of_interest.iter().map(|s| s.to_string()).collect(),
    })
  }

  pub fn next_msg(&mut self) -> Option<UtMsg> {
    if self.end {
      return None;
    }
    if self.buf.len() < 256 {
      let mut local_buf: Vec<u8> = vec![0; 1024];
      let read_size = self.reader.read(&mut local_buf).unwrap_or(0);
      self.buf.extend(local_buf.iter().take(read_size));
      info!("read {} bytes from protobuf file", read_size);
    }
    if self.buf.len() < 2 {
      self.end = true;
      None
    } else {
      // info!("left {} bytes in buffer", self.buf.len());
      let size = prost::encoding::decode_varint(&mut self.buf).unwrap_or(0);
      info!("message size: {}", size);
      let mut to_decode_buf: VecDeque<_> = self.buf.drain(..size as usize).collect();

      let decoded_msg = UtMsg::decode(&mut to_decode_buf);

      match decoded_msg {
        Ok(msg) => {
          info!("[len={:?}] message decoded: {:?}", msg.encoded_len(), msg);
          Some(msg)
        }
        Err(err) => {
          error!("message decode error: {:?}", err);
          self.end = true;
          None
        }
      }
    }
  }
}

impl Iterator for Streamer {
  type Item = UtMsg;

  fn next(&mut self) -> Option<Self::Item> {
    loop {
      match self.next_msg() {
        Some(msg) => {
          if self.symbol_of_interest.is_empty() {
            return Some(msg);
          } else {
            for symbol in &self.symbol_of_interest {
              if msg.inst.symbol() == *symbol {
                return Some(msg);
              }
            }
          }
        }
        None => {
          return None;
        }
      }
    }
  }
}
