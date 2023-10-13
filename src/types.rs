use std::ops::Range;

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

#[derive(Debug)]
pub enum ValType {}
