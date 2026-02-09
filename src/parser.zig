const std = @import("std");
const Allocator = std.mem.Allocator;
const ArrayList = std.ArrayList;

const Value = @import("root.zig").Value;
const Cons = @import("root.zig").Cons;

// ===

pub const Parser = struct {
    allocator: Allocator,
    root: Value,
    ast_stack: ArrayList(Value),

    pub fn init(self: *@This(), allocator: Allocator) !void {
        self.allocator = allocator;

        const root_cons = try self.allocator.create(Cons);
        root_cons.* = .{
            .car = .nil,
            .cdr = .nil,
        };
        self.root = .{
            .cons = root_cons,
        };

        self.ast_stack = .empty;
        try self.ast_stack.append(self.allocator, self.root);
    }

    pub fn deinit() void {
        // TODO free tree
    }

    fn appendValueToCurrentCons(self: *@This(), value_to_append: Value) !void {
        const new_cons = try self.allocator.create(Cons);
        new_cons.* = .{
            .car = .nil,
            .cdr = .nil,
        };

        const new_value = Value{
            .cons = new_cons,
        };

        self.ast_stack.items[self.ast_stack.items.len - 1].cons.* = .{
            .car = value_to_append,
            .cdr = new_value,
        };

        self.ast_stack.items[self.ast_stack.items.len - 1] = new_value;
    }

    pub fn parse(self: *@This(), token: []const u8) !void {
        if (std.mem.eql(u8, token, "(")) {
            const new_cons = try self.allocator.create(Cons);
            new_cons.* = .{
                .car = .nil,
                .cdr = .nil,
            };

            const new_value = Value{
                .cons = new_cons,
            };

            try self.appendValueToCurrentCons(new_value);

            try self.ast_stack.append(self.allocator, new_value);
        } else if (std.mem.eql(u8, token, ")")) {
            // TODO
            // error if popping root
            _ = self.ast_stack.pop();
        } else {
            // TODO
            // intern symbols
            // handle numbers
            // handle builtins
            try self.appendValueToCurrentCons(.{
                .symbol = token,
            });
        }
    }
};
