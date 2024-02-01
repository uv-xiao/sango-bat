use core::fmt;

use crate::instruction::Insn;

use super::*;

#[derive(Clone, Debug)]
pub struct Node {
  belongs_to: block_t,
  index: insn_t,
  insn: Option<Insn>,
  pub srcs: Vec<insn_t>,
}

impl Node {
  pub fn new(belongs_to: block_t, index: insn_t, insn: Insn) -> Self {
    Node {
      belongs_to,
      index,
      insn: Some(insn),
      srcs: Vec::new(),
    }
  }

  pub fn new_phi(belongs_to: block_t, index: insn_t) -> Self {
    Node {
      belongs_to,
      index,
      insn: None,
      srcs: Vec::new(),
    }
  }

  pub fn index(&self) -> insn_t {
    self.index
  }

  pub fn insn(&self) -> Option<&Insn> {
    self.insn.as_ref()
  }

  pub fn is_phi(&self) -> bool {
    self.insn.is_none()
  }

  pub fn block(&self) -> block_t {
    self.belongs_to
  }

  pub fn set_srcs_from(&mut self, that: &mut Node) {
    self.srcs.append(&mut that.srcs);
  }

  pub fn srcs(&self) -> Vec<insn_t> {
    self.srcs.clone()
  }

  pub fn set_srcs(&mut self, srcs: Vec<insn_t>) {
    self.srcs = srcs;
  }
}

impl fmt::Display for Node {
  fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
    write!(f, "Node {}", self.index)?;
    if let Some(insn) = &self.insn {
      write!(
        f,
        " {}+{}: {}",
        insn.symbol().unwrap_or(&"global".to_string()),
        insn.offset().unwrap_or(0),
        insn.disasm()
      )?;
    } else {
      write!(
        f,
        " PHI"
      )?;
    }
    Ok(())
  }
}
