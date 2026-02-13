const pyon_scheme = @import("root.zig");
const Value = pyon_scheme.Value;
const Lambda = pyon_scheme.Lambda;

// ===

pub const Builtin = struct {
    const Callback = *const fn (args: Value, environment: Value) Value;

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

fn lambda(args: Value, environment: Value) Value {
    _ = environment;

    const new_lambda = undefined;
    // TODO
    // ctx.allocator.create(Lambda);

    new_lambda.bindings = args.cons.cdr.cons.car;
    new_lambda.body = args.cons.cdr.cons.cdr;
    new_lambda.environment = .nil;

    return new_lambda;
}
