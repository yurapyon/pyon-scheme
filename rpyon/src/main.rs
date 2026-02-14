use std::cell::RefCell;
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
struct LispPtr<T> {
    contents: Option<Rc<RefCell<T>>>,
}

impl<T> LispPtr<T> {
    fn new(value: T) -> LispPtr<T> {
        LispPtr {
            contents: Some(Rc::new(RefCell::new(value))),
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

#[derive(Debug, Clone)]
enum Value {
    Nil,
    Integer(isize),
    Symbol(Rc<String>),
    Lambda(LispPtr<Lambda>),
    Pair(LispPtr<Pair>),
}

impl Value {
    fn new_pair() -> Value {
        let pair = Pair {
            car: Value::Nil,
            cdr: Value::Nil,
        };

        let ptr = LispPtr::new(pair);

        Value::Pair(ptr)
    }

    fn new_symbol(s: Rc<String>) -> Value {
        Value::Symbol(s.clone())
    }

    fn new_integer(i: isize) -> Value {
        Value::Integer(i)
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Value::Nil => write!(f, "nil"),
            Value::Integer(i) => write!(f, "i:{i}"),
            Value::Symbol(s) => write!(f, "s:{s}"),
            Value::Lambda(l) => Ok(()),
            Value::Pair(p) => {
                let ptr = p.contents.as_ref().unwrap().borrow();
                write!(f, "({} {})", ptr.car, ptr.cdr)
            }
        }
    }
}

// ctx ===

struct Context {
    symbol_table: Vec<Rc<String>>,
    ast: Value,
}

impl Context {
    fn get_symbol(self: &mut Context, name: &str) -> Rc<String> {
        let maybe_rc = self.symbol_table.iter().find(|s| s.as_str() == name);

        match maybe_rc {
            Some(rc) => rc.clone(),
            None => {
                let rc = Rc::new(String::from(name));
                self.symbol_table.push(rc.clone());
                rc.clone()
            }
        }
    }

    fn parse(self: &mut Context, t: &mut Tokenizer<'_>) {
        let append_value = |stk: &mut Vec<Value>, v: &Value| {
            let pair = Value::new_pair();

            {
                let top = stk.last().unwrap();
                let Value::Pair(ptr) = top else {
                    panic!();
                };

                let mut ptr_mut = ptr.contents.as_ref().unwrap().borrow_mut();

                ptr_mut.car = v.clone();
                ptr_mut.cdr = pair.clone();
            }

            let at = stk.len() - 1;
            stk[at] = pair.clone();
        };

        let root = Value::new_pair();

        let mut ast_stack: Vec<Value> = Vec::new();
        ast_stack.push(root.clone());

        let mut token = t.next_token();
        while token != None {
            match token.unwrap() {
                "(" => {
                    let new_list = Value::new_pair();
                    append_value(&mut ast_stack, &new_list);
                    ast_stack.push(new_list.clone());
                }
                // TODO
                //  check ast_stack len, shouldnt be < 1
                ")" => _ = ast_stack.pop(),
                s => match s.parse::<isize>() {
                    Ok(i) => {
                        append_value(&mut ast_stack, &Value::new_integer(i));
                    }
                    _ => {
                        let symbol = self.get_symbol(s);
                        append_value(&mut ast_stack, &Value::new_symbol(symbol));
                    }
                },
            }
            token = t.next_token();
        }

        // TODO
        //  check ast_stack len, should be 1
        self.ast = root;
    }
}

// main ===

fn main() {
    let mut ctx = Context {
        symbol_table: Vec::new(),
        ast: Value::Nil,
    };

    // let mut t = Tokenizer::new("(+ (* 3) (* 6))");
    let mut t = Tokenizer::new("(lambda (a b) (+ a b))");
    ctx.parse(&mut t);
    println!("{}", ctx.ast);
}
