use std::ops::{Add as OtherAdd, Div as OtherDiv, Mul as OtherMul, Sub as OtherSub};

use crate::execution;
use crate::module::Module;
// use crate::runtime::{Context, Store};
use crate::types::ValType;
use crate::validate::{self, CtrlStack, ValStack, ValidationCtx};
use paste::paste;
// control
// parametric

// drop
// select

// variable

// memory

// numeric

pub trait Instruction: validate::Validate + std::fmt::Debug // + execution::Execute
{
}

// numerics
//
// these are very generic so we (ab)use macros to reduce the amount of boilerplate

pub trait NumericInstr: Instruction {
    fn to_valtype(self) -> ValType;
}

macro_rules! numeric_instr {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy)]
        pub enum $name {
            I32,
            I64,
            F32,
            F64,
        }

        impl Instruction for $name {}
        impl NumericInstr for $name {
            fn to_valtype(self) -> ValType {
                use $name::*;
                match self {
                    I32 => ValType::I32,
                    I64 => ValType::I64,
                    F32 => ValType::F32,
                    F64 => ValType::F64,
                }
            }
        }
    };

    ($name:ident, signed) => {
        #[derive(Debug, Clone, Copy)]
        pub enum $name {
            I32,
            I64,
            U32,
            U64,
            F32,
            F64,
        }

        impl Instruction for $name {}

        impl NumericInstr for $name {
            fn to_valtype(self) -> ValType {
                use $name::*;
                match self {
                    I32 | U32 => ValType::I32,
                    I64 | U64 => ValType::I64,
                    F32 => ValType::F32,
                    F64 => ValType::F64,
                }
            }
        }
    };

    ($name:ident, integer) => {
        #[derive(Debug, Clone, Copy)]
        pub enum $name {
            I32,
            I64,
        }

        impl Instruction for $name {}
        impl NumericInstr for $name {
            fn to_valtype(self) -> ValType {
                use $name::*;
                match self {
                    I32 => ValType::I32,
                    I64 => ValType::I64,
                }
            }
        }
    };

    ($name:ident, integer, signed) => {
        #[derive(Debug, Clone, Copy)]
        pub enum $name {
            I32,
            I64,
            U32,
            U64,
        }

        impl Instruction for $name {}

        impl NumericInstr for $name {
            fn to_valtype(self) -> ValType {
                use $name::*;
                match self {
                    I32 | U32 => ValType::I32,
                    I64 | U64 => ValType::I64,
                }
            }
        }
    };

    ($name:ident, float) => {
        #[derive(Debug, Clone, Copy)]
        pub enum $name {
            F32,
            F64,
        }

        impl Instruction for $name {}
        impl NumericInstr for $name {
            fn to_valtype(self) -> ValType {
                use $name::*;
                match self {
                    F32 => ValType::F32,
                    F64 => ValType::F64,
                }
            }
        }
    };
}

numeric_instr!(Add);
numeric_instr!(Sub);
numeric_instr!(Mul);
numeric_instr!(Div, signed);
numeric_instr!(Gt, signed);
numeric_instr!(Ge, signed);
numeric_instr!(Lt, signed);
numeric_instr!(Le, signed);

numeric_instr!(WasmEq);
numeric_instr!(Ne);

numeric_instr!(Const);

numeric_instr!(And, integer);
numeric_instr!(Or, integer);
numeric_instr!(Rem, integer, signed);

numeric_instr!(Eqz, integer);
numeric_instr!(Xor, integer);
numeric_instr!(Rotl, integer);
numeric_instr!(Rotr, integer);
numeric_instr!(Clz, integer);
numeric_instr!(Ctz, integer);
numeric_instr!(Popcnt, integer);
numeric_instr!(Shl, integer);
numeric_instr!(Shr, integer, signed);

numeric_instr!(Min, float);
numeric_instr!(Max, float);
numeric_instr!(CopySign, float);
numeric_instr!(Abs, float);
numeric_instr!(Neg, float);
numeric_instr!(Ceil, float);
numeric_instr!(Floor, float);
numeric_instr!(Nearest, float);
numeric_instr!(Sqrt, float);

// memory

pub trait MemInstr: Instruction {
    fn memarg(self) -> MemArg;
    fn to_valtype(self) -> ValType;
}

