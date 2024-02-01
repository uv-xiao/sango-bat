use error_chain::error_chain;

error_chain! {
  types {
      Error, ErrorKind, ResultExt, Result;
  }

  foreign_links {
      Io(::std::io::Error);
      Json(::serde_json::Error);
      ParseIntError(::std::num::ParseIntError);
      Utf8(::std::string::FromUtf8Error);
  }

  errors {
      InvalidInsnIndex(index: usize) {
          description("Invalid instruction index")
          display("Invalid instruction index: {}", index)
      }
      NoCurrentBlock {
          description("No current block")
          display("No current block")
      }
      WrongNodeType {
          description("Wrong node type")
          display("Wrong node type (Phi/Insn)")
      }
      Analysis(m: String) {
          description("An error in the analysis")
          display("Analysis error: {}", m)
      }
      Arithmetic(m: String) {
          description("Error in evaluation of arithmetic expression")
          display("Arithmetic expression evaluation error: {}", m)
      }
      AccessUnmappedMemory(address: u64) {
          description("Attempt to access unmapped memory")
          display("Attempt to access unmapped memory at address 0x{:x}", address)
      }
      CapstoneError {
          description("Capstone failed")
          display("Capstone failed")
      }
      DisassemblyFailure {
          description("Unrecoverable error during disassembly")
          display("Disassembly Failure")
      }
      DivideByZero {
          description("Division by zero")
          display("Division by zero")
      }
      ExecutorScalar(name: String) {
          description("Executor can only execute over constant values")
          display("A scalar \"{}\" was found while executor was evaluating expression", name)
      }
      FunctionLocationApplication {
          description("Failed to apply il::FunctionLocation")
          display("Failed to apply il::FunctionLocation")
      }
      GraphEdgeNotFound(head: usize, tail: usize) {
          description("An edge was not found in a graph")
          display("The edge with head {} and tail {} does not exist in the graph", head, tail)
      }
      GraphVertexNotFound(vertex_id: usize) {
          description("A vertex was not found in a graph")
          display("The vertex id {} does not exist in the graph", vertex_id)
      }
      ProgramLocationMigration(reason: String) {
          description("Error migrating ProgramLocation between Program")
          display("Failed to migrate ProgramLocation between Program: {}", reason)
      }
      ProgramLocationApplication {
          description("Failed to apply il::ProgramLocation")
          display("Failed to apply il::ProgramLocation")
      }
      Sort {
          description("Sort error, invalid bitness between expressions")
          display("Sort error, invalid bitness between expressions")
      }
      TooManyAddressBits {
          description("A constant with >64 bits was used as an address")
          display("A constant with >64 bits was used as an address")
      }
      UnhandledIntrinsic(intrinsic_str: String) {
          description("An unhandled intrinsic was encountered during evaluation")
          display("Encountered unhandled intrinsic {}", intrinsic_str)
      }
  }
}
