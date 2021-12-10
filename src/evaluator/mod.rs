use std::{cell::RefCell, collections::HashMap, process, rc::Rc};

mod built_in_fns;

use crate::{
    ast::{
        expr::{
            arr_expr::ArrayExpr, bool_expr::BoolExpr, call_expr::CallExpr,
            function_expr::FunctionExpr, if_expr::IfExpr, index_expr::IndexExpr,
            infix_expr::InfixExpr, int_expr::IntExpr, prefix_expr::PrefixExpr, str_expr::StrExpr,
            while_expr::WhileExpr,
        },
        expression::{Expression, Node},
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
        array::Array, boolean::Boolean, built_in::BuiltIn, environment::Environment, error::Error,
        function::Function, integer::Integer, null::Null, ret::Ret, string::Str, Object,
    },
};

pub struct Evaluator {
    pub build_in_fns: HashMap<String, Rc<dyn Object>>,
}

impl Evaluator {
    pub fn new() -> Self {
        let mut build_in_fns: HashMap<String, Rc<dyn Object>> = HashMap::new();
        build_in_fns.insert("len".to_string(), Rc::new(BuiltIn::new(built_in_fns::len)));
        build_in_fns.insert(
            "to_str".to_string(),
            Rc::new(BuiltIn::new(built_in_fns::to_str)),
        );
        build_in_fns.insert(
            "to_int".to_string(),
            Rc::new(BuiltIn::new(built_in_fns::to_int)),
        );
        build_in_fns.insert(
            "puts".to_string(),
            Rc::new(BuiltIn::new(built_in_fns::puts)),
        );
        build_in_fns.insert(
            "putsln".to_string(),
            Rc::new(BuiltIn::new(built_in_fns::putsln)),
        );
        build_in_fns.insert(
            "eputs".to_string(),
            Rc::new(BuiltIn::new(built_in_fns::eputs)),
        );
        build_in_fns.insert(
            "eputsln".to_string(),
            Rc::new(BuiltIn::new(built_in_fns::eputsln)),
        );
        build_in_fns.insert(
            "append".to_string(),
            Rc::new(BuiltIn::new(built_in_fns::append)),
        );
        build_in_fns.insert(
            "slice".to_string(),
            Rc::new(BuiltIn::new(built_in_fns::slice)),
        );
        build_in_fns.insert(
            "input".to_string(),
            Rc::new(BuiltIn::new(built_in_fns::input)),
        );
        build_in_fns.insert(
            "is_error".to_string(),
            Rc::new(BuiltIn::new(built_in_fns::is_error)),
        );
        Self { build_in_fns }
    }

    pub fn set(&self, name: String, obj: Rc<dyn Object>, env: Rc<RefCell<Environment>>) {
        env.borrow_mut().set(name, obj)
    }

    pub fn get(&self, name: String, env: Rc<RefCell<Environment>>) -> Option<Rc<dyn Object>> {
        env.borrow_mut().get(name)
    }

