

pub fn insn_length(x: u64) -> i32 {
  match (x & 0x03, x & 0x1f, x & 0x3f) {
    (0..=2, _, _) => 2,
    (_, 0..=30, _) => 4,
    (_, _, 0..=62) => 6,
    _ => 8,
  }
}

pub trait InsnAccess {
  fn bits(&self) -> u64;
  fn x(b: u64, lo: u32, len: u32) -> u64 {
    (b >> lo) & ((1u64 << len) - 1)
  }
  fn xs(b: u64, lo: u32, len: u32) -> u64 {
    (((b as i64) << (64 - lo - len)) >> (64 - len))
      .try_into()
      .unwrap()
  }
  fn imm_sign(b: u64) -> u64 {
    Self::xs(b, 31, 1)
  }

  fn length(&self) -> i32 {
    insn_length(self.bits())
  }

  fn i_imm(&self) -> i64 {
    Self::xs(self.bits(), 20, 12).try_into().unwrap()
  }

  fn shamt(&self) -> i64 {
    Self::x(self.bits(), 20, 6).try_into().unwrap()
  }

  fn s_imm(&self) -> i64 {
    (Self::x(self.bits(), 7, 5) + (Self::xs(self.bits(), 25, 7) << 5))
      .try_into()
      .unwrap()
  }

  fn sb_imm(&self) -> i64 {
    ((Self::x(self.bits(), 8, 4) << 1)
      + (Self::x(self.bits(), 25, 6) << 5)
      + (Self::x(self.bits(), 7, 1) << 11)
      + (Self::imm_sign(self.bits()) << 12))
      .try_into()
      .unwrap()
  }

  fn u_imm(&self) -> i64 {
    (Self::xs(self.bits(), 12, 20) << 12).try_into().unwrap()
  }

  fn uj_imm(&self) -> i64 {
    ((Self::x(self.bits(), 21, 10) << 1)
      + (Self::x(self.bits(), 20, 1) << 11)
      + (Self::x(self.bits(), 12, 8) << 12)
      + (Self::imm_sign(self.bits()) << 20))
      .try_into()
      .unwrap()
  }

  fn rd(&self) -> u64 {
    Self::x(self.bits(), 7, 5)
  }

  fn rs1(&self) -> u64 {
    Self::x(self.bits(), 15, 5)
  }

  fn rs2(&self) -> u64 {
    Self::x(self.bits(), 20, 5)
  }

  fn rs3(&self) -> u64 {
    Self::x(self.bits(), 27, 5)
  }

  fn rm(&self) -> u64 {
    Self::x(self.bits(), 12, 3)
  }

  fn csr(&self) -> u64 {
    Self::x(self.bits(), 20, 12)
  }

  fn iorw(&self) -> u64 {
    Self::x(self.bits(), 20, 8)
  }

  fn bs(&self) -> u64 {
    Self::x(self.bits(), 30, 2)
  }

  fn rcon(&self) -> u64 {
    Self::x(self.bits(), 20, 4)
  }

  fn rvc_imm(&self) -> i64 {
    (Self::x(self.bits(), 2, 5) + (Self::xs(self.bits(), 12, 1) << 5))
      .try_into()
      .unwrap()
  }

  fn rvc_zimm(&self) -> i64 {
    (Self::x(self.bits(), 2, 5) + (Self::x(self.bits(), 12, 1) << 5))
      .try_into()
      .unwrap()
  }

  fn rvc_addi4spn_imm(&self) -> i64 {
    ((Self::x(self.bits(), 6, 1) << 2)
      + (Self::x(self.bits(), 5, 1) << 3)
      + (Self::x(self.bits(), 11, 2) << 4)
      + (Self::x(self.bits(), 7, 4) << 6))
      .try_into()
      .unwrap()
  }

  fn rvc_addi16sp_imm(&self) -> i64 {
    ((Self::x(self.bits(), 6, 1) << 4)
      + (Self::x(self.bits(), 2, 1) << 5)
      + (Self::x(self.bits(), 5, 1) << 6)
      + (Self::x(self.bits(), 3, 2) << 7)
      + (Self::xs(self.bits(), 12, 1) << 9))
      .try_into()
      .unwrap()
  }

  fn rvc_lwsp_imm(&self) -> i64 {
    ((Self::x(self.bits(), 4, 3) << 2)
      + (Self::x(self.bits(), 12, 1) << 5)
      + (Self::x(self.bits(), 2, 2) << 6))
      .try_into()
      .unwrap()
  }

  fn rvc_ldsp_imm(&self) -> i64 {
    ((Self::x(self.bits(), 5, 2) << 3)
      + (Self::x(self.bits(), 12, 1) << 5)
      + (Self::x(self.bits(), 2, 3) << 6))
      .try_into()
      .unwrap()
  }

  fn rvc_swsp_imm(&self) -> i64 {
    ((Self::x(self.bits(), 9, 4) << 2) + (Self::x(self.bits(), 7, 2) << 6))
      .try_into()
      .unwrap()
  }

  fn rvc_sdsp_imm(&self) -> i64 {
    ((Self::x(self.bits(), 10, 3) << 3) + (Self::x(self.bits(), 7, 3) << 6))
      .try_into()
      .unwrap()
  }

  fn rvc_lw_imm(&self) -> i64 {
    ((Self::x(self.bits(), 6, 1) << 2)
      + (Self::x(self.bits(), 10, 3) << 3)
      + (Self::x(self.bits(), 5, 1) << 6))
      .try_into()
      .unwrap()
  }

