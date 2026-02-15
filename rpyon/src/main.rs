use std::cell::{Ref, RefCell, RefMut};
use std::fmt;
use std::rc::Rc;

fn is_whitespace(ch: char) -> bool {
    ch == ' ' || ch == '\n'
}

fn is_break(ch: char) -> bool {
    is_whitespace(ch) || ch == '(' || ch == ')'
}

struct Tokenizer<'a> {
    input: &'a str,
    input_at: usize,
}

impl<'a> Tokenizer<'a> {
    fn new(input: &str) -> Tokenizer<'_> {
        Tokenizer { input, input_at: 0 }
    }

    fn next_char(self: &mut Tokenizer<'a>) -> Option<char> {
        if self.input_at >= self.input.len() {
            None
        } else {
            let ch = self.input.chars().nth(self.input_at);
            self.input_at += 1;
            ch
        }
    }

    fn skip_whitespace(self: &mut Tokenizer<'a>) {
        let mut ch = self.next_char();
        while ch != None {
            if !is_whitespace(ch.unwrap()) {
                self.input_at -= 1;
                break;
            }
            ch = self.next_char();
        }
    }

    fn next_token(self: &mut Tokenizer<'a>) -> Option<&'a str> {
        self.skip_whitespace();

        let start = self.input_at;
        let mut is_symbol = false;

        let mut ch = self.next_char();
        while ch != None {
            if is_break(ch.unwrap()) {
                if is_symbol {
                    self.input_at -= 1;
                }
                break;
            } else {
                is_symbol = true;
            }
            ch = self.next_char();
        }

        let end = self.input_at;

        if start == end {
            None
        } else {
            self.input.get(start..end)
        }
    }
}

#[derive(Debug, Clone)]
struct Pair {
    car: Value,
    cdr: Value,
}

#[derive(Debug, Clone)]
struct Lambda {
    bindings: Value,
    body: Value,
}

#[derive(Debug, Clone, PartialEq)]
enum BuiltinType {
    Normal,
    Macro,
    SpecialForm,
}

#[derive(Debug, Clone)]
struct Builtin {
    name: Rc<String>,
    ty: BuiltinType,
    func: fn(expr: &Value, context: &mut Context) -> Value,
}

#[derive(Debug, Clone)]
enum Value {
    Nil,
    Integer(isize),
    Symbol(Rc<String>),
    Builtin(Builtin),
    Lambda(Rc<RefCell<Lambda>>),
    Pair(Rc<RefCell<Pair>>),
}

impl Value {
    fn new_pair() -> Value {
        let pair = Pair {
            car: Value::Nil,
            cdr: Value::Nil,
        };

        let ptr = Rc::new(RefCell::new(pair));

        Value::Pair(ptr)
    }

    fn car(&self) -> Option<Ref<Value>> {
        if let Value::Pair(p0) = self {
            Some(Ref::map(p0.borrow(), |p| &p.car))
        } else {
            None
        }
    }

    fn car_mut(&self) -> Option<RefMut<Value>> {
        if let Value::Pair(p0) = self {
            Some(RefMut::map(p0.borrow_mut(), |p| &mut p.car))
        } else {
            None
        }
    }

    fn cdr(&self) -> Option<Ref<Value>> {
        if let Value::Pair(p0) = self {
            Some(Ref::map(p0.borrow(), |p| &p.cdr))
        } else {
            None
        }
    }

    fn new_symbol(s: Rc<String>) -> Value {
        Value::Symbol(s)
    }

    fn new_integer(i: isize) -> Value {
        Value::Integer(i)
    }

    fn new_builtin(b: Builtin) -> Value {
        Value::Builtin(b)
    }
}

struct ValueIter {
    current: Option<Value>,
}

impl ValueIter {
    fn from(v: Value) -> Self {
        Self { current: Some(v) }
    }

