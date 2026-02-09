// Splits the input on:
//   ( ) symbol whitespace
pub const Tokenizer = struct {
    pub const empty = @This(){
        .input = "",
        .input_at = 0,
    };

    input: []const u8,
    input_at: usize,

    pub fn setInput(self: *@This(), input: []const u8) void {
        self.input = input;
        self.input_at = 0;
    }

    fn nextChar(self: *@This()) ?u8 {
        if (self.input_at >= self.input.len) {
            return null;
        }

        const ch = self.input[self.input_at];
        self.input_at += 1;
        return ch;
    }

    fn skipWhitespace(self: *@This()) void {
        while (self.nextChar()) |ch| {
            if (!(ch == ' ' or ch == '\n')) {
                self.input_at -= 1;
                break;
            }
        }
    }

    pub fn nextToken(self: *@This()) ?[]const u8 {
        self.skipWhitespace();

        const start = self.input_at;
        var is_symbol = false;

        while (self.nextChar()) |ch| {
            switch (ch) {
                '(', ')', ' ', '\n' => {
                    if (is_symbol) {
                        self.input_at -= 1;
                    }
                    break;
                },
                else => {
                    is_symbol = true;
                },
            }
        }

        const end = self.input_at;

        if (start == end) {
            return null;
        } else {
            return self.input[start..end];
        }
    }
};
