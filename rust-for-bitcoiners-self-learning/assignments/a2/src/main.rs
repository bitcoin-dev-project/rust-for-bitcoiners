use std::vec::Vec;

/// Enum for representing Bitcoin Script Operations.
#[derive(Debug, Clone)]
enum ScriptOp {
    // define the variants for each opcodes
    OP_1,
    OP_ADD1,
}

impl ScriptOp {
    fn parse(byte: u8) -> ScriptOp {
        todo!()
    }
}

struct Script(Vec<ScriptOp>);

impl Script {
    fn parse(bytes: Vec<u8>) -> Self {
        todo!()
    }
    // NOTE: `Self` is just a synonym for Script here
    // Compare how parse is defined for `ScriptOp`
}

/// Struct for representing the execution context of the script.
struct ScriptExecutionContext {
    stack: Vec<i64>,
}

impl ScriptExecutionContext {
    /// Creates a new empty execution context.
    fn new() -> Self {
        todo!()
    }

    /// Push data onto the stack.
    fn push(&mut self, value: i64) {
        todo!()
    }

    /// Pop the top value from the stack.
    fn pop(&mut self) -> Option<i64> {
        todo!()
    }

    /// Execute the full script.
    fn run_script(&mut self, ops: Script) {
        let ops = ops.0; // NOTE: an example where variable shadowing is useful
        for op in ops {
            // Check what kind of an op it is using pattern matching
        }
    }

    /// Returns the final state of the stack after script execution.
    fn get_stack(&self) -> &Vec<i64> {
        &self.stack
    }
}

fn main() {
    let mut context = ScriptExecutionContext::new();

    // Example script: Push values onto the stack, perform arithmetic, and use flow control.
    let script = vec![
        ScriptOp::OP_1,
        ScriptOp::OP_ADD1,
        ScriptOp::OP_ADD1,
    ];

    context.run_script(Script(script));
    
    // Output the final stack state
    println!("Final stack: {:?}", context.get_stack());
}

// TODO: Add tests