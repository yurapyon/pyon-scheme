const std = @import("std");
const pyon_scheme = @import("pyon_scheme");

const Cons = pyon_scheme.Cons;
const Value = pyon_scheme.Value;
const Tokenizer = pyon_scheme.Tokenizer;
const Parser = pyon_scheme.Parser;
const Executor = pyon_scheme.Executor;
const Environment = pyon_scheme.Environment;

// ===

fn add(env: *Environment, body: Cons) *Value {
    _ = body;
    return env.allocator.create(Value) catch unreachable;
}

pub fn main() !void {
    const allocator = std.heap.c_allocator;

    var t: Tokenizer = .empty;
    t.setInput(
        \\ (list (0 . 1) (2 . 3))
        //         \\ (1 2 3 + +) (10 20 +) +
        \\ (def thingy (_w _x _y) w x y)
        //         \\ (def thingy _w_x_y w x y)
    );

    var p: Parser = undefined;
    try p.init(allocator);

    while (t.nextToken()) |token| {
        // std.debug.print("token {s}\n", .{token});
        try p.parse(token);
    }

    // p.root.prettyPrint(0);
    // std.debug.print("\n", .{});

    var root_iter = p.root.cons.iterator();
    while (root_iter.next()) |c| {
        c.car.prettyPrint(0);
    }

    //
    //     var e: Executor = undefined;
    //     e.init(allocator);
    //     // var a = Value{ .builtin = add };
    //     // try e.env.push("+", &a);
    //
    //     while (root_iter.next()) |c| {
    //         if (c.car) |v| {
    //             _ = try e.execute(v, c.cdr);
    //         }
    //     }
}
