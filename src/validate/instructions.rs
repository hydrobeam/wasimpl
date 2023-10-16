use core::slice;

use crate::instructions::*;
use crate::types::{ValType, ValidationError};
use crate::validate;
use crate::validate::{Locals, Validate, ValidationCtx};
use paste::paste;

macro_rules! truthy_block {
    ($x:block, $y:tt) => {
        $x
    };
}

use super::LabelType;

macro_rules! validate_op {
    ($name:ty, $cmd:tt) => {
        impl Validate for $name {
            fn validate<'module>(
                &self,
                v_ctx: &mut ValidationCtx,
                _context: &mut Locals,
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
            fn validate<'module>(
                &self,
                v_ctx: &mut ValidationCtx,
                _context: &mut Locals,
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
    ($name:ident, ctrl $(, $ident:ident)?) => {
        validate_ctrl!($name $(, $ident)?);
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
    fn validate<'module>(
        &self,
        v_ctx: &mut ValidationCtx,
        _context: &mut Locals,
    ) -> validate::Result<()> {
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
    fn validate<'module>(
        &self,
        v_ctx: &mut ValidationCtx,
        _context: &mut Locals,
    ) -> validate::Result<()> {
        v_ctx.pop_val()?;
        Ok(())
    }
}

impl Validate for Select {
    fn validate<'module>(
        &self,
        v_ctx: &mut ValidationCtx,
        _context: &mut Locals,
    ) -> validate::Result<()> {
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

macro_rules! validate_ctrl {
    ($name:ident $(, $true_:ident)?) => {
        impl Validate for $name {
            fn validate<'module>(
                &'module self,
                v_ctx: &mut ValidationCtx<'module>,
                context: &mut Locals,
            ) -> validate::Result<()> {
                let (in_types, out_types) = match &self.blocktype {
                    BlockType::Idx(idx) => {
                        let func = &v_ctx.module.types[*idx as usize];
                        (func.in_types.as_slice(), func.out_types.as_slice())
                    }
                    // https://stackoverflow.com/questions/55863195/how-to-create-a-slice-from-a-single-element-without-copying-that-element
                    BlockType::ValType(val) => ([].as_slice(), slice::from_ref(val)),
                    BlockType::Void => ([].as_slice(), [].as_slice()),
                };

                v_ctx.pop_vals(in_types)?;
                v_ctx.push_ctrl(LabelType::$name, in_types, out_types)?;

                // if we have an if_block  then prepend true to the instructions
                paste! {
                    for instruction in &self.[<$($true_)? instructions>] {
                        instruction.validate(v_ctx, context)?;
                    }
                }


                // only execute this for if_blocks
                $(truthy_block!(
                    {
                        for instruction in &self.false_instructions {
                            instruction.validate(v_ctx, context)?;
                        }
                    }, $true_);)?

                Ok(())
            }
        }
    };
}

validate_ctrl!(If, true_);
validate_ctrl!(Loop);
validate_ctrl!(Block);

impl Validate for Unreachable {
    fn validate<'module>(
        &self,
        v_ctx: &mut ValidationCtx<'module>,
        _context: &mut Locals,
    ) -> validate::Result<()> {
        v_ctx.unreachable();
        Ok(())
    }
}

impl Validate for Set {
    fn validate<'module>(
        &'module self,
        v_ctx: &mut ValidationCtx<'module>,
        context: &mut Locals,
    ) -> validate::Result<()> {
        let ctx_val: ValType = match self {
            Set::Local { idx } => *context.get(*idx as usize).ok_or(ValidationError::Message {
                msg: "context out of range".into(),
            })?,
            Set::Global { idx } => {
                v_ctx
                    .module
                    .globals
                    .get(*idx as usize)
                    .ok_or(ValidationError::Message {
                        msg: "globals out of range".into(),
                    })?
                    .kind
            }
        };

        v_ctx.pop_val_expect(Some(ctx_val))?;
        Ok(())
    }
}

impl Validate for Get {
    fn validate<'module>(
        &'module self,
        v_ctx: &mut ValidationCtx<'module>,
        context: &mut Locals,
    ) -> validate::Result<()> {
        let val: ValType;
        match self {
            Get::Local { idx } => {
                val = *context.get(*idx as usize).ok_or(ValidationError::Message {
                    msg: "context out of range".into(),
                })?;
            }
            Get::Global { idx } => {
                val = v_ctx
                    .module
                    .globals
                    .get(*idx as usize)
                    .ok_or(ValidationError::Message {
                        msg: "globals out of range".into(),
                    })?
                    .kind;
            }
        }
        v_ctx.push_val(Some(val));
        Ok(())
    }
}

impl Validate for Tee {
    fn validate<'module>(
        &'module self,
        v_ctx: &mut ValidationCtx<'module>,
        context: &mut Locals,
    ) -> validate::Result<()> {
        let ctx_val: ValType;
        ctx_val = *context
            .get(self.idx as usize)
            .ok_or(ValidationError::Message {
                msg: "context out of range".into(),
            })?;
        v_ctx.pop_val_expect(Some(ctx_val))?;
        v_ctx.push_val(Some(ctx_val));
        Ok(())
    }
}

impl Validate for BrIf {
    fn validate<'module>(
        &'module self,
        v_ctx: &mut ValidationCtx<'module>,
        _context: &mut Locals,
    ) -> validate::Result<()> {
        v_ctx.pop_val_expect(Some(ValType::I32))?;
        v_ctx.validate_br_op(self.label_idx)
    }
}

impl Validate for Br {
    fn validate<'module>(
        &'module self,
        v_ctx: &mut ValidationCtx<'module>,
        _context: &mut Locals,
    ) -> validate::Result<()> {
        v_ctx.validate_br_op(self.label_idx)
    }
}
