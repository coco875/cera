use crate::{
    builtin::{BuiltinFunction, EvalExecScope},
    values::Value,
};

#[derive(Debug, Clone)]
pub enum Expression {
    Value(Box<Value>),
    Builtin(BuiltinFunction, Box<[Expression]>),
    ExpressionList(Box<[Expression]>),
}

pub struct EvaluationError;

impl Expression {
    pub fn recursive_eval(&mut self, context: &mut EvalExecScope) -> Result<(), EvaluationError> {
        match self {
            Expression::Value(_) => Ok(()),
            Expression::Builtin(builtin, args) => {
                let mut errored = false;
                args.iter_mut().for_each(|arg| {
                    if arg.recursive_eval(context).is_err() {
                        errored = true;
                    }
                });
                if errored {
                    return Err(EvaluationError);
                };

                let args = args.iter().map(|elem| elem.get_val()).try_fold(
                    Vec::with_capacity(args.len()),
                    |mut prev, arg| {
                        if let Some(val) = arg {
                            prev.push(val);
                            Ok(prev)
                        } else {
                            Err(EvaluationError)
                        }
                    },
                )?;

                if let Some(val) = builtin.try_exec(args.as_slice(), context) {
                    *self = Self::Value(Box::new(val));
                    Ok(())
                } else {
                    Err(EvaluationError)
                }
            }
            Expression::ExpressionList(expressions) => {
                let mut context = context.new_scope();
                let mut errored = false;
                expressions.iter_mut().for_each(|elem| {
                    if elem.recursive_eval(&mut context).is_err() {
                        errored = true;
                    }
                });
                if errored {
                    return Err(EvaluationError);
                }

                let expressions_old = expressions;
                let mut expressions: Box<[Expression]> = Box::new([]);
                std::mem::swap(expressions_old, &mut expressions);
                let mut expressions = expressions.into_vec();

                match expressions.pop() {
                    Some(elem) => match elem.get_val_move() {
                        Ok(val) => {
                            *self = Expression::Value(val);
                            Ok(())
                        }
                        Err(val) => {
                            expressions.push(val);
                            *self = Expression::ExpressionList(expressions.into_boxed_slice());
                            Err(EvaluationError)
                        }
                    },
                    None => {
                        *self = Expression::Value(Box::new(Value::Void));
                        Ok(())
                    }
                }
            }
        }
    }
    pub fn get_val(&self) -> Option<&Value> {
        if let Self::Value(val) = self {
            Some(val.as_ref())
        } else {
            None
        }
    }
    pub fn get_val_move(self) -> Result<Box<Value>, Self> {
        if let Self::Value(val) = self {
            Ok(val)
        } else {
            Err(self)
        }
    }
}
