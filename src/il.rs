type block_t = usize;
type insn_t = usize;
type reg_t = usize;
type pc_t = u64;

pub mod block;
use std::collections::{BTreeMap, HashSet, VecDeque};

use block::*;
use log::{info, warn};

pub mod node;

// use crate::errors::*;
use crate::graph::{self, Edge};
use crate::graph::Vertex;
use crate::instruction::Insn;
use crate::streamer::proto::UtMsg;

use self::node::Node;

#[derive(Clone, Debug, Default)]
pub struct Table {
  last_insn: Option<insn_t>,
  current_block: Option<block_t>,
  reg_table: BTreeMap<reg_t, insn_t>,
}

#[derive(Clone, Debug, Default)]
pub struct Statistic {}

#[derive(Clone, Debug, Default)]
pub struct Information {
  insn_latency_count: Vec<(f64, u64)>,
  block_count: Vec<u64>,
  pub last_commit_timestamp: u64,
  pub updated_timestamp: u64,
  pub insn_latency_buffer: VecDeque<(insn_t, u64)>,
}

#[derive(Clone, Debug, Default)]
pub struct CFG {
  graph: graph::Graph<Block, BEdge>,
  next_block_index: block_t,
  next_node_index: insn_t,
  pc_insn: BTreeMap<pc_t, insn_t>,
  insn_block: Vec<(block_t, usize)>,
  stack: Table,
  information: Information,
  statistic: Statistic,
}

impl CFG {
  pub fn new() -> Self {
    let mut cfg = Self::default();
    cfg.new_block(None);
    cfg
  }

  pub fn graph(&self) -> &graph::Graph<Block, BEdge> {
    &self.graph
  }

  fn phi_block(&self) -> block_t {
    0
  }

  fn block_head(&self) -> block_t {
    1
  }

  fn new_insn_node(&mut self, insn: Insn, block_index: block_t) -> insn_t {
    let node_index = self.next_node_index();
    let block = self.block_mut(block_index);
    let pc = insn.pc();
    block.add_insn(node_index, insn);
    let idx_in_block = block.nodes().len() - 1;
    self.insn_block.push((block_index, idx_in_block));
    self.pc_insn.insert(pc, node_index);
    node_index
  }

  fn new_phi_node(&mut self) -> insn_t {
    let node_index = self.next_node_index();
    let block = self.block_mut(self.phi_block());
    block.add_phi(node_index);
    let idx_in_block = block.nodes().len() - 1;
    self.insn_block.push((self.phi_block(), idx_in_block));
    node_index
  }

  fn new_block(&mut self, last_block_index: Option<block_t>) -> &mut Block {
    let index = self.next_block_index;
    self.next_block_index += 1;
    let block = Block::new(index);
    self.graph.insert_vertex(block).unwrap();
    if let Some(last_block_index) = last_block_index {
      self
        .graph
        .insert_edge(BEdge::new(last_block_index, index))
        .unwrap();
    }
    self.graph.vertex_mut(index).unwrap()
  }

  fn next_node_index(&mut self) -> insn_t {
    let index = self.next_node_index;
    self.next_node_index += 1;
    index
  }

  fn last_insn(&self) -> Option<insn_t> {
    self.stack.last_insn
  }

  fn block(&self, index: block_t) -> &Block {
    self.graph.vertex(index).unwrap()
  }

  fn block_mut(&mut self, index: block_t) -> &mut Block {
    self.graph.vertex_mut(index).unwrap()
  }

  fn block_index_of_insn(&self, index: insn_t) -> block_t {
    let (block_index, _) = self.insn_block[index];
    block_index
  }

  fn block_of_insn(&self, index: insn_t) -> &Block {
    let (block_index, _) = self.insn_block[index];
    self.block(block_index)
  }

  fn block_mut_of_insn(&mut self, index: insn_t) -> &mut Block {
    let (block_index, _) = self.insn_block[index];
    self.block_mut(block_index)
  }

  fn node(&self, index: insn_t) -> &Node {
    let (block_index, insn_index) = self.insn_block[index];
    let block = self.block(block_index);
    block.node(insn_index).unwrap()
  }

  fn insn_node(&self, global_insn_index: insn_t) -> &Node {
    let (block_index, insn_index) = self.insn_block[global_insn_index];
    let block = self.block(block_index);
    block.insn_node(insn_index).unwrap()
  }

  fn insn_node_mut(&mut self, global_insn_index: insn_t) -> &mut Node {
    let (block_index, insn_index) = self.insn_block[global_insn_index];
    let block = self.block_mut(block_index);
    block.insn_node_mut(insn_index).unwrap()
  }

  fn node_mut(&mut self, global_insn_index: insn_t) -> &mut Node {
    let (block_index, insn_index) = self.insn_block[global_insn_index];
    let block: &mut Block = self.block_mut(block_index);
    block.node_mut(insn_index).unwrap()
  }