macro_rules! mem_instr {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy)]
        pub enum $name {
            I32(MemArg),
            I64(MemArg),
            F32(MemArg),
            F64(MemArg),
        }

        impl Instruction for $name {}

        impl MemInstr for $name {
            fn to_valtype(self) -> ValType {
                use $name::*;
                match self {
                    I32(_) => ValType::I32,
                    I64(_) => ValType::I64,
                    F32(_) => ValType::F32,
                    F64(_) => ValType::F64,
                }
            }
            fn memarg(self) -> MemArg {
                use $name::*;
                match self {
                    I32(mem) | I64(mem) | F32(mem) | F64(mem) => mem,
                }
            }
        }
    };

    ($name:ident, integer) => {
        #[derive(Debug, Clone, Copy)]
        pub enum $name {
            I32(MemArg),
            I64(MemArg),
        }

        impl Instruction for $name {}
        impl MemInstr for $name {
            fn to_valtype(self) -> ValType {
                use $name::*;
                match self {
                    I32(_) => ValType::I32,
                    I64(_) => ValType::I64,
                }
            }
            fn memarg(self) -> MemArg {
                use $name::*;
                match self {
                    I32(mem) | I64(mem) => mem,
                }
            }
        }
    };

    ($name:ident, integer, signed) => {
        #[derive(Debug, Clone, Copy)]
        pub enum $name {
            I32(MemArg),
            I64(MemArg),
            U32(MemArg),
            U64(MemArg),
        }

        impl Instruction for $name {}

        impl MemInstr for $name {
            fn to_valtype(self) -> ValType {
                use $name::*;
                match self {
                    I32(_) | U32(_) => ValType::I32,
                    I64(_) | U64(_) => ValType::I64,
                }
            }
            fn memarg(self) -> MemArg {
                use $name::*;
                match self {
                    I32(mem) | I64(mem) | U32(mem) | U64(mem) => mem,
                }
            }
        }
    };
}

#[derive(Clone, Copy, Debug)]
pub struct MemArg {
    offset: u32,
    align: u32,
}

// loads

mem_instr!(Load);
mem_instr!(Load8, integer, signed);
mem_instr!(Load16, integer, signed);

#[derive(Clone, Copy, Debug)]
pub enum Load32 {
    I64(MemArg),
    U64(MemArg),
}

impl Instruction for Load32 {}

impl MemInstr for Load32 {
    fn to_valtype(self) -> ValType {
        ValType::I64
    }
    fn memarg(self) -> MemArg {
        match self {
            Load32::I64(mem) | Load32::U64(mem) => mem,
        }
    }
}

// stores
mem_instr!(Store);
mem_instr!(Store8, integer);
mem_instr!(Store16, integer);

#[derive(Clone, Copy, Debug)]
pub struct Store32 {
    memarg: MemArg,
}

impl Instruction for Store32 {}
impl MemInstr for Store32 {
    fn to_valtype(self) -> ValType {
        ValType::I64
    }
    fn memarg(self) -> MemArg {
        self.memarg
    }
}
// variable instructions

pub trait VariableInstr: Instruction {
    fn idx(self) -> u32;
}

#[derive(Debug, Copy, Clone)]
pub enum Get {
    Local { idx: u32 },
    Global { idx: u32 },
}

impl Instruction for Get {}
impl VariableInstr for Get {
    fn idx(self) -> u32 {
        match self {
            Get::Local { idx } | Get::Global { idx } => idx,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Set {
    Local { idx: u32 },
    Global { idx: u32 },
}

impl Instruction for Set {}
impl VariableInstr for Set {
    fn idx(self) -> u32 {
        match self {
            Set::Local { idx } | Set::Global { idx } => idx,
        }
    }
}

// Local only
#[derive(Debug, Copy, Clone)]
pub struct Tee {
    pub idx: u32,
}

impl Instruction for Tee {}
impl VariableInstr for Tee {
    fn idx(self) -> u32 {
        self.idx
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Memory {
    Grow,
    Size,
    Fill,
    Copy,
    // FIXME proper type
    Init { dataidx: u32 },
}
impl Instruction for Memory {}

// parametric

#[derive(Debug)]
pub struct Drop;
#[derive(Debug)]
pub struct Select {
    pub val: Option<ValType>,
}

impl Instruction for Drop {}
impl Instruction for Select {}

// control flow

#[derive(Debug, Copy, Clone)]
pub struct Unreachable;

#[derive(Debug, Clone, Copy)]
pub enum BlockType {
    Idx(u32),
    ValType(ValType),
    // void type
    Void,
}

#[derive(Debug)]
pub struct Block {
    pub blocktype: BlockType,
    pub instructions: Vec<Box<dyn Instruction>>,
}

#[derive(Debug)]
pub struct Loop {
    pub blocktype: BlockType,
    pub instructions: Vec<Box<dyn Instruction>>,
}

#[derive(Debug)]
pub struct If {
    pub blocktype: BlockType,
    pub true_instructions: Vec<Box<dyn Instruction>>,
    pub false_instructions: Vec<Box<dyn Instruction>>,
}

impl Instruction for If {}
impl Instruction for Loop {}
impl Instruction for Block {}
impl Instruction for Unreachable {}

#[derive(Debug)]
pub struct Br {
    pub label_idx: u32,
}

#[derive(Debug)]
pub struct BrIf {
    pub label_idx: u32,
}

impl Instruction for Br {}
impl Instruction for BrIf {}
