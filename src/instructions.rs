use std::ops::{Add as OtherAdd, Div as OtherDiv, Mul as OtherMul, Sub as OtherSub};

use crate::execution;
use crate::module::Module;
use crate::runtime::{Context, Store};
use crate::validate::{self, CtrlStack, ValStack, ValType, ValidationCtx};
use paste::paste;
// control
// parametric

// drop
// select

// variable

// memory

// numeric

// enum InstrType {
//     Unreachable,
//     Nop,
//     Block,
//     LocalGet,
//     LocalSet,
// }

trait Instruction: validate::Validate + execution::Execute {
    fn to_valtype(self) -> validate::ValType;
}

macro_rules! numeric_instr {
    ($name:ident) => {
        // name is the name of the enum
        #[derive(Debug)]
        pub enum $name {
            I32,
            I64,
            F32,
            F64,
        }

        impl Instruction for $name {
            fn to_valtype(self) -> validate::ValType {
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
        // name is the name of the enum
        #[derive(Debug)]
        pub enum $name {
            I32,
            I64,
            U32,
            U64,
            F32,
            F64,
        }

        impl Instruction for $name {
            fn to_valtype(self) -> validate::ValType {
                use $name::*;
                match self {
                    I32 | U32 => ValType::I32,
                    I64 | U64 => ValType::I64,
                    F32 => ValType::F32,
                    F64 => ValType::F64,
                }
            }
        }
        // impl Instruction for $name {}
    };

    ($name:ident, dual) => {
        // name is the name of the enum
        #[derive(Debug)]
        pub enum $name {
            I32,
            I64,
        }

        impl Instruction for $name {
            fn to_valtype(self) -> validate::ValType {
                use $name::*;
                match self {
                    I32 => ValType::I32,
                    I64 => ValType::I64,
                }
            }
        }

        // impl Instruction for $name {}
    };

    ($name:ident, dual, signed) => {
        // name is the name of the enum
        #[derive(Debug)]
        pub enum $name {
            I32,
            I64,
            U32,
            U64,
        }
        // impl Instruction for $name {}
        impl Instruction for $name {
            fn to_valtype(self) -> validate::ValType {
                use $name::*;
                match self {
                    I32 | U32 => ValType::I32,
                    I64 | U64 => ValType::I64,
                }
            }
        }
    };

    ($name:ident, float) => {
        // name is the name of the enum
        #[derive(Debug)]
        pub enum $name {
            F32,
            F64,
        }

        impl Instruction for $name {
            fn to_valtype(self) -> validate::ValType {
                use $name::*;
                match self {
                    F32 => ValType::F32,
                    F64 => ValType::F64,
                }
            }
        }
        // impl Instruction for $name {}
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

numeric_instr!(And, dual);
numeric_instr!(Or, dual);
numeric_instr!(Rem, dual, signed);

numeric_instr!(Eqz, dual);
numeric_instr!(Xor, dual);
numeric_instr!(Rotl, dual);
numeric_instr!(Rotr, dual);
numeric_instr!(Clz, dual);
numeric_instr!(Ctz, dual);
numeric_instr!(Popcnt, dual);
numeric_instr!(Shl, dual);
numeric_instr!(Shr, dual, signed);
// impl Instruction for Eqz {}

numeric_instr!(Min, float);
numeric_instr!(Max, float);
numeric_instr!(CopySign, float);
numeric_instr!(Abs, float);
numeric_instr!(Neg, float);
numeric_instr!(Ceil, float);
numeric_instr!(Floor, float);
numeric_instr!(Nearest, float);
numeric_instr!(Sqrt, float);

#[derive(Debug)]
enum Get {
    Local,
    Global,
}

fn from(value: u8) -> () {
    match value {
        // unreachable
        0x00 => {}
        // nop
        0x01 => {}
        // block
        0x02 => {}
        // loop
        0x03 => {}
        // if
        0x04 => {}
        // else
        0x05 => {}
        // exception handling proposal, unimplemented
        0x06 => {}
        0x07 => {}
        0x08 => {}
        0x09 => {}
        0x0a => {}
        //
        // end
        0x0b => {}
        //br
        0x0c => {}
        //br_if
        0x0d => {}
        0x0e => {}
        0x0f => {}
        0x10 => {}
        0x11 => {}
        0x12 => {}
        0x13 => {}
        0x14 => {}
        0x15 => {}
        0x16 => {}
        0x17 => {}
        0x18 => {}
        0x19 => {}
        0x1a => {}
        0x1b => {}
        0x1c => {}
        0x1d => {}
        0x1e => {}
        0x1f => {}
        0x20 => {}
        0x21 => {}
        0x22 => {}
        0x23 => {}
        0x24 => {}
        0x25 => {}
        0x26 => {}
        0x27 => {}
        0x28 => {}
        0x29 => {}
        0x2a => {}
        0x2b => {}
        0x2c => {}
        0x2d => {}
        0x2e => {}
        0x2f => {}
        0x30 => {}
        0x31 => {}
        0x32 => {}
        0x33 => {}
        0x34 => {}
        0x35 => {}
        0x36 => {}
        0x37 => {}
        0x38 => {}
        0x39 => {}
        0x3a => {}
        0x3b => {}
        0x3c => {}
        0x3d => {}
        0x3e => {}
        0x3f => {}
        0x40 => {}
        0x41 => {}
        0x42 => {}
        0x43 => {}
        0x44 => {}
        0x45 => {}
        0x46 => {}
        0x47 => {}
        0x48 => {}
        0x49 => {}
        0x4a => {}
        0x4b => {}
        0x4c => {}
        0x4d => {}
        0x4e => {}
        0x4f => {}
        0x50 => {}
        0x51 => {}
        0x52 => {}
        0x53 => {}
        0x54 => {}
        0x55 => {}
        0x56 => {}
        0x57 => {}
        0x58 => {}
        0x59 => {}
        0x5a => {}
        0x5b => {}
        0x5c => {}
        0x5d => {}
        0x5e => {}
        0x5f => {}
        0x60 => {}
        0x61 => {}
        0x62 => {}
        0x63 => {}
        0x64 => {}
        0x65 => {}
        0x66 => {}
        0x67 => {}
        0x68 => {}
        0x69 => {}
        0x6a => {}
        0x6b => {}
        0x6c => {}
        0x6d => {}
        0x6e => {}
        0x6f => {}
        0x70 => {}
        0x71 => {}
        0x72 => {}
        0x73 => {}
        0x74 => {}
        0x75 => {}
        0x76 => {}
        0x77 => {}
        0x78 => {}
        0x79 => {}
        0x7a => {}
        0x7b => {}
        0x7c => {}
        0x7d => {}
        0x7e => {}
        0x7f => {}
        0x80 => {}
        0x81 => {}
        0x82 => {}
        0x83 => {}
        0x84 => {}
        0x85 => {}
        0x86 => {}
        0x87 => {}
        0x88 => {}
        0x89 => {}
        0x8a => {}
        0x8b => {}
        0x8c => {}
        0x8d => {}
        0x8e => {}
        0x8f => {}
        0x90 => {}
        0x91 => {}
        0x92 => {}
        0x93 => {}
        0x94 => {}
        0x95 => {}
        0x96 => {}
        0x97 => {}
        0x98 => {}
        0x99 => {}
        0x9a => {}
        0x9b => {}
        0x9c => {}
        0x9d => {}
        0x9e => {}
        0x9f => {}
        0xa0 => {}
        0xa1 => {}
        0xa2 => {}
        0xa3 => {}
        0xa4 => {}
        0xa5 => {}
        0xa6 => {}
        0xa7 => {}
        0xa8 => {}
        0xa9 => {}
        0xaa => {}
        0xab => {}
        0xac => {}
        0xad => {}
        0xae => {}
        0xaf => {}
        0xb0 => {}
        0xb1 => {}
        0xb2 => {}
        0xb3 => {}
        0xb4 => {}
        0xb5 => {}
        0xb6 => {}
        0xb7 => {}
        0xb8 => {}
        0xb9 => {}
        0xba => {}
        0xbb => {}
        0xbc => {}
        0xbd => {}
        0xbe => {}
        0xbf => {}
        0xc0 => {}
        0xc1 => {}
        0xc2 => {}
        0xc3 => {}
        0xc4 => {}
        0xc5 => {}
        0xc6 => {}
        0xc7 => {}
        0xc8 => {}
        _ => {}
    }
}

// impl Instruction for Add {
//     fn execute(&self, store: &mut Store, context: &mut Context) {
//         use Add::*;
//         match self {
//             I32 => todo!(),
//             I64 => todo!(),
//             F32 => todo!(),
//             F64 => todo!(),
//         }
//         // paste! {
//         //     12_f32.[<$name:lower>](14_f32);
//         // }

//         // substack[1].op(substack[0]);
//         // substack.pop();
//     }
//     fn validate(
//         &self,
//         module: &Module,
//         context: &mut Context,
//         vals: &mut ValStack,
//         ctrls: &mut CtrlStack,
//     ) -> validate::Result<()> {
//         use Add::*;
//         match self {
//             I32 => {
//                 vals.pop_expect(ctrls, Some(ValType::I32))?;
//                 vals.pop_expect(ctrls, Some(ValType::I32))?;
//                 vals.push(Some(ValType::I32));
//             }
//             I64 => todo!(),
//             F32 => todo!(),
//             F64 => todo!(),
//         }

//         Ok(())
//     }
// }

// impl Instruction for $name {
//     fn execute(&self, store: &mut Store, context: &mut Context) {
//         paste! {
//             12_f32.[<$name:lower>](14_f32);
//         }

//         // substack[1].op(substack[0]);
//         // substack.pop();
//     }
//     fn validate(
//         &self,
//         module: &Module,
//         context: &mut Context,
//         vals: &mut ValStack,
//         ctrls: &mut CtrlStack,
//     ) {
//         match self {}
//         todo!()
//     }
// }

// impl $name {
//     fn $op() {}
// }

// fn execute(&self, store: &mut Store, context: &mut Context) {
//     use $name::*;
//     match self {
//         I32 => {
//             todo!()
//         }
//         I64 => {
//             todo!()
//         }
//         F32 => {
//             todo!()
//         }
//         F64 => {
//             todo!()
//         }
//     }
// }
