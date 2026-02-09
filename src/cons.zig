const Value = @import("root.zig").Value;

// ===

pub const Cons = struct {
    const Iterator = struct {
        current: ?*Cons,

        fn init(start: *Cons) @This() {
            return .{
                .current = start,
            };
        }

        pub fn next(self: *@This()) ?*Cons {
            if (self.current) |curr| {
                if (curr.cdr == .nil) {
                    self.current = null;
                    return null;
                } else {
                    const prev = self.current;
                    // TODO check type error
                    self.current = curr.cdr.cons;
                    return prev;
                }
            } else {
                return null;
            }
        }
    };

    car: Value,
    cdr: Value,

    pub fn iterator(self: *@This()) Iterator {
        return Iterator.init(self);
    }
};
