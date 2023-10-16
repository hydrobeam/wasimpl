// validation entities

mod instructions;

pub type Result<T> = std::result::Result<T, ValidationError>;
use std::ops::{Deref, DerefMut, Index, IndexMut};

use crate::types::ValType;

use crate::types::{Locals, ValidationError};
use crate::module::Module;
use crate::instructions::MemArg;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LabelType {
    Block,
    Loop,
    If,
    Else,
}

#[derive(Debug, Clone, Copy)]
pub struct CtrlFrame<'a> {
    opcode: LabelType,
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
pub struct ValidationCtx<'module> {
    module: &'module Module,
    ctrls: CtrlStack<'module>,
    vals: ValStack,
}

impl<'module> ValidationCtx<'module> {
    pub fn validate_br_op(&mut self, label_idx: u32) -> Result<()> {
        if self.ctrls.len() < label_idx as usize {
            Err(ValidationError::Message { msg: "bad".into() })
        } else {
            // clones
            let temp = self.ctrls[label_idx as usize];
            let lt = ValidationCtx::label_types(&temp);

            self.pop_vals(lt)?;
            self.push_vals(lt);

            Ok(())
        }
    }

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

    pub fn validate_single_boolean_op(&mut self, val: Option<ValType>) -> Result<()> {
        self.pop_val_expect(val)?;
        self.push_val(Some(ValType::I32));
        Ok(())
    }

    // a little redundant (does one thing), but nice for macros / consistency
    pub fn validate_push_op(&mut self, val: Option<ValType>) -> Result<()> {
        self.push_val(val);
        Ok(())
    }

    pub fn validate_single_op(&mut self, val: Option<ValType>) -> Result<()> {
        self.pop_val_expect(val)?;
        self.push_val(val);
        Ok(())
    }

    pub fn validate_mem_op(&mut self, val: Option<ValType>, memarg: MemArg) -> Result<()> {
        // TODO: use memargs
        self.pop_val_expect(val)?;
        self.push_val(val);
        Ok(())
    }

    pub fn validate_else_op(&mut self) -> Result<()> {
        // very hacky, but we run into double mutable borrows
        // if we allow pop_ctrl do its thang
        let frame = self.ctrls[0];
        self.pop_ctrl()?;
        if LabelType::If != frame.opcode {
            Err(ValidationError::Message {
                msg: "use of `else` in non-if control instruction".to_string(),
            })?
        };
        self.push_ctrl(LabelType::Else, frame.start_types, frame.end_types)?;

        Ok(())
    }

    pub fn validate_end_op(&mut self) -> Result<()> {
        let frame = self.ctrls[0];
        self.pop_ctrl()?;
        self.push_vals(frame.end_types);
        Ok(())
    }
}

impl<'module> ValidationCtx<'module> {
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
            if let Some(_) = expect {
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
        opcode: LabelType,
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

    pub fn pop_ctrl(&mut self) -> Result<()> {
        if self.ctrls.is_empty() {
            Err(ValidationError::Catastrophic)?
        }
        let frame = &self.ctrls[0];
        let h = frame.height;
        self.pop_vals(frame.end_types)?;

        if self.vals.len() != h {
            Err(ValidationError::InvalidDepth {
                max_depth: self.vals.len() as u8,
                got_depth: h as u8,
            })?
        }
        // REVIEW: borrow checking error when returning frame var,
        // don't know how to resolve.
        Ok(())
    }

    pub fn label_types(frame: &'a CtrlFrame) -> &'a [ValType] {
        if let LabelType::Loop = frame.opcode {
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
    fn validate<'module>(
        &'module self,
        v_ctx: &mut ValidationCtx<'module>,
        context: &mut Locals,
    ) -> Result<()>;
}
