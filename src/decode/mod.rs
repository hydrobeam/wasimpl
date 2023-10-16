use std::io::Read;
use std::num::TryFromIntError;

use crate::instructions::*;
use crate::types::{ValType, ValidationError};

const END_CODE: u8 = 0x0B;
const ELSE_CODE: u8 = 0x05;

pub enum DecodeError {
    Msg(String),
    NoMoreBytes,
    FailedByteConversion,
    IntegerOverflow,
    Reserved(u8),
}

impl From<TryFromIntError> for DecodeError {
    fn from(value: TryFromIntError) -> Self {
        Self::FailedByteConversion
    }
}

impl From<leb128::read::Error> for DecodeError {
    fn from(value: leb128::read::Error) -> Self {
        match value {
            leb128::read::Error::IoError(_) => DecodeError::NoMoreBytes,
            leb128::read::Error::Overflow => DecodeError::IntegerOverflow,
        }
    }
}

pub type Result<T> = std::result::Result<T, DecodeError>;

pub struct Decoder<'buf> {
    byte_buf: &'buf [u8],
    index: usize,
}

impl Read for Decoder<'_> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        for i in 0..buf.len() {
            if let Ok(val) = self.consume_byte() {
                buf[i] = val;
            } else {
                return Ok(i);
            }
        }

        Ok(buf.len())
    }
}

impl<'buf> Decoder<'buf> {
    pub fn len(&self) -> usize {
        self.byte_buf.len()
    }

    pub fn curr_byte(&self) -> u8 {
        self.byte_buf[self.index]
    }

    pub fn prev_byte(&self) -> u8 {
        self.byte_buf[self.index - 1]
    }

    pub fn try_byte(&self) -> Result<u8> {
        self.byte_buf
            .get(self.index)
            .copied()
            .ok_or(DecodeError::NoMoreBytes)
    }

    pub fn peek_byte(&self) -> Result<u8> {
        self.byte_buf
            .get(self.index + 1)
            .copied()
            .ok_or(DecodeError::NoMoreBytes)
    }

    pub fn consume_byte(&mut self) -> Result<u8> {
        let ret = self.try_byte()?;
        self.index += 1;
        Ok(ret)
    }

    fn read4_bytes(&self) -> Result<u32> {
        if self.len() <= self.index + 3 {
            Err(DecodeError::NoMoreBytes)
        } else {
            Ok(u32::from_le_bytes([
                self.byte_buf[self.index],
                self.byte_buf[self.index + 1],
                self.byte_buf[self.index + 2],
                self.byte_buf[self.index + 3],
            ]))
        }
    }

    fn read8_bytes(&self) -> Result<u64> {
        if self.len() <= self.index + 7 {
            Err(DecodeError::NoMoreBytes)
        } else {
            Ok(u64::from_le_bytes([
                self.byte_buf[self.index],
                self.byte_buf[self.index + 1],
                self.byte_buf[self.index + 2],
                self.byte_buf[self.index + 3],
                self.byte_buf[self.index + 4],
                self.byte_buf[self.index + 5],
                self.byte_buf[self.index + 6],
                self.byte_buf[self.index + 7],
            ]))
        }
    }

    pub fn read_memarg(&mut self) -> Result<MemArg> {
        todo!()
    }

    pub fn next(&mut self) {
        self.index += 1;
    }
    pub fn advance(&mut self, num: usize) {
        self.index += num;
    }
}

// helper functions
impl<'buf> Decoder<'buf> {
    pub fn read_string(&mut self) -> Result<&'buf str> {
        let len = leb128::read::unsigned(self)?;

        Ok(std::str::from_utf8(self.read_bytes(len as usize)?)
            .map_err(|x| DecodeError::Msg("Invalid utf-8 encoding".into()))?)
    }

    pub fn read_bytes(&mut self, len: usize) -> Result<&'buf [u8]> {
        if self.len() <= self.index + len {
            Err(DecodeError::NoMoreBytes)
        } else {
            Ok(&self.byte_buf[self.index..(self.index + len)])
        }
    }
    pub fn read_i32(&mut self) -> Result<i32> {
        let val = leb128::read::signed(self)?;
        Ok(i32::try_from(val)?)
    }
    pub fn read_i64(&mut self) -> Result<i64> {
        Ok(leb128::read::signed(self)?)
    }
    pub fn read_f32(&mut self) -> Result<f32> {
        Ok(f32::from_bits(self.read4_bytes()?))
    }
    pub fn read_f64(&mut self) -> Result<f64> {
        Ok(f64::from_bits(self.read8_bytes()?))
    }
    pub fn read_s33(&mut self) -> Result<i64> {
        Ok(leb128::read::signed(self)?)
    }

    pub fn read_blocktype(&mut self) -> Result<BlockType> {
        let b = self.consume_byte()?;

        if b == 0x40 {
            Ok(BlockType::Void)
        } else if let Ok(res) = ValType::from_byte(b) {
            Ok(BlockType::ValType(res))
        } else {
            let s33 = self.read_s33()?;
            Ok(BlockType::Idx(u32::try_from(s33)?))
        }
    }
}

