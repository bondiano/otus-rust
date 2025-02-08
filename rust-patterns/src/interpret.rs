pub struct Interpreter<'a> {
    it: std::str::Chars<'a>,
    result: i32,
}

type Token = char;

/// Simple LISP-like summation interpreter
impl<'a> Interpreter<'a> {
    pub fn new(str: &'a str) -> Self {
        Self {
            it: str.chars(),
            result: 0,
        }
    }

    pub fn interpret(&mut self) -> i32 {
        while let Some(token) = self.next_token() {
            self.parse_token(token);
        }

        self.result
    }

    fn next_token(&mut self) -> Option<Token> {
        self.it.next()
    }

    fn parse_token(&mut self, token: Token) {
        match token {
            '(' | '+' | ' ' | ')' => (),
            token if token.is_ascii_digit() => self.read_number(token.to_digit(10).unwrap() as i32),
            _ => panic!("Invalid token: {}", token),
        }
    }

    fn read_number(&mut self, current: i32) {
        let mut number = current;
        while let Some(c) = self.next_token() {
            if c.is_ascii_digit() {
                number = number * 10 + (c as i32 - '0' as i32);
            } else {
                break;
            }
        }
        self.result += number;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interpret() {
        let mut interpreter = Interpreter::new("(+ 1 2)");
        assert_eq!(interpreter.interpret(), 3);
    }

    #[test]
    fn test_interpret_complex() {
        let mut interpreter = Interpreter::new("(+ 1 2 3 4 5)");
        assert_eq!(interpreter.interpret(), 15);
    }
}
