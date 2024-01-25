use std::{
  collections::VecDeque,
  fs::File,
  io::{self, BufReader, Read},
  path::Path,
};

use log::info;
use prost::Message;

use self::proto::MtMsg;

pub mod proto {
  include!(concat!(env!("OUT_DIR"), "/mtproto.rs"));
}

pub struct Streamer {
  reader: BufReader<File>,
  buf: VecDeque<u8>,
  end: bool,
}

impl Streamer {
  pub fn new<P: AsRef<Path>>(file_path: P) -> io::Result<Self> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let buf = VecDeque::new();
    Ok(Self {
      reader,
      buf,
      end: false,
    })
  }

  pub fn next_msg(&mut self) -> Option<MtMsg> {
    if self.end {
      return None;
    }
    if self.buf.len() < 128 {
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
      let decoded_msg = proto::MtMsg::decode_length_delimited(&mut self.buf);
      match decoded_msg {
        Ok(msg) => {
          info!("[len={:?}] message decoded: {:?}", msg.encoded_len(), msg);
          Some(msg)
        }
        Err(err) => {
          info!("message decode error: {:?}", err);
          self.end = true;
          None
        }
      }
    }
  }
}

impl Iterator for Streamer {
  type Item = MtMsg;

  fn next(&mut self) -> Option<Self::Item> {
    self.next_msg()
  }
}
