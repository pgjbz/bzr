use std::process;

use crate::{
    ast::{
        expr::{
            bool_expr::BoolExpr, infix_expr::InfixExpr, int_expr::IntExpr, prefix_expr::PrefixExpr,
            str_expr::StrExpr,
        },
        expression::Node,
        program::Program,
        statement::Statement,
        stmt::expression_stmt::ExpressionStatement,
    },
    object::{boolean::Boolean, integer::Integer, null::Null, string::Str, Object},
};

pub fn eval(node: Option<&dyn Node>) -> Option<Box<dyn Object>> {
    if let Some(node) = node {
        if let Some(program) = node.as_any().downcast_ref::<Program>() {
            Some(parse_statements(&program.statements))
        } else if let Some(stmt) = node.as_any().downcast_ref::<ExpressionStatement>() {
            match &stmt.expression {
                Some(expr) => eval(Some(expr.as_ref())),
                None => None,
            }
        } else if let Some(integer) = node.as_any().downcast_ref::<IntExpr>() {
            Some(Box::new(Integer::new(integer.value)))
        } else if let Some(boolean) = node.as_any().downcast_ref::<BoolExpr>() {
            Some(Box::new(Boolean::new(boolean.value)))
        } else if let Some(string) = node.as_any().downcast_ref::<StrExpr>() {
            Some(Box::new(Str::new(string.value.clone())))
        } else if let Some(prefix) = node.as_any().downcast_ref::<PrefixExpr>() {
            let right = eval(Some(prefix.right.as_ref().unwrap().as_ref()));
            eval_prefix_expr(right.unwrap(), &prefix.operator)
        } else if let Some(infix) = node.as_any().downcast_ref::<InfixExpr>() {
            let right = eval(Some(infix.right.as_ref().unwrap().as_ref()));
            let left = eval(Some(infix.left.as_ref().unwrap().as_ref()));
            eval_infix_expr(left.unwrap(), right.unwrap(), &infix.operator)
        } else {
            Some(Box::new(Null))
        }
    } else {
        None
    }
}

fn eval_prefix_expr(right: Box<dyn Object>, operator: &str) -> Option<Box<dyn Object>> {
    match operator {
        "!" => eval_bang_operator(right),
        "-" => eval_minus_prefix_operator(right),
        _ => {
            eprintln!("invalid expression {}", right);
            None
        }
    }
}

fn eval_infix_expr(
    left: Box<dyn Object>,
    right: Box<dyn Object>,
    operator: &str,
) -> Option<Box<dyn Object>> {
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
                "+" => Some(Box::new(Integer::new(left + right))),
                "-" => Some(Box::new(Integer::new(left - right))),
                "*" => Some(Box::new(Integer::new(left * right))),
                "/" => Some(Box::new(Integer::new(left / right))),
                "!=" => Some(Box::new(Boolean::new(left != right))),
                "==" => Some(Box::new(Boolean::new(left == right))),
                ">=" => Some(Box::new(Boolean::new(left >= right))),
                "<=" => Some(Box::new(Boolean::new(left <= right))),
                ">" => Some(Box::new(Boolean::new(left > right))),
                "<" => Some(Box::new(Boolean::new(left < right))),
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
                "!=" => Some(Box::new(Boolean::new(left != right))),
                "==" => Some(Box::new(Boolean::new(left == right))),
                ">=" => Some(Box::new(Boolean::new(left >= right))),
                "<=" => Some(Box::new(Boolean::new(left <= right))),
                ">" => Some(Box::new(Boolean::new(left & !right))),
                "<" => Some(Box::new(Boolean::new(!left & right))),
                _ => None,
            }
        } else {
            None
        }
    } else {
        None
    }
}

fn eval_bang_operator(right: Box<dyn Object>) -> Option<Box<dyn Object>> {
    if let Some(boolean) = right.as_any().downcast_ref::<Boolean>() {
        let mut val = boolean.val.borrow_mut();
        *val = !*val;
    } else {
        eprintln!("invalid expression '!{}'", right);
        return None;
    }
    Some(right)
}

fn eval_minus_prefix_operator(right: Box<dyn Object>) -> Option<Box<dyn Object>> {
    if let Some(boolean) = right.as_any().downcast_ref::<Integer>() {
        let mut val = boolean.val.borrow_mut();
        *val = !*val;
    } else {
        eprintln!("invalid expression '-{}'", right);
        return None;
    }
    Some(right)
}

fn parse_statements(stmts: &[Box<dyn Statement>]) -> Box<dyn Object> {
    let mut result = None;
    for stmt in stmts.iter() {
        result = eval(Some(stmt.as_ref()))
    }
    if let Some(result) = result {
        result
    } else {
        process::exit(1)
    }
}