  fn rvc_ld_imm(&self) -> i64 {
    ((Self::x(self.bits(), 10, 3) << 3) + (Self::x(self.bits(), 5, 2) << 6))
      .try_into()
      .unwrap()
  }

  fn rvc_j_imm(&self) -> i64 {
    ((Self::x(self.bits(), 3, 3) << 1)
      + (Self::x(self.bits(), 11, 1) << 4)
      + (Self::x(self.bits(), 2, 1) << 5)
      + (Self::x(self.bits(), 7, 1) << 6)
      + (Self::x(self.bits(), 6, 1) << 7)
      + (Self::x(self.bits(), 9, 2) << 8)
      + (Self::x(self.bits(), 8, 1) << 10)
      + (Self::xs(self.bits(), 12, 1) << 11))
      .try_into()
      .unwrap()
  }

  fn rvc_b_imm(&self) -> i64 {
    ((Self::x(self.bits(), 3, 2) << 1)
      + (Self::x(self.bits(), 10, 2) << 3)
      + (Self::x(self.bits(), 2, 1) << 5)
      + (Self::x(self.bits(), 5, 2) << 6)
      + (Self::xs(self.bits(), 12, 1) << 8))
      .try_into()
      .unwrap()
  }

  fn rvc_simm3(&self) -> i64 {
    Self::x(self.bits(), 10, 3).try_into().unwrap()
  }

  fn rvc_rd(&self) -> u64 {
    self.rd()
  }

  fn rvc_rs1(&self) -> u64 {
    self.rd()
  }

  fn rvc_rs2(&self) -> u64 {
    Self::x(self.bits(), 2, 5)
  }

  fn rvc_rs1s(&self) -> u64 {
    8 + Self::x(self.bits(), 7, 3)
  }

  fn rvc_rs2s(&self) -> u64 {
    8 + Self::x(self.bits(), 2, 3)
  }

  fn rvc_lbimm(&self) -> u64 {
    (Self::x(self.bits(), 5, 1) << 1) + Self::x(self.bits(), 6, 1)
  }

  fn rvc_lhimm(&self) -> u64 {
    Self::x(self.bits(), 5, 1) << 1
  }

  fn rvc_r1sc(&self) -> u64 {
    Self::x(self.bits(), 7, 3)
  }

  fn rvc_r2sc(&self) -> u64 {
    Self::x(self.bits(), 2, 3)
  }

  fn rvc_rlist(&self) -> u64 {
    Self::x(self.bits(), 4, 4)
  }

  fn rvc_spimm(&self) -> u64 {
    Self::x(self.bits(), 2, 2) << 4
  }

  fn rvc_index(&self) -> u64 {
    Self::x(self.bits(), 2, 8)
  }

  fn v_vm(&self) -> u64 {
    Self::x(self.bits(), 25, 1)
  }

  fn v_wd(&self) -> u64 {
    Self::x(self.bits(), 26, 1)
  }

  fn v_nf(&self) -> u64 {
    Self::x(self.bits(), 29, 3)
  }

  fn v_simm5(&self) -> i64 {
    Self::xs(self.bits(), 15, 5).try_into().unwrap()
  }

  fn v_zimm5(&self) -> u64 {
    Self::x(self.bits(), 15, 5)
  }

  fn v_zimm10(&self) -> u64 {
    Self::x(self.bits(), 20, 10)
  }

  fn v_zimm11(&self) -> u64 {
    Self::x(self.bits(), 20, 11)
  }

  fn v_lmul(&self) -> u64 {
    Self::x(self.bits(), 20, 2)
  }

  fn v_frac_lmul(&self) -> u64 {
    Self::x(self.bits(), 22, 1)
  }

  fn v_sew(&self) -> u64 {
    1 << (Self::x(self.bits(), 23, 3) + 3)
  }

  fn v_width(&self) -> u64 {
    Self::x(self.bits(), 12, 3)
  }

  fn v_mop(&self) -> u64 {
    Self::x(self.bits(), 26, 2)
  }

  fn v_lumop(&self) -> u64 {
    Self::x(self.bits(), 20, 5)
  }

  fn v_sumop(&self) -> u64 {
    Self::x(self.bits(), 20, 5)
  }

  fn v_vta(&self) -> u64 {
    Self::x(self.bits(), 26, 1)
  }

  fn v_vma(&self) -> u64 {
    Self::x(self.bits(), 27, 1)
  }

  fn v_mew(&self) -> u64 {
    Self::x(self.bits(), 28, 1)
  }

  fn v_zimm6(&self) -> u64 {
    Self::x(self.bits(), 15, 5) + (Self::x(self.bits(), 26, 1) << 5)
  }

  fn p_imm2(&self) -> u64 {
    Self::x(self.bits(), 20, 2)
  }

  fn p_imm3(&self) -> u64 {
    Self::x(self.bits(), 20, 3)
  }
  fn p_imm4(&self) -> u64 {
    Self::x(self.bits(), 20, 4)
  }
  fn p_imm5(&self) -> u64 {
    Self::x(self.bits(), 20, 5)
  }
  fn p_imm6(&self) -> u64 {
    Self::x(self.bits(), 20, 6)
  }

  // fn zcmp_regmask(&self) -> u64 {
  // }

  // fn zcmp_stack_adjustment(&self, xlen: u64) -> u64 {
  // }
}

