use std::{process, rc::Rc};

use crate::{
    ast::{
        expr::{
            bool_expr::BoolExpr, function_expr::FunctionExpr, if_expr::IfExpr,
            infix_expr::InfixExpr, int_expr::IntExpr, prefix_expr::PrefixExpr, str_expr::StrExpr,
        },
        expression::Node,
        identifier::Identifier,
        program::Program,
        statement::Statement,
        stmt::{
            block_stmt::BlockStatement, expression_stmt::ExpressionStatement, let_stmt::Let,
            return_stmt::Return, var_stmt::Var,
        },
        types::Type,
    },
    object::{
        boolean::Boolean, environment::Environment, error::Error, integer::Integer, null::Null,
        ret::Ret, string::Str, Object,
    },
};

pub struct Evaluator;

impl Evaluator {
    pub fn new() -> Self {
        Self
    }

    pub fn set(&self, name: String, obj: Rc<dyn Object>, env: &mut Environment) {
        env.set(name, obj)
    }

    pub fn get(&self, name: String, env: &mut Environment) -> Option<Rc<dyn Object>> {
        env.get(name)
    }

    pub fn eval(&self, node: Option<&dyn Node>, env: &mut Environment) -> Option<Rc<dyn Object>> {
        if let Some(node) = node {
            if let Some(program) = node.as_any().downcast_ref::<Program>() {
                Some(self.eval_statements(&program.statements, env))
            } else if let Some(stmt) = node.as_any().downcast_ref::<ExpressionStatement>() {
                match &stmt.expression {
                    Some(expr) => self.eval(Some(expr.as_ref()), env),
                    None => None,
                }
            } else if let Some(integer) = node.as_any().downcast_ref::<IntExpr>() {
                Some(Rc::new(Integer::new(integer.value)))
            } else if let Some(boolean) = node.as_any().downcast_ref::<BoolExpr>() {
                Some(Rc::new(Boolean::new(boolean.value)))
            } else if let Some(string) = node.as_any().downcast_ref::<StrExpr>() {
                Some(Rc::new(Str::new(string.value.clone())))
            } else if let Some(prefix) = node.as_any().downcast_ref::<PrefixExpr>() {
                let right = self.eval(Some(prefix.right.as_ref().unwrap().as_ref()), env);
                if self.is_error(&right) {
                    return right;
                }
                self.eval_prefix_expr(right.unwrap(), &prefix.operator)
            } else if let Some(infix) = node.as_any().downcast_ref::<InfixExpr>() {
                let right = self.eval(Some(infix.right.as_ref().unwrap().as_ref()), env);
                if self.is_error(&right) {
                    return right;
                }
                let left = self.eval(Some(infix.left.as_ref().unwrap().as_ref()), env);
                if self.is_error(&left) {
                    return left;
                }
                self.eval_infix_expr(left.unwrap(), right.unwrap(), &infix.operator)
            } else if let Some(if_expr) = node.as_any().downcast_ref::<IfExpr>() {
                self.eval_if_expression(if_expr, env)
            } else if let Some(block_stmt) = node.as_any().downcast_ref::<BlockStatement>() {
                Some(self.eval_statements(&block_stmt.statements, env))
            } else if let Some(ret) = node.as_any().downcast_ref::<Return>() {
                self.eval_ret_stmt(ret, env)
            } else if let Some(let_stmt) = node.as_any().downcast_ref::<Let>() {
                //TODO: make let immutable
                let val = self.eval(Some(let_stmt.value.as_ref()), env);
                if self.is_error(&val) {
                    return val;
                }
                self.set(
                    let_stmt.name.to_string(),
                    Rc::clone(val.as_ref().unwrap()),
                    env,
                );
                val
            } else if let Some(var) = node.as_any().downcast_ref::<Var>() {
                let val = self.eval(Some(var.value.as_ref()), env);
                if self.is_error(&val) {
                    return val;
                }
                self.set(var.name.to_string(), Rc::clone(val.as_ref().unwrap()), env);
                val
            } else if let Some(identifier) = node.as_any().downcast_ref::<Identifier>() {
                self.eval_identifier(identifier, env)
            } else if let Some(_function) = node.as_any().downcast_ref::<FunctionExpr>() {
                todo!()
            } else {
                Some(Rc::new(Null))
            }
        } else {
            None
        }
    }

    fn eval_prefix_expr(&self, right: Rc<dyn Object>, operator: &str) -> Option<Rc<dyn Object>> {
        match operator {
            "!" => self.eval_bang_operator(right),
            "-" => self.eval_minus_prefix_operator(right),
            _ => Some(Rc::new(Error::new(format!("invalid expression {}", right)))),
        }
    }

