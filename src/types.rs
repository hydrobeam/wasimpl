use crate::decode;
use std::ops::{Deref, Range};

pub struct WasmError {
    range: Range<usize>,
    err: WError,
}

pub enum ValidationError {
    TypeMismatch {
        location: String,
        expected: String,
        got: String,
    },
    InvalidDepth {
        max_depth: u8,
        got_depth: u8,
    },
    LimitExceeded {
        msg: String,
    },
    Catastrophic,
    Message {
        msg: String,
    },
}

pub enum ExecutionError {
    Unreachable,
}

pub enum WError {
    Validation(ValidationError),
    Trap,
    ExecutionError,
}

#[derive(Debug)]
pub enum VecType {}

#[derive(Debug)]
pub enum RefType {}

#[derive(Debug)]
pub enum ExternVal {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValType {
    I32,
    I64,
    F32,
    F64,
    V128,
    FuncRef,
    ExternRef,
}

impl ValType {
    pub fn is_num(self) -> bool {
        matches!(
            self,
            ValType::F32 | ValType::F64 | ValType::I32 | ValType::I64
        )
    }
    pub fn is_vec(self) -> bool {
        matches!(self, ValType::V128)
    }

    pub fn is_ref(self) -> bool {
        matches!(self, ValType::FuncRef | ValType::ExternRef)
    }

    pub fn from_byte(b: u8) -> decode::Result<ValType> {
        use ValType::*;
        match b {
            // numtype
            0x7f => Ok(I32),
            0x7e => Ok(I64),
            0x7d => Ok(F32),
            0x7c => Ok(F64),
            // vec type
            0x7b => Ok(V128),

            // reftype
            0x70 => Ok(FuncRef),
            0x6f => Ok(ExternRef),
            _ => Err(decode::DecodeError::Msg("invalid type".into())),
        }
    }
}

#[derive(Debug)]
pub struct Locals(Vec<ValType>);

impl Deref for Locals {
    type Target = Vec<ValType>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
