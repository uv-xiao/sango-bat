use core::fmt;

use crate::{graph, il::node::*, instruction::Insn};

use super::{block_t, insn_t};

use crate::errors::*;

#[derive(Clone, Debug, Default)]
pub struct Block {
  index: block_t,
  pub nodes: Vec<Node>,
}

// Naming rule: index is global, idx is local
impl Block {
  pub fn new(index: block_t) -> Self {
    Block {
      index,
      nodes: Vec::new(),
    }
  }

  pub fn phi_nodes(&self) -> Vec<&Node> {
    self.nodes.iter().filter(|n| n.is_phi()).collect()
  }
  pub fn insn_nodes(&self) -> Vec<&Node> {
    self.nodes.iter().filter(|n| !n.is_phi()).collect()
  }
  
  pub fn nodes(&self) -> &Vec<Node> {
    &self.nodes
  }
  pub fn nodes_mut(&mut self) -> &mut Vec<Node> {
    &mut self.nodes
  }

  pub fn node(&self, idx: insn_t) -> Result<&Node> {
    self.nodes.get(idx as usize).ok_or(ErrorKind::InvalidInsnIndex(idx).into())
  }

  pub fn node_mut(&mut self, idx: insn_t) -> Result<&mut Node> {
    self.nodes.get_mut(idx as usize).ok_or(ErrorKind::InvalidInsnIndex(idx).into())
  }

  pub fn insn_node(&self, idx: insn_t) -> Result<&Node> {
    let node = self.node(idx)?;
    if node.is_phi() {
      Err(ErrorKind::WrongNodeType.into())
    } else {
      Ok(node)
    }
  }

  pub fn insn_node_mut(&mut self, idx: insn_t) -> Result<&mut Node> {
    let node = self.node_mut(idx)?;
    if node.is_phi() {
      Err(ErrorKind::WrongNodeType.into())
    } else {
      Ok(node)
    }
  }

  pub fn add_insn(&mut self, index: insn_t, insn: Insn) -> insn_t {
    let insn_node = Node::new(self.index, index, insn);
    self.nodes.push(insn_node);
    index
  }

  pub fn add_phi(&mut self, index: insn_t) -> insn_t {
    let phi_node = Node::new_phi(self.index, index);
    self.nodes.push(phi_node);
    index
  }

  pub fn add_node(&mut self, node: &mut Node) {
    if node.is_phi() {
      self.add_phi(node.index());
      self.nodes.last_mut().unwrap().set_srcs_from(node);
    } else {
      self.add_insn(node.index(), node.insn().unwrap().clone());
      self.nodes.last_mut().unwrap().set_srcs_from(node);
    }
  }

}

impl fmt::Display for Block {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    writeln!(f, "[ Block: 0x{:X} ]", self.index)?;
    for phi_node in self.phi_nodes() {
      writeln!(f, "{}", phi_node)?;
    }
    for insn in self.nodes() {
      writeln!(f, "{}", insn)?;
    }
    Ok(())
  }
}

impl graph::Vertex for Block {
  fn index(&self) -> usize {
    self.index
  }

  fn dot_label(&self) -> String {
    format!("{}", self)
  }
}

#[derive(Clone, Debug, Default)]
pub struct Edge {
  head: block_t,
  tail: block_t,
}

impl Edge {
  
  pub fn new(head: block_t, tail: block_t) -> Self {
    Edge {
      head,
      tail,
    }
  }
}

impl graph::Edge for Edge {


  fn head(&self) -> usize {
    self.head as usize
  }

  fn tail(&self) -> usize {
    self.tail as usize
  }

  fn dot_label(&self) -> String {
    "".to_string()
  }
}