    fn eval_infix_expr(
        &self,
        left: Rc<dyn Object>,
        right: Rc<dyn Object>,
        operator: &str,
    ) -> Option<Rc<dyn Object>> {
        if left.get_type() != right.get_type() {
            return Some(Rc::new(Error::new(format!(
                "incompatible types {} and {}",
                right.get_type(),
                left.get_type()
            ))));
        }
        if let Some(left) = left.as_any().downcast_ref::<Integer>() {
            if let Some(right) = right.as_any().downcast_ref::<Integer>() {
                let left = *left.val.borrow();
                let right = *right.val.borrow();

                match operator {
                    "+" => Some(Rc::new(Integer::new(left + right))),
                    "-" => Some(Rc::new(Integer::new(left - right))),
                    "*" => Some(Rc::new(Integer::new(left * right))),
                    "/" => Some(Rc::new(Integer::new(left / right))),
                    "!=" => Some(Rc::new(Boolean::new(left != right))),
                    "==" => Some(Rc::new(Boolean::new(left == right))),
                    ">=" => Some(Rc::new(Boolean::new(left >= right))),
                    "<=" => Some(Rc::new(Boolean::new(left <= right))),
                    ">" => Some(Rc::new(Boolean::new(left > right))),
                    "<" => Some(Rc::new(Boolean::new(left < right))),
                    _ => Some(Rc::new(Error::new(format!(
                        "unknown operator {}",
                        operator
                    )))),
                }
            } else {
                Some(Rc::new(Error::new(format!("miss right operator {}", left))))
            }
        } else if let Some(left) = left.as_any().downcast_ref::<Boolean>() {
            if let Some(right) = right.as_any().downcast_ref::<Boolean>() {
                let left = *left.val.borrow();
                let right = *right.val.borrow();
                match operator {
                    "!=" => Some(Rc::new(Boolean::new(left != right))),
                    "==" => Some(Rc::new(Boolean::new(left == right))),
                    ">=" => Some(Rc::new(Boolean::new(left >= right))),
                    "<=" => Some(Rc::new(Boolean::new(left <= right))),
                    ">" => Some(Rc::new(Boolean::new(left & !right))),
                    "<" => Some(Rc::new(Boolean::new(!left & right))),
                    "||" => Some(Rc::new(Boolean::new(left || right))),
                    "&&" => Some(Rc::new(Boolean::new(left && right))),
                    _ => Some(Rc::new(Error::new(format!(
                        "unsupported operation {} {} {}",
                        left, operator, right
                    )))),
                }
            } else {
                Some(Rc::new(Error::new(format!("miss right operator {}", left))))
            }
        } else {
            Some(Rc::new(Error::new(format!(
                "unsupported operation {} {} {}",
                left, operator, right
            ))))
        }
    }

    fn eval_bang_operator(&self, right: Rc<dyn Object>) -> Option<Rc<dyn Object>> {
        if let Some(boolean) = right.as_any().downcast_ref::<Boolean>() {
            let mut val = boolean.val.borrow_mut();
            *val = !*val;
        } else {
            return Some(Rc::new(Error::new(format!(
                "invalid expression '!{}'",
                right
            ))));
        }
        Some(right)
    }

    fn eval_minus_prefix_operator(&self, right: Rc<dyn Object>) -> Option<Rc<dyn Object>> {
        if let Some(integer) = right.as_any().downcast_ref::<Integer>() {
            let mut val = integer.val.borrow_mut();
            *val = !*val;
        } else {
            return Some(Rc::new(Error::new(format!(
                "invalid expression '-{}'",
                right
            ))));
        }
        Some(right)
    }

    fn eval_statements(
        &self,
        stmts: &[Box<dyn Statement>],
        env: &mut Environment,
    ) -> Rc<dyn Object> {
        let mut result = None;
        for stmt in stmts.iter() {
            result = self.eval(Some(stmt.as_ref()), env);
            if let Some(ref res) = result {
                if let Some(ret) = res.as_any().downcast_ref::<Ret>() {
                    let return_value = Rc::clone(&ret.val);
                    return return_value;
                }
            }
        }
        if let Some(result) = result {
            result
        } else {
            process::exit(1)
        }
    }

    fn eval_if_expression(
        &self,
        if_expr: &IfExpr,
        env: &mut Environment,
    ) -> Option<Rc<dyn Object>> {
        let condition = self.eval(Some(if_expr.condition.as_ref()), env);
        if self.is_error(&condition) {
            return condition;
        }
        match condition {
            Some(condition) => match condition.as_any().downcast_ref::<Boolean>() {
                Some(condition) => {
                    if *condition.val.borrow_mut() {
                        if let Some(ref consequence) = if_expr.consequence {
                            self.eval(Some(consequence.as_ref()), env)
                        } else {
                            None
                        }
                    } else if let Some(ref el_if) = if_expr.el_if {
                        self.eval(Some(el_if.as_ref()), env)
                    } else if let Some(ref alternative) = if_expr.alternative {
                        self.eval(Some(alternative.as_ref()), env)
                    } else {
                        None
                    }
                }
                None => None,
            },
            None => None,
        }
    }

    fn eval_ret_stmt(&self, ret: &Return, env: &mut Environment) -> Option<Rc<dyn Object>> {
        match &ret.return_value {
            Some(expr) => {
                let val = self.eval(Some(expr.as_ref()), env);
                match val {
                    _ if self.is_error(&val) => val,
                    Some(val) => Some(Rc::new(Ret::new(val))),
                    None => None,
                }
            }
            None => None,
        }
    }

    fn is_error(&self, to_check: &Option<Rc<dyn Object>>) -> bool {
        matches!(to_check, Some(check) if check.get_type() == Type::Error)
    }

    fn eval_identifier(
        &self,
        identifier: &Identifier,
        env: &mut Environment,
    ) -> Option<Rc<dyn Object>> {
        let val = self.get(identifier.to_string(), env);
        if val.is_some() {
            val
        } else {
            Some(Rc::new(Error::new(format!(
                "unknown word '{}'",
                identifier
            ))))
        }
    }
}

impl Default for Evaluator {
    fn default() -> Self {
        Self::new()
    }
}
