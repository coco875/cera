use crate::{
    expressions::Expression,
    types::{
        ArrayType, ComptimeFunctionType, ContainerType, ErrorSetType, FloatType, IntType,
        PointerType, Type,
    },
};

pub enum Value {
    Float(Float),
    Int(Int),
    Container(Container),
    Pointer(Pointer),
    Error(Error),
    ErrorUnion(ErrorUnion),
    Option(OptionValue),
    Array(Array),
    Function(Function),
    Type(Type),
    Void,
    Undefined,
}

pub struct Float {
    pub bytes: Box<[u8]>,
    pub float_type: FloatType,
}

pub struct Int {
    pub bytes: Box<[u8]>,
    pub int_type: IntType,
}

pub struct Container {
    pub bytes: Box<[u8]>,
    pub container_type: ContainerType,
}

pub struct Pointer {
    pub bytes: Box<[u8]>,
    pub pointer_type: PointerType,
}

pub struct Error {
    pub bytes: Box<[u8]>,
    pub error_set: ErrorSetType,
}

pub struct ErrorUnion {
    pub result: Result<Box<Value>, Error>,
}

pub struct OptionValue {
    pub value: Option<Box<Value>>,
}

pub struct Array {
    pub bytes: Box<[u8]>,
    pub array_type: ArrayType,
}

pub struct Function {
    pub expression: Expression,
    pub function_type: ComptimeFunctionType,
}