    fn next(&mut self) -> Option<Value> {
        let ret = self.current.clone();
        self.current = self
            .current
            .as_ref()
            .and_then(Value::cdr)
            .filter(|v| match v.cdr() {
                None => false,
                Some(v) => {
                    if let Value::Nil = *v {
                        false
                    } else {
                        true
                    }
                }
            })
            .map(|v| (*v).clone());
        ret
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Value::Nil => write!(f, "nil"),
            Value::Integer(i) => write!(f, "i.{i}"),
            Value::Symbol(s) => write!(f, "{s}"),
            Value::Builtin(b) => write!(f, "{}", b.name),
            Value::Lambda(l) => {
                let l = l.borrow();
                write!(f, "l.({} [{}])", l.bindings, l.body)
            }
            Value::Pair(p) => match &*p.borrow() {
                Pair {
                    car: Value::Nil,
                    cdr: Value::Nil,
                } => {
                    write!(f, "()")
                }
                p => {
                    /*
                    write!(f, "({} {})", p.car, p.cdr)
                    */
                    let mut iter = ValueIter::from(self.clone());

                    write!(f, "( ");
                    while let Some(v) = iter.next() {
                        write!(f, "{} ", v.car().unwrap());
                    }
                    write!(f, ")")
                }
            },
        }
    }
}

// ctx ===

#[derive(Debug)]
struct Context {
    symbol_table: Vec<Rc<String>>,
    builtins: Vec<Builtin>,
    ast: Value,
}

impl Context {
    fn add_builtin(
        self: &mut Context,
        name: &str,
        ty: BuiltinType,
        func: fn(&Value, &mut Self) -> Value,
    ) {
        let name_rc = self.get_symbol(name);
        self.builtins.push(Builtin {
            name: name_rc,
            ty,
            func,
        });
    }

    fn get_builtin(self: &Context, name: &str) -> Option<Builtin> {
        self.builtins
            .iter()
            .find(|b| b.name.as_str() == name)
            .cloned()
    }

    fn get_symbol(self: &mut Context, name: &str) -> Rc<String> {
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

    fn parse(self: &mut Context, t: &mut Tokenizer<'_>) {
        let append_value = |stk: &mut Vec<Value>, v: Value| {
            let Value::Pair(ptr) = stk.last().unwrap() else {
                panic!();
            };

            let pair = Value::new_pair();

            _ = ptr.replace(Pair {
                car: v,
                cdr: pair.clone(),
            });

            *stk.last_mut().unwrap() = pair;
        };

        let root = Value::new_pair();

        let mut ast_stack: Vec<Value> = Vec::new();
        ast_stack.push(root.clone());

        while let Some(token) = t.next_token() {
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

    fn process_special_forms(&mut self) {
        let mut iter = ValueIter::from(self.ast.clone());
        while let Some(v) = &iter.next() {
            let new_car = {
                let expr = v.car().unwrap();

                let builtin = expr.car().and_then(|v| {
                    if let Value::Symbol(s) = &*v {
                        self.get_builtin(s.as_str())
                    } else {
                        None
                    }
                });

                builtin.map(|b| (b.func)(&expr, self))
            };

            if let Some(new_car) = new_car {
                *v.car_mut().unwrap() = new_car;
            }
        }
    }
}

// main ===

fn main() {
    let mut ctx = Context {
        symbol_table: Vec::new(),
        builtins: Vec::new(),
        ast: Value::Nil,
    };

    ctx.add_builtin("lambda", BuiltinType::SpecialForm, |v, _| {
        let bindings = v.cdr().as_ref().and_then(|v| v.car()).unwrap().clone();
        let body = v.cdr().as_ref().and_then(|v| v.cdr()).unwrap().clone();
        Value::Lambda(Rc::new(RefCell::new(Lambda { bindings, body })))
    });
    ctx.add_builtin("+", BuiltinType::Normal, |_, _| Value::Nil);

    let input = "(lambda (a b) (+ a b)) (= 2 3)";

    let mut t = Tokenizer::new(input);
    ctx.parse(&mut t);
    println!("{}", ctx.ast);

    ctx.process_special_forms();
    println!("{}", ctx.ast);
}
