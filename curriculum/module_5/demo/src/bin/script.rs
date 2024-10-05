#![allow(unused)]
#[derive(Debug, Clone, PartialEq)]
enum ScriptOpcode {
    OpDup,          // OP_DUP
    OpHash160,      // OP_HASH160
    OpEqual,        // OP_EQUAL
    OpCheckSig,     // OP_CHECKSIG
    OpReturn,       // OP_RETURN
}

use ScriptOpcode::*;

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

impl TryFrom<u8> for ScriptOpcode {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            188 => Ok(OpDup),
            // TODO extend this pattern matching to cover other cases
            _ => Err(format!("Invalid opcode: {}", value)),
        }
    }
}

struct Empty;


fn main() {
    let op = OpEqual;
    let op1 = op.clone();
    let x = ScriptOpcode::try_from("OP_DUP");

    let y: Result<ScriptOpcode, _> = TryFrom::try_from("OP_CHECK_SIG");

    let z: Result<ScriptOpcode, _> = "OP_DUP".try_into();

    let op: ScriptOpcode = 188u8.try_into().unwrap();

    assert_eq!(op, OpDup);
}


