use crate::types::{ExternVal, ValType, WasmError};

#[derive(Debug)]
pub struct Mem;

#[derive(Debug)]
pub struct Data;

#[derive(Debug)]
pub struct Module {
    pub types: Vec<FuncType>,
    pub funcs: Vec<Func>,
    pub tables: Vec<Table>,
    pub mems: Vec<Mem>,
    pub globals: Vec<Global>,
    pub elem: Vec<Elem>,
    pub data: Vec<Data>,
    // Index of a function
    // https://www.w3.org/TR/wasm-core-1/#start-function
    pub start: usize,
    pub imports: Vec<Import>,
    pub exports: Vec<Export>,
}

#[derive(Debug)]
pub struct FuncType {}
#[derive(Debug)]
pub struct Func {}
#[derive(Debug)]
pub struct Table {}
#[derive(Debug)]
pub struct Global {
    kind: ValType,
    mutable: bool,
}
#[derive(Debug)]
pub struct Elem {}

#[derive(Debug)]
pub struct Import {
    name: String,
    description: ImportExportDescription,
}

#[derive(Debug)]
pub struct Export {
    module_name: String,
    name: String,
    description: ImportExportDescription,
}

#[derive(Debug)]
pub struct ModuleInstance {}

// There is one difference between the descriptions for in/out, but we
// ignore it (the spec says embeddings can choose not to implement it.)
// refer to https://www.w3.org/TR/wasm-core-1/#imports
/// Import/export description.
/// Each value represents an ID for the respective environment.
#[derive(Debug)]
enum ImportExportDescription {
    Func(u32),
    Table(u32),
    Mem(u32),
    Global(u32),
}

impl Module {
    fn decode(bytes: &[u8]) -> Result<Self, WasmError> {
        todo!()
    }

    fn parse(chars: &str) -> Result<Self, WasmError> {
        unimplemented!("please convert the text to binary")
    }

    fn validate(&self) -> Result<(), WasmError> {
        todo!()
    }

    // fn instantiate(&self, store: &mut Store, external_vals: &[ExternVal]) -> () {
    //     todo!()
    // }
}