const fn stop_cond(x: u8) -> bool {
    x == END_CODE || x == ELSE_CODE
}

impl<'buf> Decoder<'buf> {
    fn decode_ops(&mut self, instruction_buf: &mut Vec<Box<dyn Instruction>>) -> Result<()> {
        let mut op = self.consume_byte()?;
        while !stop_cond(op) {
            instruction_buf.push(match op {
                // unreachable
                0x00 => Box::new(Unreachable),
                // nop
                0x01 => continue,
                // block
                0x02 => {
                    let blocktype = self.read_blocktype()?;
                    let mut instructions: Vec<Box<dyn Instruction>> = Vec::new();
                    self.decode_ops(&mut instructions)?;
                    if self.prev_byte() == ELSE_CODE {
                        Err(DecodeError::Msg("else in non-if statement".into()))?
                    } else {
                        Box::new(Block {
                            blocktype,
                            instructions,
                        })
                    }
                }
                // loop
                0x03 => {
                    let blocktype = self.read_blocktype()?;
                    let mut instructions: Vec<Box<dyn Instruction>> = Vec::new();
                    self.decode_ops(&mut instructions)?;
                    if self.prev_byte() == ELSE_CODE {
                        Err(DecodeError::Msg("else in non-if statement".into()))?
                    } else {
                        Box::new(Loop {
                            blocktype,
                            instructions,
                        })
                    }
                }
                // if
                0x04 => {
                    let blocktype = self.read_blocktype()?;
                    let mut true_instructions: Vec<Box<dyn Instruction>> = Vec::new();
                    let mut false_instructions: Vec<Box<dyn Instruction>> = Vec::new();
                    self.decode_ops(&mut true_instructions)?;
                    if self.prev_byte() == ELSE_CODE {
                        self.next();
                        self.decode_ops(&mut false_instructions)?;
                    }
                    Box::new(If {
                        blocktype,
                        true_instructions,
                        false_instructions,
                    })
                }
                // else
                0x05 => {
                    unreachable!("handled in stop condition")
                }
                // exception handling proposal, unimplemented
                0x06 | 0x07 | 0x08 | 0x09 | 0x0a => {
                    unimplemented!("the exception handling proposal is not supported")
                }
                //
                // end
                0x0b => {
                    unreachable!("handled in stop condition")
                }
                //br
                0x0c => Box::new(Br {
                    label_idx: self.read4_bytes()?,
                }),
                //br_if
                0x0d => Box::new(BrIf {
                    label_idx: self.read4_bytes()?,
                }),
                // br_table
                0x0e => {}
                // return
                0x0f => {}
                // call
                0x10 => {}
                // call_indirect
                0x11 => {}
                // return_call
                0x12 => {}
                // return_call_indirect
                0x13 => {}
                // call_ref
                0x14 => {}
                // return_call_ref
                0x15 => {}
                // reserved
                0x16 => {}
                0x17 => {}
                //
                0x18 | 0x19 => {
                    unimplemented!("exception handling proposal")
                }
                // drop
                0x1a => Box::new(Drop),
                // select
                0x1b => Box::new(Select { val: None }),
                // select t
                0x1c => {
                    let len = leb128::read::unsigned(self)?;
                    if len != 1 {
                        Err(DecodeError::Msg("invalid select".into()));
                    }
                    let t = ValType::from_byte(self.consume_byte()?)?;
                    Box::new(Select { val: Some(t) })
                }
                // reserved
                a @ 0x1d | a @ 0x1e | a @ 0x1f => Err(DecodeError::Reserved(a))?,
                // local.get
                0x20 => {
                    let idx = self.read4_bytes()?;
                    Box::new(Get::Local { idx })
                }
                0x21 => {
                    let idx = self.read4_bytes()?;
                    Box::new(Set::Local { idx })
                }
                0x22 => {
                    let idx = self.read4_bytes()?;
                    Box::new(Tee { idx })
                }
                0x23 => {
                    let idx = self.read4_bytes()?;
                    Box::new(Get::Global { idx })
                }
                0x24 => {
                    let idx = self.read4_bytes()?;
                    Box::new(Set::Global { idx })
                }
                // table.get
                // table.set
                0x25 | 0x26 => {
                    unimplemented!("Access tables proposal")
                }
                // reserved
                a @ 0x27 => DecodeError::Reserved(a),
                // numerics
                0x28 => Box::new(Load::I32(self.read_memarg()?)),
                0x29 => Box::new(Load::I64(self.read_memarg()?)),
                0x2a => Box::new(Load::F32(self.read_memarg()?)),
                0x2b => Box::new(Load::F64(self.read_memarg()?)),
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
            });

            op = self.consume_byte()?;
        }

        todo!()
    }
}
