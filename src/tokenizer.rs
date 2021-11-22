use crate::syntax::SyntaxKind;

#[derive(Debug)]
pub struct Tokenizer<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Tokenizer<'a> {
        Tokenizer { input, pos: 0 }
    }

    pub fn end_of_file(&self) -> bool {
        self.pos >= self.input.len()
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = (SyntaxKind, String);

    fn next(&mut self) -> Option<Self::Item> {
        if self.end_of_file() {
            return None;
        }

        match self.input.as_bytes().get(self.pos) {
            Some(b'+') => {
                self.pos += 1;
                return Some((SyntaxKind::ADD, "+".into()));
            }
            Some(b'-') => {
                self.pos += 1;
                return Some((SyntaxKind::SUB, "-".into()));
            }
            Some(b'*') => {
                self.pos += 1;
                return Some((SyntaxKind::MUL, "*".into()));
            }
            Some(b'/') => {
                self.pos += 1;
                return Some((SyntaxKind::DIV, "/".into()));
            }
            Some(b' ' | b'\t' | b'\r') => {
                let start = self.pos;
                self.pos += 1;

                while matches!(
                    self.input.as_bytes().get(self.pos),
                    Some(b' ' | b'\t' | b'\r')
                ) {
                    self.pos += 1;
                }

                return Some((SyntaxKind::WHITESPACE, self.input[start..=self.pos-1].into()));
            }
            Some(b'0'..=b'9') => {
                let start = self.pos;
                self.pos += 1;

                while matches!(self.input.as_bytes().get(self.pos), Some(b'0'..=b'9')) {
                    self.pos += 1;
                }

                if matches!(self.input[self.pos..].as_bytes(), [b'.', b'0'..=b'9', ..]) {
                    self.pos += 2;
                    while matches!(self.input.as_bytes().get(self.pos), Some(b'0'..=b'9')) {
                        self.pos += 1;
                    }
                }

                return Some((SyntaxKind::NUMBER, self.input[start..=self.pos-1].into()));
            }
            Some(_) => unreachable!(),
            None => return None,
        };
    }
}