    pub fn eval(
        &self,
        node: Option<&dyn Node>,
        env: Rc<RefCell<Environment>>,
    ) -> Option<Rc<dyn Object>> {
        if let Some(node) = node {
            if let Some(program) = node.as_any().downcast_ref::<Program>() {
                Some(self.eval_statements(&program.statements, Rc::clone(&env)))
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
                let right = self.eval(
                    Some(prefix.right.as_ref().unwrap().as_ref()),
                    Rc::clone(&env),
                );
                if self.is_error(&right) {
                    return right;
                }
                self.eval_prefix_expr(right.unwrap(), &prefix.operator)
            } else if let Some(infix) = node.as_any().downcast_ref::<InfixExpr>() {
                let right = self.eval(
                    Some(infix.right.as_ref().unwrap().as_ref()),
                    Rc::clone(&env),
                );
                if self.is_error(&right) {
                    return right;
                }
                if infix.operator == "=" {
                    let right = self.eval(
                        Some(infix.right.as_ref().unwrap().as_ref()),
                        Rc::clone(&env),
                    );
                    self.set(
                        infix.left.as_ref().unwrap().to_string(),
                        Rc::clone(right.as_ref().unwrap()),
                        Rc::clone(&env),
                    );
                    right
                } else {
                    let left =
                        self.eval(Some(infix.left.as_ref().unwrap().as_ref()), Rc::clone(&env));
                    if self.is_error(&left) {
                        return left;
                    }
                    self.eval_infix_expr(left.unwrap(), right.unwrap(), &infix.operator)
                }
            } else if let Some(if_expr) = node.as_any().downcast_ref::<IfExpr>() {
                self.eval_if_expression(if_expr, Rc::clone(&env))
            } else if let Some(while_expr) = node.as_any().downcast_ref::<WhileExpr>() {
                self.eval_while_expression(while_expr, Rc::clone(&env))
            } else if let Some(block_stmt) = node.as_any().downcast_ref::<BlockStatement>() {
                Some(self.eval_statements(&block_stmt.statements, env))
            } else if let Some(ret) = node.as_any().downcast_ref::<Return>() {
                self.eval_ret_stmt(ret, Rc::clone(&env))
            } else if let Some(let_stmt) = node.as_any().downcast_ref::<Let>() {
                //TODO: make let immutable
                let val = self.eval(Some(let_stmt.value.as_ref()), Rc::clone(&env));
                if self.is_error(&val) {
                    return val;
                }
                self.set(
                    let_stmt.name.to_string(),
                    Rc::clone(val.as_ref().unwrap()),
                    Rc::clone(&env),
                );
                val
            } else if let Some(var) = node.as_any().downcast_ref::<Var>() {
                let val = self.eval(Some(var.value.as_ref()), Rc::clone(&env));
                if self.is_error(&val) {
                    return val;
                }
                self.set(
                    var.name.to_string(),
                    Rc::clone(val.as_ref().unwrap()),
                    Rc::clone(&env),
                );
                val
            } else if let Some(identifier) = node.as_any().downcast_ref::<Identifier>() {
                self.eval_identifier(identifier, env)
            } else if let Some(array) = node.as_any().downcast_ref::<ArrayExpr>() {
                let elements = self.eval_expressions(&array.value, Rc::clone(&env));
                let mut elems = Vec::with_capacity(10);
                for elem in elements {
                    elems.push(elem.unwrap());
                }
                Some(Rc::new(Array::new(elems)))
            } else if let Some(idx_expr) = node.as_any().downcast_ref::<IndexExpr>() {
                let left = self.eval(Some(idx_expr.left.as_ref()), Rc::clone(&env));
                if self.is_error(&left) {
                    return left;
                }
                let idx = self.eval(Some(idx_expr.index.as_ref()), Rc::clone(&env));
                if self.is_error(&idx) {
                    return idx;
                }
                self.eval_index_expr(left.unwrap(), idx.unwrap())
            } else if let Some(function) = node.as_any().downcast_ref::<FunctionExpr>() {
                let env = Rc::clone(&env);
                let body = function.body.as_ref().map(Rc::clone);
                let parameters: Vec<Rc<dyn Expression>> =
                    function.parameters.iter().map(Rc::clone).collect();
                let function =
                    Function::new(parameters, Rc::clone(&function.name), body, Rc::clone(&env));
                let function_name = function.name.to_string();
                let function_ref: Rc<dyn Object> = Rc::new(function);
                self.set(function_name, Rc::clone(&function_ref), Rc::clone(&env));
                Some(function_ref)
            } else if let Some(call) = node.as_any().downcast_ref::<CallExpr>() {
                let function = self.eval(Some(call.function.as_ref()), Rc::clone(&env));
                if self.is_error(&function) {
                    return function;
                }
                let arguments = self.eval_expressions(&call.arguments, Rc::clone(&env));
                if arguments.len() == 1 && self.is_error(arguments.first().unwrap()) {
                    return function;
                }
                self.apply_function(function.unwrap(), arguments)
            } else {
                Some(Rc::new(Null))
            }
        } else {
            None
        }
    }

