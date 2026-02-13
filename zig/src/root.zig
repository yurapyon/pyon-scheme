const std = @import("std");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;

// exports ===

pub const Tokenizer = @import("tokenizer.zig").Tokenizer;
pub const Parser = @import("parser.zig").Parser;
pub const Cons = @import("cons.zig").Cons;

// ===

pub const Environment = struct {
    allocator: Allocator,
    stack: ArrayList(struct {
        symbol: []const u8,
        value: *Value,
    }),

    pub fn init(self: *@This(), allocator: Allocator) void {
        self.allocator = allocator;
    }

    pub fn push(self: *@This(), symbol: []const u8, value: *Value) !void {
        try self.stack.append(
            self.allocator,
            .{
                .symbol = symbol,
                .value = value,
            },
        );
    }

    pub fn pop(self: *@This(), count: usize) void {
        // TODO
        _ = self;
        _ = count;
    }

    pub fn find(self: *@This(), symbol: []const u8) ?*Value {
        // TODO
        _ = self;
        _ = symbol;
        return null;
    }
};

fn assoc(alist: Cons, symbol: []const u8) ?Value {
    const alist_iter = alist.iterator();
    while (alist_iter.next()) |p| {
        const pair = p.car;
        const key = pair.cons.car.symbol;
        if (std.mem.eql(u8, key, symbol)) {
            return pair.cons.cdr;
        }
    }

    return null;
}

pub const Lambda = struct {
    bindings: Value,
    body: Value,

    pub fn apply(self: @This(), args: Value, environment: Value) @This() {
        const initial_env_cons = environment.cons;

        var curr_env = environment.cons;
        var bindings_iter = self.bindings.iterator();
        var args_iter = args.cons.iterator();
        while (bindings_iter.next()) |b| {
            const arg_value = args_iter.next();
            if (arg_value) |a| {
                const pair: Cons = .{
                    .car = b.car,
                    .cdr = a.car,
                };
                const new_cons = undefined;
                curr_env.cdr = new_cons;
                new_cons.* = .{
                    .car = pair,
                    .cdr = .nil,
                };
                curr_env = new_cons;
            }
        }

        const ret = self.body.eval(curr_env);

        initial_env_cons.cdr = .nil;

        return ret;
    }
};

pub const Value = union(enum) {
    nil,
    integer: isize,
    symbol: []const u8,
    // TODO quoted symbol
    // q_symbol: []const u8,
    // TODO
    builtin: void,
    lambda: *Lambda,
    cons: *Cons,

    pub fn eval(self: @This(), environment: Value) @This() {
        switch (self) {
            .cons => |c| {
                const func = c.car;
                const args = c.cdr;

                const from_env = func.eval(environment);

                switch (from_env) {
                    .builtin => |b| {
                        return b.apply(args, environment);
                    },
                    .lambda => |l| {
                        return l.apply(args, environment);
                    },
                    else => {
                        // TODO invalid type error
                    },
                }
            },
            .symbol => |s| {
                if (assoc(environment.cons, s)) |from_env| {
                    return from_env;
                } else {
                    //TODO error
                }
            },
            else => return self,
        }
    }

    pub fn prettyPrint(self: *@This(), indent: usize) void {
        switch (self.*) {
            .integer => {},
            .symbol => |str| {
                std.debug.print("{s}", .{str});
            },
            .cons => |cons| {
                std.debug.print("\n", .{});
                for (0..indent) |_| {
                    std.debug.print(" ", .{});
                }
                std.debug.print("(", .{});
                cons.car.prettyPrint(indent + 2);
                cons.cdr.prettyPrint(indent + 2);
                std.debug.print(")", .{});
            },
            else => {
                // TODO
            },
        }
    }
};

const SymbolTable = struct {
    allocator: Allocator,
    strings: ArrayList([]u8),

    fn init(self: *@This(), allocator: Allocator) void {
        self.allocator = allocator;
        self.strings = .empty;
    }

    fn deinit(self: *@This()) void {
        // TODO free all strings
        _ = self;
    }

    fn intern(str: []const u8) void {
        _ = str;
    }

    fn getId(str: []const u8) usize {
        _ = str;
        // TODO
        // try to find string in strings
        // if not found
        //   intern
        // return id
    }
};

pub const Executor = struct {
    allocator: Allocator,
    ast: Value,
    env: Environment,

    pub fn init(self: *@This(), allocator: Allocator) void {
        self.allocator = allocator;
        self.env.init(self.allocator);
    }

    pub fn execute(self: *@This(), operator: *Value, args: ?*Value) !*Value {
        switch (operator.*) {
            .nil => {},
            .integer => {
                return error.Error;
            },
            .symbol => |s| {
                const in_env = self.env.find(s);
                if (in_env) |v| {
                    return v;
                } else {
                    return error.SymbolNotFound;
                }
            },
            .builtin => |b| {
                // return b(&self.env, args.?.cons);
                _ = b;
                return error.Error;
            },
            .lambda => |*l| {
                var bindings_iter = l.bindings.iterator();
                // TODO handle args == null
                var args_iter = args.?.cons.iterator();
                while (bindings_iter.next()) |b| {
                    const arg_value = args_iter.next();
                    if (arg_value) |a| {
                        // TODO handle binding is null
                        // TODO make sure binding is a symbol
                        try self.env.push(b.car.?.symbol, a.car.?);
                    }
                }

                // TODO handle body.car == null
                const ret = try self.execute(l.body.car.?, l.body.cdr);
                // TODO pop count from env
                return ret;
            },
            .cons => {
                return error.Error;
            },
        }
    }
};

pub const MacroEvaluator = struct {
    ast: Value,

    // pub fn transform()
};