  fn current_block_mut(&mut self) -> &mut Block {
    match self.stack.current_block {
      Some(index) => self.block_mut(index),
      None => panic!("current block is None"),
    }
  }

  fn node_is_leading(&self, index: insn_t) -> bool {
    let (_, insn_idx) = self.insn_block[index];
    insn_idx == 0
  }

  fn current_block_index(&self) -> Option<block_t> {
    self.stack.current_block
  }

  fn last_insn_is_not_control(&self) -> bool {
    match self.last_insn() {
      Some(last_insn) => {
        let last_insn_node = self.insn_node(last_insn);
        !last_insn_node.insn().unwrap().is_control()
      }
      None => false,
    }
  }

  fn last_insn_is_control(&self) -> bool {
    match self.last_insn() {
      Some(last_insn) => {
        let last_insn_node = self.insn_node(last_insn);
        last_insn_node.insn().unwrap().is_control()
      }
      None => false,
    }
  }

  #[allow(unreachable_code)]
  fn concat_block(&mut self, first: block_t, second: block_t) {
    panic!("deprecated: do NOT use concat_block");
    info!("concat block: {} -> {}", first, second);
    let mut second_block = self.block(second).clone();
    let first_block = self.block_mut(first);
    // Create a temporary vector to store modifications
    let mut modifications = Vec::new();

    for node in second_block.nodes_mut().iter_mut() {
      first_block.add_node(node);

      // Store modifications in the temporary vector
      modifications.push((
        node.index(),
        first_block.index(),
        first_block.nodes().len() as usize - 1,
      ));
    }

    // Apply modifications after the iteration is complete
    for (node_index, block_index, block_node_idx) in modifications.iter() {
      self.insn_block[*node_index] = (*block_index, *block_node_idx);
    }

    self.graph.remove_vertex(second).unwrap();
  }

  fn split_block(&mut self, block_index: block_t, insn_index: insn_t) {
    let insn_index_in_block = self.insn_block[insn_index].1;

    let new_block = self.new_block(Some(block_index)).index();

    let mut modifications = Vec::new();

    let mut block_nodes: Vec<Node> = Vec::new();
    let mut new_block_nodes: Vec<Node> = Vec::new();
    let mut phi_nodes_used_by_new_block: HashSet<insn_t> = HashSet::new();

    for (idx, node) in self.block(block_index).nodes().iter().enumerate() {
      if node.is_phi() {
        panic!("phi node in the middle of a block")
      } else if idx < insn_index_in_block {
        block_nodes.push(node.clone());
        modifications.push((node.index(), block_index, block_nodes.len() - 1));
      } else {
        for src in node.srcs() {
          if self.node(src).is_phi() {
            phi_nodes_used_by_new_block.insert(src);
          }
        }
        new_block_nodes.push(node.clone());
        modifications.push((node.index(), new_block, new_block_nodes.len() - 1));
      }
    }

    self.block_mut(block_index).nodes = block_nodes;
    self.block_mut(new_block).nodes = new_block_nodes;

    for (node_index, block_index, block_node_idx) in modifications.iter() {
      self.insn_block[*node_index] = (*block_index, *block_node_idx);
    }
  }

  fn equal_or_phi_includes(&self, old: insn_t, new: insn_t) -> bool {
    if self.node(old).is_phi() {
      self.node(old).srcs().contains(&new)
    } else {
      old == new
    }
  }

  fn phi_merge(&mut self, old: insn_t, new: insn_t) -> insn_t {
    if self.node(old).is_phi() {
      let node = self.node_mut(old);
      node.srcs.push(new);
      old
    } else {
      let phi_index = self.new_phi_node();
      let phi_node = self.node_mut(phi_index);
      phi_node.set_srcs(vec![old, new]);
      phi_index
    }
  }


  fn update_information(&mut self, msg: &UtMsg, insn_index: insn_t) {
    let timestamp = msg.timestamp.commit;
    if timestamp > self.information.last_commit_timestamp {
      // update latency according to the buffer
      let buffer_size = self.information.insn_latency_buffer.len();
      for (insn_index, timestamp) in self.information.insn_latency_buffer.drain(..) {
        let latency = (timestamp - self.information.last_commit_timestamp) as f64 / buffer_size as f64;
        
      }

      
    } else {
      assert_eq!(timestamp, self.information.last_commit_timestamp);
      // append the latency to the buffer
      self.information.insn_latency_buffer.push_back((insn_index, timestamp));
    }
  }

