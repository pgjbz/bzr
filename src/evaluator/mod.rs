use crate::{
    ast::{
        expr::{int_expr::IntExpr, bool_expr::BoolExpr}, expression::Node, program::Program, statement::Statement,
        stmt::expression_stmt::ExpressionStatement,
    },
    object::{integer::Integer, null::Null, Object, boolean::Boolean},
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
    } else {
        Some(Box::new(Null))
    }
}

fn parse_statements(stmts: &[Box<dyn Statement>]) -> Box<dyn Object> {
    let mut result = None;
    for stmt in stmts.iter() {
        result = Some(eval(stmt.as_ref()))
    }
    result.unwrap().unwrap()
}