    fn eval_index_expr(
        &self,
        left: Rc<dyn Object>,
        index: Rc<dyn Object>,
    ) -> Option<Rc<dyn Object>> {
        if (left.get_type() == Type::Array || left.get_type() == Type::String)
            && index.get_type() == Type::Int
        {
            self.eval_array_index_expr(left, index)
        } else {
            Some(Rc::new(Error::new(format!(
                "index operation not suported: {}",
                left.get_type()
            ))))
        }
    }

    fn eval_array_index_expr(
        &self,
        left: Rc<dyn Object>,
        index: Rc<dyn Object>,
    ) -> Option<Rc<dyn Object>> {
        let index = *index
            .as_any()
            .downcast_ref::<Integer>()
            .unwrap()
            .val
            .borrow_mut();
        if let Some(array) = left.as_any().downcast_ref::<Array>() {
            let arr = array.elements.borrow_mut();
            let max = arr.len() as i64 - 1;
            if index < 0 || index > max || max < 0 {
                return Some(Rc::new(Null));
            }
            let element = arr.get(index as usize).unwrap();
            Some(Rc::clone(element))
        } else if let Some(string) = left.as_any().downcast_ref::<Str>() {
            let max = string.val.len() as i64 - 1;
            if index < 0 || index > max || max < 0 {
                return Some(Rc::new(Null));
            }
            let ch = string.val.chars().nth(index as usize).unwrap();
            Some(Rc::new(Str::new(ch.to_string())))
        } else {
            Some(Rc::new(Error::new(format!(
                "index operation not suported: {}[{}]",
                left.get_type(),
                index
            ))))
        }
    }

    fn apply_function(
        &self,
        function: Rc<dyn Object>,
        args: Vec<Option<Rc<dyn Object>>>,
    ) -> Option<Rc<dyn Object>> {
        if let Some(function) = function.as_any().downcast_ref::<Function>() {
            let new_env = self.create_function_environment(function, &args);
            let evaluated = self.eval(
                Some(function.body.as_ref().unwrap().as_ref()),
                Rc::clone(&new_env),
            );
            if self.is_error(&evaluated) {
                return evaluated;
            }
            self.extract_ret_val(evaluated)
        } else if let Some(built_in) = function.as_any().downcast_ref::<BuiltIn>() {
            let mut arguments = Vec::with_capacity(5);
            for arg in args {
                arguments.push(arg.unwrap());
            }
            let func = built_in.function;
            Some(func(&arguments))
        } else {
            Some(Rc::new(Error::new(format!(
                "not a function {}",
                function.get_type()
            ))))
        }
    }

    fn extract_ret_val(&self, evaluated: Option<Rc<dyn Object>>) -> Option<Rc<dyn Object>> {
        if let Some(ret) = evaluated.as_ref().unwrap().as_any().downcast_ref::<Ret>() {
            Some(Rc::clone(&ret.val))
        } else {
            evaluated
        }
    }

    fn create_function_environment(
        &self,
        function: &Function,
        args: &[Option<Rc<dyn Object>>],
    ) -> Rc<RefCell<Environment>> {
        let env = Rc::new(RefCell::new(Environment::new(Some(Rc::clone(
            &function.env,
        )))));
        for (idx, arg) in args.iter().enumerate() {
            env.borrow_mut().set(
                function.parameters.get(idx).unwrap().to_string(),
                Rc::clone(arg.as_ref().unwrap()),
            )
        }
        env
    }

