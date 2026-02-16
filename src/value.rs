use std::cell::{Ref, RefCell, RefMut};
use std::fmt;
use std::rc::Rc;

use crate::Context;

#[derive(Debug, Clone)]
pub struct Pair {
    pub car: Value,
    pub cdr: Value,
}

#[derive(Debug, Clone)]
pub struct Lambda {
    pub bindings: Value,
    pub body: Value,
}

#[derive(Debug, Clone)]
pub enum BuiltinType {
    Normal,
    Macro,
    SpecialForm,
}

#[derive(Debug, Clone)]
pub struct Builtin {
    pub name: Rc<String>,
    pub ty: BuiltinType,
    pub func: fn(args: &Value, context: &mut Context, environment: &Value) -> Value,
}

#[derive(Debug, Clone)]
pub enum Value {
    Nil,
    Integer(isize),
    Symbol(Rc<String>),
    Builtin(Builtin),
    Lambda(Rc<RefCell<Lambda>>),
    Pair(Rc<RefCell<Pair>>),
}

impl Value {
    // pairs ===

    pub fn new_pair() -> Value {
        Self::new_pair_car_cdr(Value::Nil, Value::Nil)
    }

    pub fn new_pair_car_cdr(car: Value, cdr: Value) -> Value {
        let pair = Pair { car, cdr };
        let ptr = Rc::new(RefCell::new(pair));
        Value::Pair(ptr)
    }

    pub fn car(&self) -> Option<Value> {
        if let Value::Pair(p0) = self {
            Some(p0.borrow().car.clone())
        } else {
            None
        }
    }

    pub fn cdr(&self) -> Option<Value> {
        if let Value::Pair(p0) = self {
            Some(p0.borrow().cdr.clone())
        } else {
            None
        }
    }

    pub fn borrow_car(&self) -> Option<Ref<'_, Value>> {
        if let Value::Pair(p0) = self {
            Some(Ref::map(p0.borrow(), |p| &p.car))
        } else {
            None
        }
    }

    pub fn borrow_mut_car(&self) -> Option<RefMut<'_, Value>> {
        if let Value::Pair(p0) = self {
            Some(RefMut::map(p0.borrow_mut(), |p| &mut p.car))
        } else {
            None
        }
    }

    pub fn borrow_cdr(&self) -> Option<Ref<'_, Value>> {
        if let Value::Pair(p0) = self {
            Some(Ref::map(p0.borrow(), |p| &p.cdr))
        } else {
            None
        }
    }

    pub fn borrow_mut_cdr(&self) -> Option<RefMut<'_, Value>> {
        if let Value::Pair(p0) = self {
            Some(RefMut::map(p0.borrow_mut(), |p| &mut p.cdr))
        } else {
            None
        }
    }

    // other stuff ===

    pub fn new_symbol(s: Rc<String>) -> Value {
        Value::Symbol(s)
    }

    pub fn new_integer(i: isize) -> Value {
        Value::Integer(i)
    }

    pub fn new_builtin(b: Builtin) -> Value {
        Value::Builtin(b)
    }

    pub fn new_lambda(l: Lambda) -> Value {
        Value::Lambda(Rc::new(RefCell::new(l)))
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Value::Nil => write!(f, "nil"),
            Value::Integer(i) => write!(f, "i.{i}"),
            Value::Symbol(s) => write!(f, "{s}"),
            Value::Builtin(b) => write!(f, "[{}]", b.name),
            Value::Lambda(l) => {
                let l = l.borrow();
                write!(f, "\\{}:{}", l.bindings, l.body)
            }
            Value::Pair(p) => match &*p.borrow() {
                Pair {
                    car: Value::Nil,
                    cdr: Value::Nil,
                } => {
                    write!(f, "()")
                }
                p => {
                    if matches!(p.cdr, Value::Pair(_)) {
                        let iter = ValueListIter::from(self.clone());
                        _ = write!(f, "(");
                        for value in iter {
                            _ = write!(f, "{}", value.borrow_car().unwrap());

                            match &*value.borrow_cdr().unwrap() {
                                v @ Value::Pair(_) => {
                                    if matches!(&*v.borrow_cdr().unwrap(), Value::Nil) {
                                        Ok(())
                                    } else {
                                        write!(f, " ")
                                    };
                                }
                                _ => (),
                                // TODO handle dotted
                                // Value::Pair(_) => write!(f," "),
                            }
                        }
                        write!(f, ")")
                    } else {
                        write!(f, "({} . {})", p.car, p.cdr)
                    }
                }
            },
        }
    }
}

// ===

pub struct ValueListIter {
    current: Option<Value>,
}

impl ValueListIter {
    pub fn from(v: Value) -> Self {
        Self { current: Some(v) }
    }
}

impl Iterator for ValueListIter {
    type Item = Value;
    fn next(&mut self) -> Option<Self::Item> {
        // TODO
        // iterating on () should return none right off the bat
        let ret = self.current.clone();
        self.current = self
            .current
            .as_ref()
            .and_then(|v| v.borrow_cdr())
            .filter(|v| {
                v.borrow_car().is_some_and(|v| !matches!(*v, Value::Nil))
                    && v.borrow_cdr().is_some_and(|v| !matches!(*v, Value::Nil))
            })
            .map(|v| (*v).clone());
        ret
    }
}

// ===

pub struct ValueTreeIter {
    iterators: Vec<ValueListIter>,
}

impl ValueTreeIter {
    pub fn from(v: Value) -> Self {
        let iterators = vec![ValueListIter::from(v)];
        Self { iterators }
    }
}

impl Iterator for ValueTreeIter {
    type Item = Value;
    fn next(&mut self) -> Option<Self::Item> {
        let mut ret = self.iterators.last_mut().and_then(|i| i.next());

        while matches!(ret, None) {
            let i = self.iterators.pop();
            if matches!(i, None) {
                break;
            }

            ret = self.iterators.last_mut().and_then(|i| i.next());
        }

        if matches!(ret, None) {
            return None;
        }

        match ret.as_ref().unwrap().car() {
            Some(v @ Value::Pair(_)) => {
                // TODO
                // If something like this: (a b (c d)) is encountered
                // ( ( c d ) ) will be returned at some point
                // unsure if this value should be skipped
                self.iterators.push(ValueListIter::from(v.clone()));
            }
            _ => (),
        }

        ret
    }
}
