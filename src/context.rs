use std::cell::Ref;
use std::rc::Rc;

use crate::tokenizer::Tokenizer;
use crate::value::{Builtin, BuiltinType, Pair, Value, ValueListIter, ValueTreeIter};

#[derive(Debug)]
pub struct Context {
    pub global_environment: Value,
    pub symbol_table: Vec<Rc<String>>,
    pub builtins: Vec<Builtin>,
    pub ast: Value,
}

impl Context {
    pub fn new() -> Self {
        Self {
            global_environment: Value::new_pair(),
            symbol_table: Vec::new(),
            builtins: Vec::new(),
            ast: Value::Nil,
        }
    }

    pub fn add_builtin(
        &mut self,
        name: &str,
        ty: BuiltinType,
        func: fn(&Value, &mut Self, &Value) -> Value,
    ) {
        let name_rc = self.get_symbol(name);
        let new_env_value = Value::new_pair_car_cdr(
            Value::new_symbol(Rc::clone(&name_rc)),
            Value::new_builtin(Builtin {
                name: name_rc,
                ty,
                func,
            }),
        );

        let new_link = Value::new_pair_car_cdr(new_env_value, self.global_environment.clone());

        self.global_environment = new_link;
    }

    fn get_builtin(&self, name: &str) -> Option<Builtin> {
        self.builtins
            .iter()
            .find(|b| b.name.as_str() == name)
            .cloned()
    }

    fn get_symbol(&mut self, name: &str) -> Rc<String> {
        let maybe_rc = self.symbol_table.iter().find(|s| s.as_str() == name);

        match maybe_rc {
            Some(rc) => Rc::clone(rc),
            None => {
                let rc = Rc::new(String::from(name));
                self.symbol_table.push(Rc::clone(&rc));
                Rc::clone(&rc)
            }
        }
    }

    pub fn parse(self: &mut Context, t: &mut Tokenizer<'_>) {
        let append_value = |stk: &mut Vec<Value>, v: Value| {
            let Some(Value::Pair(pair)) = stk.last() else {
                panic!();
            };

            let new_pair = Value::new_pair();

            _ = pair.replace(Pair {
                car: v,
                cdr: new_pair.clone(),
            });

            *stk.last_mut().unwrap() = new_pair;
        };

        let root = Value::new_pair();

        let mut ast_stack: Vec<Value> = Vec::new();
        ast_stack.push(root.clone());

        for token in t {
            match token {
                "(" => {
                    let new_list = Value::new_pair();
                    append_value(&mut ast_stack, new_list.clone());
                    ast_stack.push(new_list);
                }
                // TODO
                //  check ast_stack len, shouldnt be < 1
                ")" => _ = ast_stack.pop(),
                s => {
                    if let Ok(i) = s.parse::<isize>() {
                        append_value(&mut ast_stack, Value::new_integer(i));
                    } else {
                        let symbol = self.get_symbol(s);
                        append_value(&mut ast_stack, Value::new_symbol(symbol.clone()));
                    }
                }
            }
        }

        // TODO
        //  check ast_stack len, should be 1
        self.ast = root;
    }

    pub fn process_special_forms(&mut self) {
        let iter = ValueTreeIter::from(self.ast.clone());
        for value in iter {
            let Value::Pair(pair) = value else {
                panic!();
            };

            let new_value = {
                let expr = &pair.borrow().car;

                expr.try_borrow_car()
                    .filter(|v| Value::is_symbol(v))
                    .map(|v| Ref::map(v, Value::as_symbol))
                    .and_then(|s| self.global_environment.assoc(s.as_str()))
                    .and_then(|v| {
                        if let Value::Builtin(
                            b @ Builtin {
                                ty: BuiltinType::SpecialForm,
                                ..
                            },
                        ) = v
                        {
                            Some(b)
                        } else {
                            None
                        }
                    })
                    .map(|b| (b.func)(expr, self, &Value::Nil))
            };

            if let Some(new_value) = new_value {
                pair.borrow_mut().car = new_value;
            }
        }
    }

    pub fn eval(&mut self, value: &Value, environment: &Value) -> Value {
        match value {
            Value::Pair(p) => {
                let func = &p.borrow().car;
                let args = &p.borrow().cdr;

                let from_env = self.eval(func, environment);

                match from_env {
                    Value::Builtin(b) => (b.func)(args, self, environment),
                    Value::Lambda(l) => {
                        // TODO
                        // bind bindings to env
                        // TODO
                        // body should eval all values in order
                        self.eval(&l.borrow().body.car(), environment)
                    }
                    _ => {
                        // Error
                        Value::Nil
                    }
                }
            }
            Value::Symbol(s) => {
                // TODO error if not found
                environment.assoc(s).unwrap()
            }
            _ => value.clone(),
        }
    }
}
