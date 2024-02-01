use crate::il::CFG;
use crate::errors::*;
use crate::streamer::Streamer;

pub fn stream_to_cfg(stream: &mut Streamer) -> Result<CFG> {
  let mut cfg = CFG::new();

  for msg in stream {
    cfg.parse_msg(msg);
  }
  
  Ok(cfg)
}