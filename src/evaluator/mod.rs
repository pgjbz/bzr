use std::{process, rc::Rc};

use crate::{
    ast::{
        expr::{
            bool_expr::BoolExpr, if_expr::IfExpr, infix_expr::InfixExpr, int_expr::IntExpr,
            prefix_expr::PrefixExpr, str_expr::StrExpr,
        },
        expression::Node,
        program::Program,
        statement::Statement,
        stmt::{
            block_stmt::BlockStatement, expression_stmt::ExpressionStatement, return_stmt::Return,
        },
    },
    object::{
        boolean::Boolean, error::Error, integer::Integer, null::Null, ret::Ret, string::Str, Object,
    },
};

pub fn eval(node: Option<&dyn Node>) -> Option<Rc<dyn Object>> {
    if let Some(node) = node {
        if let Some(program) = node.as_any().downcast_ref::<Program>() {
            Some(eval_statements(&program.statements))
        } else if let Some(stmt) = node.as_any().downcast_ref::<ExpressionStatement>() {
            match &stmt.expression {
                Some(expr) => eval(Some(expr.as_ref())),
                None => None,
            }
        } else if let Some(integer) = node.as_any().downcast_ref::<IntExpr>() {
            Some(Rc::new(Integer::new(integer.value)))
        } else if let Some(boolean) = node.as_any().downcast_ref::<BoolExpr>() {
            Some(Rc::new(Boolean::new(boolean.value)))
        } else if let Some(string) = node.as_any().downcast_ref::<StrExpr>() {
            Some(Rc::new(Str::new(string.value.clone())))
        } else if let Some(prefix) = node.as_any().downcast_ref::<PrefixExpr>() {
            let right = eval(Some(prefix.right.as_ref().unwrap().as_ref()));
            eval_prefix_expr(right.unwrap(), &prefix.operator)
        } else if let Some(infix) = node.as_any().downcast_ref::<InfixExpr>() {
            let right = eval(Some(infix.right.as_ref().unwrap().as_ref()));
            let left = eval(Some(infix.left.as_ref().unwrap().as_ref()));
            eval_infix_expr(left.unwrap(), right.unwrap(), &infix.operator)
        } else if let Some(if_expr) = node.as_any().downcast_ref::<IfExpr>() {
            eval_if_expression(if_expr)
        } else if let Some(block_stmt) = node.as_any().downcast_ref::<BlockStatement>() {
            Some(eval_statements(&block_stmt.statements))
        } else if let Some(ret) = node.as_any().downcast_ref::<Return>() {
            eval_ret_stmt(ret)
        } else {
            Some(Rc::new(Null))
        }
    } else {
        None
    }
}

fn eval_prefix_expr(right: Rc<dyn Object>, operator: &str) -> Option<Rc<dyn Object>> {
    match operator {
        "!" => eval_bang_operator(right),
        "-" => eval_minus_prefix_operator(right),
        _ => Some(Rc::new(Error::new(format!("invalid expression {}", right)))),
    }
}

fn eval_infix_expr(
    left: Rc<dyn Object>,
    right: Rc<dyn Object>,
    operator: &str,
) -> Option<Rc<dyn Object>> {
    if left.get_type() != right.get_type() {
        eprintln!(
            "incompatible types {} and {}",
            right.get_type(),
            left.get_type()
        );
        return None;
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
                _ => None,
            }
        } else {
            None
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
                _ => None,
            }
        } else {
            None
        }
    } else {
        None
    }
}

fn eval_bang_operator(right: Rc<dyn Object>) -> Option<Rc<dyn Object>> {
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

fn eval_minus_prefix_operator(right: Rc<dyn Object>) -> Option<Rc<dyn Object>> {
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

fn eval_statements(stmts: &[Box<dyn Statement>]) -> Rc<dyn Object> {
    let mut result = None;
    for stmt in stmts.iter() {
        result = eval(Some(stmt.as_ref()));
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

fn eval_if_expression(if_expr: &IfExpr) -> Option<Rc<dyn Object>> {
    let condition = eval(Some(if_expr.condition.as_ref()));
    match condition {
        Some(condition) => match condition.as_any().downcast_ref::<Boolean>() {
            Some(condition) => {
                if *condition.val.borrow_mut() {
                    if let Some(ref consequence) = if_expr.consequence {
                        eval(Some(consequence.as_ref()))
                    } else {
                        None
                    }
                } else if let Some(ref el_if) = if_expr.el_if {
                    eval(Some(el_if.as_ref()))
                } else if let Some(ref alternative) = if_expr.alternative {
                    eval(Some(alternative.as_ref()))
                } else {
                    None
                }
            }
            None => None,
        },
        None => None,
    }
}

fn eval_ret_stmt(ret: &Return) -> Option<Rc<dyn Object>> {
    match &ret.return_value {
        Some(expr) => {
            let val = eval(Some(expr.as_ref()));
            match val {
                Some(val) => Some(Rc::new(Ret::new(val))),
                None => None,
            }
        }
        None => None,
    }
}
