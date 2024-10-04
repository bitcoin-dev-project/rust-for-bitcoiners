#![allow(unused)]
#[derive(Debug, Clone, PartialEq)]
enum ScriptOpcode {
    OpDup,          // OP_DUP
    OpHash160,      // OP_HASH160
    OpEqual,        // OP_EQUAL
    OpCheckSig,     // OP_CHECKSIG
    OpReturn,       // OP_RETURN
}

impl TryFrom<&str> for ScriptOpcode {
    type Error = String;

    fn try_from(op: &str) -> Result<Self, Self::Error> {
        match op {
            "OP_DUP" => Ok(ScriptOpcode::OpDup),
            "OP_HASH160" => Ok(ScriptOpcode::OpHash160),
            "OP_EQUAL" => Ok(ScriptOpcode::OpEqual),
            "OP_CHECKSIG" => Ok(ScriptOpcode::OpCheckSig),
            "OP_RETURN" => Ok(ScriptOpcode::OpReturn),
            _ => Err(format!("Invalid opcode: {}", op)),
        }
    }
}


#[derive(Debug, Clone, PartialEq)]
struct Script {
    opcodes: Vec<ScriptOpcode>,
}

impl Script {
    fn new(opcodes: Vec<ScriptOpcode>) -> Self {
        Script { opcodes }
    }
}

impl IntoIterator for Script {
    type Item = ScriptOpcode;
    type IntoIter = std::vec::IntoIter<ScriptOpcode>;

    fn into_iter(self) -> Self::IntoIter {
        self.opcodes.into_iter()
    }
}

fn main() {
    let script = Script::new(vec![
        ScriptOpcode::OpDup,
        ScriptOpcode::OpHash160,
        ScriptOpcode::OpEqual,
        ScriptOpcode::OpCheckSig,
    ]);

    for opcode in script {
        println!("{:?}", opcode);
    }

    let x: Result<ScriptOpcode, _> = "OP_DUP".try_into();

}


