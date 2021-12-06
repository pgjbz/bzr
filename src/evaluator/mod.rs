use crate::{
    ast::{
        expr::{
            bool_expr::BoolExpr, int_expr::IntExpr, prefix_expr::PrefixExpr, str_expr::StrExpr,
        },
        expression::Node,
        program::Program,
        statement::Statement,
        stmt::expression_stmt::ExpressionStatement,
    },
    object::{boolean::Boolean, integer::Integer, null::Null, string::Str, Object},
};

pub fn eval(node: &dyn Node) -> Option<Box<dyn Object>> {
    if let Some(program) = node.as_any().downcast_ref::<Program>() {
        Some(parse_statements(&program.statements))
    } else if let Some(stmt) = node.as_any().downcast_ref::<ExpressionStatement>() {
        match &stmt.expression {
            Some(expr) => eval(expr.as_ref()),
            None => None,
        }
    } else if let Some(integer) = node.as_any().downcast_ref::<IntExpr>() {
        Some(Box::new(Integer::new(integer.value)))
    } else if let Some(boolean) = node.as_any().downcast_ref::<BoolExpr>() {
        Some(Box::new(Boolean::new(boolean.value)))
    } else if let Some(string) = node.as_any().downcast_ref::<StrExpr>() {
        Some(Box::new(Str::new(string.value.clone())))
    } else if let Some(prefix) = node.as_any().downcast_ref::<PrefixExpr>() {
        let right = eval(prefix.right.as_ref().unwrap().as_ref());
        eval_prefix_expr(right.unwrap(), &prefix.operator)
    } else {
        Some(Box::new(Null))
    }
}

fn eval_prefix_expr(right: Box<dyn Object>, operator: &str) -> Option<Box<dyn Object>> {
    match operator {
        "!" => eval_bang_operator(right),
        _ => None,
    }
}

fn eval_bang_operator(right: Box<dyn Object>) -> Option<Box<dyn Object>> {
    if let Some(boolean) = right.as_any().downcast_ref::<Boolean>() {
        let mut val = boolean.val.borrow_mut();
        *val = !*val;
    } else {
        return None;
    }
    Some(right)
}

fn parse_statements(stmts: &[Box<dyn Statement>]) -> Box<dyn Object> {
    let mut result = None;
    for stmt in stmts.iter() {
        result = Some(eval(stmt.as_ref()))
    }
    result.unwrap().unwrap()
}
