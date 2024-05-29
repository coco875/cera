use crate::values::Value;

#[derive(Debug, Clone)]
pub enum BuiltinFunction {
    TypeOf,
}

impl BuiltinFunction {
    pub fn try_exec(&self, args: &[&Value], _context: &mut EvalExecScope) -> Option<Value> {
        match self {
            BuiltinFunction::TypeOf => {
                if args.len() != 1 {
                    return None;
                }
                return Some(Value::Type(args[0].get_type()));
            }
        }
    }
}

/// A context for evaluating expressions, essentially a stack machine, with scoping rules
pub struct EvalExecScope<'t> {
    pub super_scope: Option<&'t EvalExecScope<'t>>,
}

impl<'t> EvalExecScope<'t> {
    pub fn new_scope(&'t self) -> EvalExecScope<'t> {
        Self {
            super_scope: Some(self),
        }
    }
}
