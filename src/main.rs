#![allow(unused)]

use crate::context::Context;
use crate::tokenizer::Tokenizer;
use crate::value::{BuiltinType, Lambda, Value, ValueListIter, ValueTreeIter};

mod context;
mod tokenizer;
mod value;

// ===

fn main() {
    let mut ctx = Context::new();

    ctx.add_builtin("lambda", BuiltinType::SpecialForm, |v, _, _| {
        let bindings = v.cdr().car();
        let body = v.cdr().cdr();
        Value::new_lambda(Lambda { bindings, body })
    });
    ctx.add_builtin("+", BuiltinType::Normal, |args, ctx, env| {
        let iter = ValueListIter::from(args.clone());
        let mut ct = 0;
        for v in iter {
            /*
            let from_env = ctx.eval(&v.car().unwrap(), env);
            match from_env {
                Value::Integer(i) => ct += i,
                // TODO handle error
                _ => (),
            }
            */
            match v.car() {
                Value::Integer(i) => ct += i,
                // TODO handle error
                _ => (),
            }
        }
        Value::new_integer(ct)
    });

    // let input = "((lambda (a b) (+ a b )) (lambda (c d) (+ c d)) 150) 1 2 3";
    // let input = "(+ 1 2)";
    let input = "((lambda () (+ 1 2)))";

    let mut t = Tokenizer::new(input);
    ctx.parse(&mut t);

    /*
    let i = ValueTreeIter::from(ctx.ast.clone());
    for value in i {
        println!(">> {}\n", value);
    }
    */

    ctx.process_special_forms();
    println!("ast:\n{}\nenv:\n{}", ctx.ast, ctx.global_environment);

    let env = ctx.global_environment.clone();

    let v = ctx.eval(&ctx.ast.car(), &env);
    println!("result: {}", v);
}
