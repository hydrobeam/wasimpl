// #![feature(concat_idents)]

use types::WasmError;

mod decode;
mod execution;
mod instructions;
mod module;
mod runtime;
mod types;
mod validate;

pub fn run() -> Result<(), WasmError> {
    todo!()
}