  /*
   * Parse a message:
   * - function handling
   * - block handling
   *   1. if never met the instruction:
   *     1.1. if the last instruction is a control instruction or there's no last instruction:
   *          - create a new block
   *     1.2. if the last instruction is not a control instruction:
   *          - add the instruction to the current block
   *   2. if met the instruction before:
   *     2.1. [deprecated] if the last instruction is not a control instruction, but the current instruction is the start of a new block:
   *          - append the block of the current instruction to the current block
   *     2.2. if the last instruction is a control instruction, but the current instruction is not the start of a block;
   *          - split the block
   * - node handling
   *   - iterate the registers of the instruction
   */
  pub fn parse_msg(&mut self, msg: UtMsg) {
    let insn = Insn::new(&msg);
    let pc = insn.pc();

    // TODO: function handling

    let insn_option = self.pc_insn.get(&pc).cloned();
    let new_insn_node = match insn_option {
      Some(global_insn) => {
        if self.last_insn_is_not_control() {
          // deprecated: there is a last instruction and it's not a control instruction
          // append the block of the current instruction to the current block
          // let insn_node = self.insn_node(global_insn);

          // if self.node_is_leading(global_insn) {
          //   if let Some(block_index) = self.current_block_index() {
          //     self.concat_block(block_index, insn_node.block());
          //   }
          // }
          global_insn
        } else if self.last_insn_is_control() && !self.node_is_leading(global_insn) {
          let old_block_index = self.block_index_of_insn(global_insn);
          info!(
            "before split: {} is the {}-th insn of block {}",
            global_insn, self.insn_block[global_insn].1, old_block_index
          );
          self.split_block(self.block_index_of_insn(global_insn), global_insn);
          info!(
            "split block: {} --[{}]-> {}",
            old_block_index,
            self.insn_node(global_insn).insn().unwrap().disasm(),
            self.block_index_of_insn(global_insn)
          );

          info!(
            "add edge: {} -> {}",
            self.block_index_of_insn(self.last_insn().unwrap()),
            self.block_index_of_insn(global_insn)
          );

          self
            .graph
            .insert_edge(BEdge::new(
              self.block_index_of_insn(self.last_insn().unwrap()),
              self.block_index_of_insn(global_insn),
            ))
            .unwrap();

          global_insn
        } else {
          global_insn
        }
      }
      None => {
        if self.last_insn_is_not_control() {
          // there is a last instruction and it's not a control instruction
          // append the current instruction to the current block
          self.new_insn_node(insn, self.current_block_index().unwrap())
          // let node_index = self.next_node_index();
          // let block = self.current_block_mut();
          // block.add_insn(node_index, insn)
        } else {
          // there is no last instruction or the last instruction is a control instruction
          // create a new block
          let block = self.new_block(self.current_block_index()).index();
          self.new_insn_node(insn, block)
        }
      }
    };

    // TODO: statistic

    // handle dependencies
    if let Some(reg_access) = msg.reg.as_ref() {
      let mut srcs = Vec::new();
      for reg in reg_access.srcs.iter() {
        let reg = *reg as usize;
        let reg_node = self.stack.reg_table.get(&reg).cloned();
        if let Some(reg_node) = reg_node {
          srcs.push(reg_node);
        }
      }

      // compare the elements of the new srcs with the old node.srcs
      // if they are not equal, update the srcs by creating a phi node
      let node = self.insn_node(new_insn_node);
      let mut old_srcs = node.srcs();
      // fill the old_srcs with elements from srcs to make them have the same length
      while old_srcs.len() < srcs.len() {
        old_srcs.push(srcs[old_srcs.len()]);
      }
      let mut final_srcs = Vec::new();
      for (old_src, new_src) in old_srcs.iter().zip(srcs.iter()) {
        if !self.equal_or_phi_includes(*old_src, *new_src) {
          let phi_index =
            self.phi_merge(*old_src, *new_src);
          final_srcs.push(phi_index);
        } else {
          final_srcs.push(*old_src);
        }
      }

      self.insn_node_mut(new_insn_node).set_srcs(final_srcs);
    }

    // update stack/table: last_insn, current_block, reg_table, information
    self.stack.last_insn = Some(new_insn_node);
    self.stack.current_block = Some(self.insn_node(new_insn_node).block());

    if let Some(reg_access) = msg.reg.as_ref() {
      for dst in reg_access.dsts.iter() {
        let dst = *dst as usize;
        self.stack.reg_table.insert(dst, new_insn_node);
      }
    }

    self.update_information(&msg, new_insn_node);


  }

  pub fn debug(&self) {
    for block in self.graph.vertices() {
      println!("block: {}", block.index());
      for node in block.nodes() {
        println!("  node: {}", node);
        println!("    insn: {:?}", node.insn());
        println!("    srcs: {:?}", node.srcs());
      }
    }

    for edge in self.graph.edges() {
      println!("edge: {} -> {}", edge.head(), edge.tail());
    }

    let loop_tree = self.graph.compute_loop_tree(self.block_head()).unwrap();

    for loop_node in loop_tree.vertices() {
      println!("loop: {:?}", loop_node);
    }
  }
}
