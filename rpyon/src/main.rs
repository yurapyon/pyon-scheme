#![allow(unused)]

use crate::context::Context;
use crate::tokenizer::Tokenizer;
use crate::value::{BuiltinType, Lambda, Value};

mod context;
mod tokenizer;
mod value;

// ===

fn main() {
    let mut ctx = Context {
        symbol_table: Vec::new(),
        builtins: Vec::new(),
        ast: Value::Nil,
    };

    ctx.add_builtin("lambda", BuiltinType::SpecialForm, |v, _| {
        let bindings = v.cdr().and_then(|v| v.car()).unwrap();
        let body = v.cdr().and_then(|v| v.cdr()).unwrap();
        Value::new_lambda(Lambda { bindings, body })
    });
    ctx.add_builtin("+", BuiltinType::Normal, |_, _| Value::Nil);

    let input = "(lambda xyz (lambda abc (xyz abc))) (+ 2 3)";

    let mut t = Tokenizer::new(input);
    ctx.parse(&mut t);
    println!("{}", ctx.ast);

    ctx.process_special_forms();
    println!("{}", ctx.ast);
}
