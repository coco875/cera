use crate::{
    expressions::Expression,
    types::{
        ArrayType, ComptimeFunctionType, ContainerType, ErrorSetType, ErrorUnionType, FloatType,
        IntType, OptionType, PointerType, Type,
    },
};

#[derive(Debug, Clone)]
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

impl Value {
    pub fn get_type(&self) -> Type {
        match self {
            Value::Float(Float { float_type: _, .. }) => todo!(),
            Value::Int(Int { int_type: _, .. }) => todo!(),
            Value::Container(Container { container_type: _, .. }) => todo!(),
            Value::Pointer(Pointer { pointer_type: _, .. }) => todo!(),
            Value::Error(Error { error_set_type: _, .. }) => todo!(),
            Value::ErrorUnion(ErrorUnion {
                error_union_type: _, ..
            }) => todo!(),
            Value::Option(OptionValue { option_type: _, .. }) => todo!(),
            Value::Array(Array { array_type: _, .. }) => todo!(),
            Value::Function(Function { function_type, .. }) => {
                Type::ComptimeFunction(function_type.clone())
            }
            Value::Type(_) => Type::Type,
            Value::Void => Type::Void,
            Value::Undefined => Type::Undefined,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Float {
    pub bytes: Box<[u8]>,
    pub float_type: FloatType,
}

#[derive(Debug, Clone)]
pub struct Int {
    pub bytes: Box<[u8]>,
    pub int_type: IntType,
}

#[derive(Debug, Clone)]
pub struct Container {
    pub bytes: Box<[u8]>,
    pub container_type: ContainerType,
}

#[derive(Debug, Clone)]
pub struct Pointer {
    pub bytes: Box<[u8]>,
    pub pointer_type: PointerType,
}

#[derive(Debug, Clone)]
pub struct Error {
    pub bytes: Box<[u8]>,
    pub error_set_type: ErrorSetType,
}

#[derive(Debug, Clone)]
pub struct ErrorUnion {
    pub bytes: Box<[u8]>,
    pub error_union_type: ErrorUnionType,
}

#[derive(Debug, Clone)]
pub struct OptionValue {
    pub bytes: Box<[u8]>,
    pub option_type: OptionType,
}

#[derive(Debug, Clone)]
pub struct Array {
    pub bytes: Box<[u8]>,
    pub array_type: ArrayType,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub expression: Expression,
    pub function_type: ComptimeFunctionType,
}
