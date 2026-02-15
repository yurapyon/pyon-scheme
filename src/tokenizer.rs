fn is_whitespace(ch: char) -> bool {
    ch == ' ' || ch == '\n'
}

fn is_break(ch: char) -> bool {
    is_whitespace(ch) || ch == '(' || ch == ')'
}

pub struct Tokenizer<'a> {
    input: &'a str,
    input_at: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &str) -> Tokenizer<'_> {
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
        while let Some(ch) = self.next_char() {
            if !is_whitespace(ch) {
                self.input_at -= 1;
                break;
            }
        }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = &'a str;
    fn next(self: &mut Tokenizer<'a>) -> Option<Self::Item> {
        self.skip_whitespace();

        let start = self.input_at;
        let mut is_symbol = false;

        while let Some(ch) = self.next_char() {
            if is_break(ch) {
                if is_symbol {
                    self.input_at -= 1;
                }
                break;
            } else {
                is_symbol = true;
            }
        }

        let end = self.input_at;

        if start == end {
            None
        } else {
            self.input.get(start..end)
        }
    }
}
