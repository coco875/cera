use hashbrown::HashMap;

use crate::expressions::Expression;

pub enum Visibility {
    Private,
    Public,
    CratePublic,
}

pub struct ContainerType {
    pub fields: HashMap<Box<str>, ContainerField>,
    pub variant: ContainerVariant,
}

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

pub struct ContainerField {
    pub visibility: Visibility,
    pub field_kind: FieldKind,
    pub field_type: Option<Expression>,
    pub val: Expression,
}

pub enum FieldKind {
    Instance,
    Const,
    Static,
}

pub struct IntType {
    pub signed: bool,
    pub bits: u16,
}

pub enum FloatType {
    F16,
    F32,
    F64,
    F80,
    F128,
}

pub struct ComptimeFunctionType {
    pub parameters: Expression,
    pub result: Expression,
}

pub enum CallingConvetion {
    Cera,
}

pub struct RuntimeFunctionType {
    pub base_signature: ComptimeFunctionType,
    pub calling_convention: CallingConvetion,
}

pub struct PointerType {
    pub pointed_value: Expression,
}

type ErrorID = u32;

pub struct ErrorSetType {
    pub possible_errors: Box<[ErrorID]>,
}

pub struct ErrorUnionType {
    pub ok: Box<Type>,
    pub err: ErrorSetType,
}

pub struct OptionType {
    pub some_type: Box<Type>,
}

pub struct ArrayType {
    pub size: usize,
    pub indexed_type: Box<Type>,
}

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
