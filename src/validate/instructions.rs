macro_rules! validate_binary {
    ($self:ident, $v_ctx:ident) => {
        match $self {
            I32 => $v_ctx.validate_binary_op(Some(ValType::I32)),
            I64 => $v_ctx.validate_binary_op(Some(ValType::I64)),
            F32 => $v_ctx.validate_binary_op(Some(ValType::F32)),
            F64 => $v_ctx.validate_binary_op(Some(ValType::F64)),
        }
    };
}

macro_rules! validate_binary_signed {
    ($self:ident, $v_ctx:ident) => {
        match $self {
            I32 | U32 => $v_ctx.validate_binary_op(Some(ValType::I32)),
            I64 | U64 => $v_ctx.validate_binary_op(Some(ValType::I64)),
            F32 => $v_ctx.validate_binary_op(Some(ValType::F32)),
            F64 => $v_ctx.validate_binary_op(Some(ValType::F64)),
        }
    };
}
