use crate::values::Value;

pub enum BuiltinFunction {}

impl BuiltinFunction {
    pub fn try_exec(&self, args: &[&Value], context: &mut EvalExecContext) -> Option<Value> {
        todo!()
    }
}

/// A context for evaluating expressions, essentially a stack machine, with scoping rules
pub struct EvalExecContext {}
