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

    fn find_in_global_environment(&self, name: &str) -> Option<Value> {
        let mut iter = ValueListIter::from(self.global_environment.clone());

        let from_env = iter.find(|v| {
            let car = v.borrow_car().unwrap();
            let caar = car.borrow_car().unwrap();
            if let Value::Symbol(s) = &*caar {
                s.as_str() == name
            } else {
                false
            }
        });

        from_env.and_then(|v| v.car().and_then(|v| v.cdr()))
    }

    pub fn process_special_forms(&mut self) {
        let iter = ValueTreeIter::from(self.ast.clone());
        for value in iter {
            let new_car = {
                let expr = value.borrow_car().unwrap();

                let builtin = expr.borrow_car().and_then(|v| {
                    if let Value::Symbol(s) = &*v {
                        self.find_in_global_environment(s.as_str()).and_then(|v| {
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
                    } else {
                        None
                    }
                });

                builtin.map(|b| (b.func)(&expr, self, &Value::Nil))
            };

            if let Some(new_car) = new_car {
                *value.borrow_mut_car().unwrap() = new_car;
            }
        }
    }

    pub fn eval(&mut self, value: &Value, environment: &Value) -> Value {
        // TODO
        match &value {
            v @ Value::Pair(_) => {
                let func = v.car().unwrap();
                let args = v.cdr().unwrap();

                let from_env = self.eval(&func, environment);

                match from_env {
                    Value::Builtin(b) => (b.func)(&args, self, environment),
                    // TODO
                    Value::Lambda(l) => {
                        // TODO
                        // bind bindings to env
                        // evaluate body
                        let v = self.eval(&l.borrow().body.car().unwrap(), environment);
                        v
                    }
                    _ => {
                        // Error
                        Value::Nil
                    }
                }
            }
            Value::Symbol(s) => {
                let mut iter = ValueListIter::from(environment.clone());
                let pair = iter.find(|v| {
                    if let Some(Value::Symbol(sym)) = v.car().and_then(|v| v.car()) {
                        *s == sym
                    } else {
                        // TODO
                        // panic!()
                        false
                    }
                });
                // TODO check exists in env
                let value = pair.and_then(|v| v.car()).and_then(|v| v.cdr()).unwrap();
                value
                // Value::Builtin(self.builtins.last().unwrap().clone())
            }
            _ => value.clone(),
        }
    }
}
