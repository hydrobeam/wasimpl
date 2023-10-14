use crate::instructions::*;
use crate::validate;
use crate::validate::{Context, ValType, Validate, ValidationCtx};

// boolean truth, return the first value
macro_rules! truth {
    ($x:tt, $y:tt) => {
        $x
    };
}

// boolean truth, return the first value
macro_rules! ipat {
    // (s, $x:pat, $y:pat) => {
    //     $x
    // };
    // (n, $x:pat, $y:pat) => {
    //     $y
    // };
    (f,

    ) => {};
}

macro_rules! fpat {
    (f, $x:pat, $y:pat) => {
        _
    };
    ($a:ident, $x:pat, $y:pat) => {
        $x
    };
}

// macro_rules! if_float {
//     (s, $q:pat) => {
//         $q
//     };
//     (f, $q:pat) => {
//         _
//     };
// }

macro_rules! validate_op {
    ($name:ty, $cmd:tt, $s:ident) => {
        impl Validate for $name {
            fn validate(
                &self,
                v_ctx: &mut ValidationCtx,
                _context: &mut Context,
            ) -> validate::Result<()> {
                v_ctx.$cmd(Some(self.to_valtype()))
            }
        }
    };
}

macro_rules! op_choose {
    ($name:ident, boolean) => {
        validate_op!($name, validate_boolean_op);
    };
    ($name:ident, boolean, signed) => {
        validate_op!($name, validate_boolean_op);
    };
    ($name:ty, binary) => {
        validate_op!($name, validate_binary_op);
    };
    ($name:ty, binary, signed) => {
        validate_op!($name, validate_binary_op);
    };
    (Const) => {
        validate_op!(Const, validate_const);
    };
    ($name:ty, float) => {
        validate_op!($name, validate_binary_op);
    };
}

op_choose!(Div, binary, signed);
op_choose!(Add, binary);
op_choose!(Sub, binary);
op_choose!(Mul, binary);

op_choose!(Gt, boolean, signed);
op_choose!(Ge, boolean, signed);
op_choose!(Lt, boolean, signed);
op_choose!(Le, boolean, signed);

op_choose!(WasmEq, boolean);
op_choose!(Ne, boolean);

op_choose!(Const);

// // duals
op_choose!(Eqz, binary);
op_choose!(Xor, binary);
op_choose!(Rotl, binary);
op_choose!(Rotr, binary);
op_choose!(Clz, binary);
op_choose!(Ctz, binary);
op_choose!(Popcnt, binary);
op_choose!(And, binary);
op_choose!(Or, binary);

op_choose!(Shr, binary);

// floats
op_choose!(Min, float);
op_choose!(Max, float);
op_choose!(CopySign, float);
op_choose!(Abs, float);
op_choose!(Neg, float);
op_choose!(Ceil, float);
op_choose!(Floor, float);
op_choose!(Nearest, float);
op_choose!(Sqrt, float);

// op_choose!(Rotr, binary, signed);

// impl Validate for Eqz {
//     fn validate(&self, v_ctx: &mut ValidationCtx, _context: &mut Context) -> validate::Result<()> {
//         match self {
//             Eqz::I32 => {
//                 v_ctx.pop_val_expect(Some(ValType::I32))?;
//             }
//             Eqz::I64 => {
//                 v_ctx.pop_val_expect(Some(ValType::I64))?;
//             }
//         }
//         v_ctx.push_val(Some(ValType::I32));
//         Ok(())
//     }
// }
