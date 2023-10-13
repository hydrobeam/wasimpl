// validation entities

mod instructions;

pub type Result<T> = std::result::Result<T, ValidationError>;
use std::ops::{Deref, DerefMut, Index, IndexMut};

use crate::{module::Module, runtime::Context, types::ValidationError};

#[derive(Debug, Clone, Copy)]
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
    fn is_num(self) -> bool {
        matches!(
            self,
            ValType::F32 | ValType::F64 | ValType::I32 | ValType::I64
        )
    }
    fn is_vec(self) -> bool {
        matches!(self, ValType::V128)
    }

    fn is_ref(self) -> bool {
        matches!(self, ValType::FuncRef | ValType::ExternRef)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CtrlFrame<'a> {
    opcode: u32,
    start_types: &'a [ValType],
    end_types: &'a [ValType],
    height: usize,
    unreachable: bool,
}

#[derive(Debug)]
pub struct CtrlStack<'a>(Vec<CtrlFrame<'a>>);

pub type ValStack = Vec<Option<ValType>>;

impl<'a> CtrlStack<'a> {
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl<'a> Index<usize> for CtrlStack<'a> {
    type Output = CtrlFrame<'a>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[(self.len() - 1) - index]
    }
}

impl<'a> IndexMut<usize> for CtrlStack<'a> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let val = (self.len() - 1) - index;
        &mut self.0[val]
    }
}

impl<'a> Deref for CtrlStack<'a> {
    type Target = Vec<CtrlFrame<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for CtrlStack<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug)]
pub struct ValidationCtx<'a> {
    module: Module,
    ctrls: CtrlStack<'a>,
    vals: ValStack,
}

impl<'a> ValidationCtx<'a> {
    pub fn validate_binary_op(&mut self, val: Option<ValType>) -> Result<()> {
        self.pop_val_expect(val)?;
        self.pop_val_expect(val)?;
        self.push_val(val);
        Ok(())
    }

    pub fn validate_boolean_op(&mut self, val: Option<ValType>) -> Result<()> {
        self.pop_val_expect(val)?;
        self.pop_val_expect(val)?;
        self.push_val(Some(ValType::I32));
        Ok(())
    }
}

impl<'a> ValidationCtx<'a> {
    pub fn len_vals(&self) -> usize {
        self.vals.len()
    }
    pub fn push_val(&mut self, val: Option<ValType>) {
        self.vals.push(val);
    }
    pub fn pop_val(&mut self) -> Result<Option<ValType>> {
        let underflow = self.len_vals() == self.ctrls[0].height;
        if underflow && self.ctrls[0].unreachable {
            Ok(None)
        } else if underflow {
            Err(ValidationError::LimitExceeded {
                msg: String::from("hi"),
            })
        } else {
            Ok(self.vals.pop().ok_or(ValidationError::Catastrophic)?)
        }
    }

    pub fn pop_val_expect(&mut self, expect: Option<ValType>) -> Result<Option<ValType>> {
        let actual = self.pop_val()?;
        // FIXME: double writing
        if let Some(in_acc) = actual {
            if let Some(in_expect) = expect {
                Ok(Some(in_acc))
            } else {
                Err(ValidationError::TypeMismatch {
                    location: "?".to_string(),
                    expected: "?".to_string(),
                    got: "?".to_string(),
                })
            }
        } else {
            Err(ValidationError::TypeMismatch {
                location: "?".to_string(),
                expected: "?".to_string(),
                got: "?".to_string(),
            })
        }
    }

    pub fn push_vals(&mut self, vals: &[ValType]) {
        for val in vals {
            self.push_val(Some(*val));
        }
    }

    pub fn pop_vals(&mut self, vals: &[ValType]) -> Result<ValStack> {
        let mut popped = Vec::with_capacity(vals.len());
        for val in vals.iter().rev() {
            popped.push(self.pop_val_expect(Some(*val))?);
        }
        popped.reverse();
        Ok(popped)
    }
}

/// Implementation of CtrlStack methods
impl<'a> ValidationCtx<'a> {
    pub fn push_ctrl(
        &mut self,
        opcode: u32,
        start_types: &'a [ValType],
        end_types: &'a [ValType],
    ) -> Result<()> {
        self.ctrls.push(CtrlFrame {
            opcode,
            start_types,
            end_types,
            height: self.vals.len(),
            unreachable: false,
        });
        self.pop_vals(start_types)?;
        Ok(())
    }

    pub fn pop_ctrl(&mut self, vals: &mut ValStack) -> Result<&'a CtrlFrame> {
        if self.ctrls.is_empty() {
            Err(ValidationError::Catastrophic)?
        }
        let frame = &self.ctrls[0];
        let h = frame.height;
        self.pop_vals(frame.end_types)?;

        if vals.len() != h {
            Err(ValidationError::InvalidDepth {
                max_depth: vals.len() as u8,
                got_depth: h as u8,
            })?
        }
        // REVIEW: borrow checking error when returning frame var,
        // don't know how to resolve.
        Ok(&self.ctrls[0])
    }

    pub fn label_types(frame: &'a CtrlFrame) -> &'a [ValType] {
        // FIXME: loop
        if frame.opcode == 0x13 {
            frame.start_types
        } else {
            frame.end_types
        }
    }

    pub fn unreachable(&mut self) {
        self.vals.truncate(self.ctrls[0].height);
        self.ctrls[0].unreachable = true;
    }
}

pub trait Validate {
    fn validate(&self, v_ctx: &mut ValidationCtx, context: &mut Context) -> Result<()>;
}
