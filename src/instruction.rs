// pub mod opcode;
pub mod accessor;
pub mod flag;

use crate::streamer::proto;

use self::{accessor::InsnAccess, flag::Flags};

#[derive(Clone, Debug)]
pub struct Insn {
  b: u64,
  mnemonic: String,
  disasm: String,
  pc: u64,
  symbol: Option<String>,
  offset: Option<u64>,
  flags: u64,
}

impl Insn {
  pub fn new(msg: &proto::MtMsg) -> Self {
    Insn {
      b: msg.inst.binary as u64,
      mnemonic: msg.inst.mnemonic.clone(),
      disasm: msg.inst.disasm.clone(),
      pc: msg.inst.pc,
      symbol: msg.inst.symbol.clone(),
      offset: msg.inst.offset,
      flags: msg.inst.flags,
    }
  }

  pub fn pc(&self) -> u64 {
    self.pc
  }

  pub fn symbol(&self) -> Option<&String> {
    self.symbol.as_ref()
  }

  pub fn offset(&self) -> Option<u64> {
    self.offset
  }

  pub fn mnemonic(&self) -> &str {
    &self.mnemonic
  }

  pub fn disasm(&self) -> &str {
    &self.disasm
  }
  

  pub fn has_flag(&self, flag: Flags) -> bool {
    flag.contains_flag(self.flags)
  }

  pub fn is_load(&self) -> bool {
    self.has_flag(Flags::IsLoad)
  }

  pub fn is_store(&self) -> bool {
    self.has_flag(Flags::IsStore)
  }

  pub fn is_control(&self) -> bool {
    self.has_flag(Flags::IsControl)
  }

  // pub fn is_direct_control(&self) -> bool {
  //   self.has_flag(Flags::IsDirectControl)
  // }

  // pub fn is_indirect_control(&self) -> bool {
  //   self.has_flag(Flags::IsIndirectControl)
  // }

  pub fn is_call(&self) -> bool {
    self.has_flag(Flags::IsCall)
  }
  
  pub fn is_return(&self) -> bool {
    self.has_flag(Flags::IsReturn)
  }
}

impl InsnAccess for Insn {
  fn bits(&self) -> u64 {
    self.b
  }
}
