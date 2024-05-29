use hashbrown::HashMap;

use crate::expressions::Expression;

#[derive(Debug, Clone)]
pub enum Visibility {
    Private,
    Public,
}

#[derive(Debug, Clone)]
pub struct ContainerType {
    pub fields: HashMap<Box<str>, ContainerField>,
    pub variant: ContainerVariant,
}

#[derive(Debug, Clone)]
pub enum ContainerVariant {
    Struct {
        ptr_coerce_target: Option<Box<str>>,
        droppers: Box<[Box<str>]>,
    },
    Trait {
        ptr_coerce_target: Option<Box<str>>,
    },
    Union,
    Enum {
        backing_type: IntType,
        is_exhaustive: bool,
    },
}

#[derive(Debug, Clone)]
pub struct ContainerField {
    pub visibility: Visibility,
    pub field_kind: FieldKind,
    pub field_type: Option<Expression>,
    pub val: Expression,
}

#[derive(Debug, Clone)]
pub enum FieldKind {
    Instance,
    Const,
    Static,
}

#[derive(Debug, Clone)]
pub struct IntType {
    pub signed: bool,
    pub bits: u16,
}

#[derive(Debug, Clone)]
pub enum FloatType {
    F16,
    F32,
    F64,
    F80,
    F128,
}

#[derive(Debug, Clone)]
pub struct ComptimeFunctionType {
    pub parameters: Expression,
    pub result: Expression,
}

#[derive(Debug, Clone)]
pub enum CallingConvetion {
    Cera,
}

#[derive(Debug, Clone)]
pub struct RuntimeFunctionType {
    pub base_signature: ComptimeFunctionType,
    pub calling_convention: CallingConvetion,
}

#[derive(Debug, Clone)]
pub struct PointerType {
    pub pointed_value: Expression,
}

type ErrorID = u32;

#[derive(Debug, Clone)]
pub struct ErrorSetType {
    pub possible_errors: Box<[ErrorID]>,
}

#[derive(Debug, Clone)]
pub struct ErrorUnionType {
    pub ok: Box<Type>,
    pub err: ErrorSetType,
}

#[derive(Debug, Clone)]
pub struct OptionType {
    pub some_type: Box<Type>,
}

#[derive(Debug, Clone)]
pub struct ArrayType {
    pub size: usize,
    pub indexed_type: Box<Type>,
}

#[derive(Debug, Clone)]
pub enum Type {
    Container(ContainerType),
    Int(IntType),
    Float(FloatType),
    Pointer(PointerType),
    ErrorSet(ErrorSetType),
    ErrorUnion(ErrorUnionType),
    Option(OptionType),
    Array(ArrayType),
    Type,
    ComptimeFunction(ComptimeFunctionType),
    RuntimeFunction(RuntimeFunctionType),
    Void,
    Undefined,
}
