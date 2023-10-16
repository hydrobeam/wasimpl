use std::ops::{Deref, DerefMut, Index, IndexMut};

// #[derive(Debug)]
// pub struct Store {}
// pub enum StackVal {
//     Num(NumType),
//     Label { arity: u8 },
//     ActivationFrame { arity: u8 },
// }

// struct Context  {
//     arity:
// }

#[derive(Debug)]
pub enum StackVal {
    I32(i32),
    I64(f32),
    F32(i64),
    F64(f64),
}

#[derive(Debug)]
pub struct Stack(Vec<StackVal>);

impl Deref for Stack {
    type Target = Vec<StackVal>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Stack {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Index<usize> for Stack {
    type Output = StackVal;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[(self.len() - 1) - index]
    }
}

impl IndexMut<usize> for Stack {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let val = (self.len() - 1) - index;
        &mut self.0[val]
    }
}
