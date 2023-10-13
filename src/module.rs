use crate::{
    runtime::Store,
    types::{ExternVal, ValType, WasmError},
};

#[derive(Debug)]
struct Mem;

#[derive(Debug)]
struct Data;

#[derive(Debug)]
pub struct Module {
    types: Vec<FuncType>,
    funcs: Vec<Func>,
    tables: Vec<Table>,
    mems: Vec<Mem>,
    globals: Vec<Global>,
    elem: Vec<Elem>,
    data: Vec<Data>,
    // Index of a function
    // https://www.w3.org/TR/wasm-core-1/#start-function
    start: usize,
    imports: Vec<Import>,
    exports: Vec<Export>,
}

#[derive(Debug)]
struct FuncType {}
#[derive(Debug)]
struct Func {}
#[derive(Debug)]
struct Table {}
#[derive(Debug)]
struct Global {
    kind: ValType,
    mutable: bool,
}
#[derive(Debug)]
struct Elem {}

#[derive(Debug)]
struct Import {
    name: String,
    description: ImportExportDescription,
}

#[derive(Debug)]
struct Export {
    module_name: String,
    name: String,
    description: ImportExportDescription,
}

#[derive(Debug)]
struct ModuleInstance {}

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

    fn instantiate(&self, store: &mut Store, external_vals: &[ExternVal]) -> () {
        todo!()
    }
}