    fn eval_expressions(
        &self,
        args: &[Rc<dyn Expression>],
        env: Rc<RefCell<Environment>>,
    ) -> Vec<Option<Rc<dyn Object>>> {
        let mut evaluated_args = Vec::new();
        for arg in args {
            let evaluated = self.eval(Some(arg.as_ref()), Rc::clone(&env));
            if self.is_error(&evaluated) {
                return vec![evaluated];
            }
            evaluated_args.push(evaluated)
        }
        evaluated_args
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
        } else if let Some(left) = left.as_any().downcast_ref::<Str>() {
            if let Some(right) = right.as_any().downcast_ref::<Str>() {
                let left = &left.val;
                let right = &right.val;
                match operator {
                    "+" => Some(Rc::new(Str::new(format!("{}{}", left, right)))),
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
        stmts: &[Rc<dyn Statement>],
        env: Rc<RefCell<Environment>>,
    ) -> Rc<dyn Object> {
        let mut result = None;
        for stmt in stmts.iter() {
            result = self.eval(Some(stmt.as_ref()), Rc::clone(&env));
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
        env: Rc<RefCell<Environment>>,
    ) -> Option<Rc<dyn Object>> {
        let condition = self.eval(Some(if_expr.condition.as_ref()), Rc::clone(&env));
        if self.is_error(&condition) {
            return condition;
        }
        match condition {
            Some(condition) => match condition.as_any().downcast_ref::<Boolean>() {
                Some(condition) => {
                    let new_env = Rc::new(RefCell::new(Environment::new(Some(Rc::clone(&env)))));
                    if *condition.val.borrow_mut() {
                        if let Some(ref consequence) = if_expr.consequence {
                            self.eval(Some(consequence.as_ref()), Rc::clone(&new_env))
                        } else {
                            None
                        }
                    } else if let Some(ref el_if) = if_expr.el_if {
                        self.eval(Some(el_if.as_ref()), Rc::clone(&new_env))
                    } else if let Some(ref alternative) = if_expr.alternative {
                        self.eval(Some(alternative.as_ref()), Rc::clone(&new_env))
                    } else {
                        None
                    }
                }
                None => None,
            },
            None => None,
        }
    }

    fn eval_while_expression(
        &self,
        while_expr: &WhileExpr,
        env: Rc<RefCell<Environment>>,
    ) -> Option<Rc<dyn Object>> {
        let mut original_condition =
            self.eval(Some(while_expr.condition.as_ref()), Rc::clone(&env));
        if self.is_error(&original_condition) {
            return original_condition;
        }
        let mut obj: Option<Rc<dyn Object>> = None;
        loop {
            obj = match original_condition {
                Some(ref condition) => match condition.as_any().downcast_ref::<Boolean>() {
                    Some(condition) => {
                        let new_env =
                            Rc::new(RefCell::new(Environment::new(Some(Rc::clone(&env)))));
                        if *condition.val.borrow_mut() {
                            let ret = self.eval(
                                Some(while_expr.consequence.as_ref().unwrap().as_ref()),
                                Rc::clone(&env),
                            );
                            original_condition =
                                self.eval(Some(while_expr.condition.as_ref()), Rc::clone(&new_env));
                            ret
                        } else {
                            break;
                        }
                    }
                    None => None,
                },
                None => None,
            }
        }
        obj
    }

    fn eval_ret_stmt(&self, ret: &Return, env: Rc<RefCell<Environment>>) -> Option<Rc<dyn Object>> {
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
        env: Rc<RefCell<Environment>>,
    ) -> Option<Rc<dyn Object>> {
        let val = self.get(identifier.to_string(), env);
        if val.is_some() {
            val
        } else {
            let built_in = self.build_in_fns.get(&identifier.to_string());
            if let Some(built_in) = built_in {
                Some(Rc::clone(built_in))
            } else {
                Some(Rc::new(Error::new(format!(
                    "unknown word '{}'",
                    identifier
                ))))
            }
        }
    }
}

impl Default for Evaluator {
    fn default() -> Self {
        Self::new()
    }
}
