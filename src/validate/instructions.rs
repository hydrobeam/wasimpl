use crate::instructions::Instruction;
use crate::instructions::*;
use crate::types::ValidationError;
use crate::validate;
use crate::validate::{Context, ValType, Validate, ValidationCtx};
use paste::paste;

macro_rules! validate_op {
    ($name:ty, $cmd:tt) => {
        impl Validate for $name {
            fn validate(
                &self,
                v_ctx: &mut ValidationCtx,
                _context: &mut Context,
            ) -> validate::Result<()> {
                paste! {
                    v_ctx.[<validate_ $cmd _op>](Some(self.to_valtype()))
                }
            }
        }
    };
}

macro_rules! validate_mem {
    ($name:ty) => {
        impl Validate for $name {
            fn validate(
                &self,
                v_ctx: &mut ValidationCtx,
                _context: &mut Context,
            ) -> validate::Result<()> {
                v_ctx.validate_mem_op(Some(self.to_valtype()), self.memarg())
            }
        }
    };
}

macro_rules! op_choose {
    ($name:ty, mem) => {
        validate_mem!($name);
    };
    ($name:ident, $cmd:ident) => {
        validate_op!($name, $cmd);
    };
}

op_choose!(Div, binary);
op_choose!(Add, binary);
op_choose!(Sub, binary);
op_choose!(Mul, binary);

op_choose!(Gt, boolean);
op_choose!(Ge, boolean);
op_choose!(Lt, boolean);
op_choose!(Le, boolean);

op_choose!(WasmEq, boolean);
op_choose!(Ne, boolean);

op_choose!(Const, push);

// // duals
op_choose!(Eqz, single_boolean);
op_choose!(Xor, binary);
op_choose!(Rotl, binary);
op_choose!(Rotr, binary);

op_choose!(Clz, single);
op_choose!(Ctz, single);
op_choose!(Popcnt, single);

op_choose!(And, binary);
op_choose!(Or, binary);
op_choose!(Shl, binary);
op_choose!(Shr, binary);
op_choose!(Rem, binary);

// floats
op_choose!(Min, single);
op_choose!(Max, single);
op_choose!(CopySign, single);
op_choose!(Abs, single);
op_choose!(Neg, single);
op_choose!(Ceil, single);
op_choose!(Floor, single);
op_choose!(Nearest, single);
op_choose!(Sqrt, single);

op_choose!(Load, mem);
op_choose!(Load8, mem);
op_choose!(Load16, mem);
op_choose!(Load32, mem);

op_choose!(Store, mem);
op_choose!(Store8, mem);
op_choose!(Store16, mem);
op_choose!(Store32, mem);

impl Validate for Memory {
    fn validate(&self, v_ctx: &mut ValidationCtx, context: &mut Context) -> validate::Result<()> {
        match self {
            Memory::Grow => v_ctx.validate_single_op(Some(ValType::I32)),
            Memory::Size => v_ctx.validate_push_op(Some(ValType::I32)),
            Memory::Fill | Memory::Copy => v_ctx
                .pop_vals(&[ValType::I32, ValType::I32, ValType::I32])
                // HACK: is there a more idiomatic way to do this
                .map(|_| ()),
            Memory::Init { dataidx } => {
                v_ctx
                    .module
                    .data
                    .get(*dataidx as usize)
                    .ok_or(ValidationError::Message {
                        msg: format!("dataidx: `{}` not available for memory.init call", dataidx),
                    })?;
                v_ctx.pop_vals(&[ValType::I32, ValType::I32, ValType::I32])?;
                Ok(())
            }
        }
    }
}

impl Validate for crate::instructions::Drop {
    fn validate(&self, v_ctx: &mut ValidationCtx, context: &mut Context) -> validate::Result<()> {
        v_ctx.pop_val().map(|_| ())
    }
}

impl Validate for Select {
    fn validate(&self, v_ctx: &mut ValidationCtx, context: &mut Context) -> validate::Result<()> {
        v_ctx.pop_val_expect(Some(ValType::I32))?;
        if self.val.is_some() {
            v_ctx.pop_val_expect(self.val)?;
            v_ctx.pop_val_expect(self.val)?;
            v_ctx.push_val(self.val);
        } else {
            let t1 = v_ctx.pop_val_expect(self.val)?;
            let t2 = v_ctx.pop_val_expect(self.val)?;

            match (t1, t2) {
                (None, None) => v_ctx.push_val(t2),
                (None, Some(_)) => Err(ValidationError::Catastrophic)?,
                (Some(_), None) => Err(ValidationError::Catastrophic)?,
                (Some(i1), Some(i2)) => {
                    if (i1.is_num() && i2.is_num()) || (i1.is_vec() || i2.is_vec() || i1 != i2) {
                        v_ctx.push_val(t1);
                    } else {
                        Err(ValidationError::Catastrophic)?
                    }
                }
            }
        };
        Ok(())
    }
}
