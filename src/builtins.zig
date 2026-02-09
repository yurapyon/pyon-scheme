const Context = void;
const Value = void;
const Lambda = void;

pub const Builtin = struct {
    // const Callback = *const fn (env: *Environment, body: Cons) *Value;
    const Callback = void;

    name: []const u8,
    callback: Callback,
    is_macro: bool,
};

pub const builtins = [_]Builtin{
    .{
        .name = "lambda",
        .callback = &lambda,
        .is_macro = true,
    },
};

fn lambda(ctx: *Context, expr: *Value) Value {
    const new_lambda = ctx.allocator.create(Lambda);

    new_lambda.bindings = expr.cons.cdr.cons.car;
    new_lambda.body = expr.cons.cdr.cons.cdr;
    new_lambda.environment = .nil;

    return new_lambda;
}
