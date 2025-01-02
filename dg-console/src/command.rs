// use core::range::Range;
use std::str::Chars;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Command {
    name: String,
    values: Vec<Value>,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Value {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    String(String),
    Array(Vec<Value>),
}

impl Value {
    fn same_kind(&self, o: &Value) -> bool {
        match (self, o) {
            (Value::Integer(_), Value::Integer(_)) => true,
            (Value::Float(_), Value::Float(_)) => true,
            (Value::Boolean(_), Value::Boolean(_)) => true,
            (Value::String(_), Value::String(_)) => true,
            (Value::Array(_), Value::Array(_)) => true,
            _ => false,
        }
    }
}

pub fn parse_command(s: &str) -> Option<Command> {
    Parser::new(s).parse_command()
}

#[derive(Debug)]
enum Base {
    Decimal,
    Hexadecimal,
    Binary,
}

impl Base {
    fn radix(&self) -> u32 {
        match self {
            Base::Decimal => 10,
            Base::Hexadecimal => 16,
            Base::Binary => 2,
        }
    }

    fn offset(&self) -> usize {
        match self {
            Base::Decimal => 0,
            Base::Hexadecimal | Base::Binary => 2,
        }
    }
}

#[derive(Debug)]
enum LiteralKind {
    Int(Base),
    Float,
    String,
}

#[derive(Debug)]
enum TokenKind {
    Ident,
    Whitespace,
    Literal(LiteralKind),
    OpeningBracket,
    ClosingBracket,
    Comma,
    Unknown,
}

#[derive(Debug)]
struct Token<'a> {
    kind: TokenKind,
    // range: Range<usize>,
    s: &'a str,
}

impl<'a> Token<'a> {
    fn is_ident(&self) -> bool {
        match self.kind {
            TokenKind::Ident => true,
            _ => false,
        }
    }

    fn is_whitespace(&self) -> bool {
        match self.kind {
            TokenKind::Whitespace => true,
            _ => false,
        }
    }

    fn is_integer(&self) -> bool {
        match self.kind {
            TokenKind::Literal(LiteralKind::Int(_)) => true,
            _ => false,
        }
    }

    fn is_float(&self) -> bool {
        match self.kind {
            TokenKind::Literal(LiteralKind::Float) => true,
            _ => false,
        }
    }

    fn is_string(&self) -> bool {
        match self.kind {
            TokenKind::Literal(LiteralKind::String) => true,
            _ => false,
        }
    }

    fn is_opening_bracket(&self) -> bool {
        match self.kind {
            TokenKind::OpeningBracket => true,
            _ => false,
        }
    }

    fn is_closing_bracket(&self) -> bool {
        match self.kind {
            TokenKind::ClosingBracket => true,
            _ => false,
        }
    }

    // fn is_comma(&self) -> bool {
    //     match self.kind {
    //         TokenKind::Comma => true,
    //         _ => false,
    //     }
    // }
}

#[derive(Clone)]
struct Tokenizer<'a> {
    s: &'a str,
    chars: Chars<'a>,
    current: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(s: &'a str) -> Self {
        Self {
            s,
            chars: s.chars(),
            current: 0,
        }
    }

    pub fn next_token(&mut self) -> Option<Token<'a>> {
        let start = self.current;
        let next = self.next_char()?;

        let kind = match next {
            '[' => TokenKind::OpeningBracket,
            ']' => TokenKind::ClosingBracket,
            ',' => TokenKind::Comma,
            c if Self::is_whitespace(c) => self.consume_whitespace(),
            c if Self::is_ident_start(c) => self.consume_ident(),
            c if c.is_numeric() => self.consume_number(c),
            '"' => self.consume_string(),
            _ => TokenKind::Unknown,
        };

        let end = self.current;

        Some(Token {
            kind,
            // range: Range { start, end },
            s: &self.s[start..end],
        })
    }

    fn next_char(&mut self) -> Option<char> {
        let c = self.chars.next();
        if c.is_some() {
            self.current += 1;
        }
        c
    }

    fn look_ahead(&self) -> Option<char> {
        self.chars.clone().next()
    }

    fn consume_whitespace(&mut self) -> TokenKind {
        self.consume_while(Self::is_whitespace);
        TokenKind::Whitespace
    }

    fn is_whitespace(c: char) -> bool {
        match c {
            ' ' => true,
            '\n' => true,
            _ => false,
        }
    }

    fn is_ident_start(c: char) -> bool {
        match c {
            'a'..='z' => true,
            'A'..='Z' => true,
            '_' => true,
            _ => false,
        }
    }

    fn is_ident(c: char) -> bool {
        match c {
            'a'..='z' => true,
            'A'..='Z' => true,
            '0'..='9' => true,
            '_' => true,
            _ => false,
        }
    }

    fn consume_while<P>(&mut self, p: P)
    where
        P: Fn(char) -> bool,
    {
        while let Some(c) = self.look_ahead() {
            if p(c) {
                self.next_char();
            } else {
                break;
            }
        }
    }

    fn consume_ident(&mut self) -> TokenKind {
        self.consume_while(Self::is_ident);
        TokenKind::Ident
    }

    fn consume_number(&mut self, start: char) -> TokenKind {
        let base = if start == '0' {
            match self.look_ahead() {
                Some('x') => {
                    self.next_char();
                    Base::Hexadecimal
                }
                Some('b') => {
                    self.next_char();
                    Base::Binary
                }
                _ => Base::Decimal,
            }
        } else {
            Base::Decimal
        };

        match base {
            Base::Decimal => self.consume_while(|c| match c {
                '0'..='9' => true,
                _ => false,
            }),
            Base::Hexadecimal => self.consume_while(|c| match c {
                '0'..='9' => true,
                'a'..='f' => true,
                'A'..='F' => true,
                _ => false,
            }),
            Base::Binary => self.consume_while(|c| match c {
                '0'..='1' => true,
                _ => false,
            }),
        }

        if self.look_ahead() == Some('.')
            && match base {
                Base::Decimal => true,
                _ => false,
            }
        {
            self.next_token();
            self.consume_while(|c| match c {
                '0'..='9' => true,
                _ => false,
            });
            TokenKind::Literal(LiteralKind::Float)
        } else {
            TokenKind::Literal(LiteralKind::Int(base))
        }
    }

    fn consume_string(&mut self) -> TokenKind {
        while let Some(c) = self.look_ahead() {
            if match c {
                '\\' if self.look_ahead() == Some('"') || self.look_ahead() == Some('\\') => {
                    self.next_char();
                    true
                }
                '"' => false,
                _ => true,
            } {
                self.next_char();
            } else {
                // To include the last doublequote
                self.next_char();
                break;
            }
        }
        TokenKind::Literal(LiteralKind::String)
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

pub struct Parser<'a> {
    tokenizer: Tokenizer<'a>,
    token: Token<'a>,
}

impl<'a> Parser<'a> {
    fn new(src: &'a str) -> Self {
        let mut tokenizer = Tokenizer::new(src);
        let token = tokenizer.next().expect("tokenizer has no tokens to parse");
        Self { tokenizer, token }
    }

    fn parse_command(&mut self) -> Option<Command> {
        let name = self.parse_ident()?;
        self.next();
        let mut values = Vec::new();

        loop {
            if !self.next() {
                break;
            }

            if self.token.is_whitespace() {
                continue;
            }

            if let Some(value) = self.parse_value() {
                values.push(value);
            } else {
                break;
            }
        }

        Some(Command { name, values })
    }

    fn parse_ident(&mut self) -> Option<String> {
        if !self.token.is_ident() {
            return None;
        }

        Some(self.token.s.to_owned())
    }

    fn parse_integer(&mut self) -> Option<Value> {
        if !self.token.is_integer() {
            return None;
        }

        let v = match &self.token.kind {
            TokenKind::Literal(LiteralKind::Int(base)) => {
                i64::from_str_radix(&self.token.s[base.offset()..], base.radix()).unwrap()
            }
            _ => unreachable!(),
        };

        Some(Value::Integer(v))
    }

    fn parse_float(&mut self) -> Option<Value> {
        if !self.token.is_float() {
            return None;
        }

        Some(Value::Float(self.token.s.parse().unwrap()))
    }

    fn parse_bool(&mut self) -> Option<Value> {
        if !self.token.is_ident() {
            return None;
        }

        match self.token.s.to_lowercase().as_str() {
            "true" => Some(Value::Boolean(true)),
            "false" => Some(Value::Boolean(false)),
            _ => None,
        }
    }

    fn parse_string(&mut self) -> Option<Value> {
        if !self.token.is_string() {
            return None;
        }

        let end = self.token.s.len() - 1;
        Some(Value::String(self.token.s[1..end].to_owned()))
    }

    fn parse_value(&mut self) -> Option<Value> {
        self.parse_float()
            .or(self.parse_integer())
            .or(self.parse_bool())
            .or(self.parse_string())
            .or(self.parse_array())
    }

    fn parse_array(&mut self) -> Option<Value> {
        if !self.token.is_opening_bracket() {
            return None;
        }
        self.next().then_some(())?;
        while self.token.is_whitespace() {
            self.next().then_some(())?;
        }

        let mut first: Option<Value> = None;
        let mut values = Vec::new();
        while !self.token.is_closing_bracket() {
            let value = self.parse_value()?;
            match &first {
                Some(v) => if !v.same_kind(&value) {
                    return None;
                },
                None => first = Some(value.clone()),
            }

            values.push(value);

            self.next().then_some(())?;
            self.skip_whitespace();

            match self.token.kind {
                TokenKind::Comma => {
                    self.next().then_some(())?;
                    self.skip_whitespace();
                    if self.token.is_closing_bracket() {
                        break;
                    }
                },
                TokenKind::ClosingBracket => break,
                _ => return None,
            }
        }

        Some(Value::Array(values))
    }

    fn skip_whitespace(&mut self) -> bool {
        let whitespace = self.token.is_whitespace();
        while self.token.is_whitespace() {
            if !self.next() {
                return true;
            }
        }
        whitespace
    }

    fn next(&mut self) -> bool {
        match self.tokenizer.next() {
            Some(t) => {
                self.token = t;
                true
            }
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::command::{Parser, TokenKind};

    use super::{Command, Tokenizer, Value};

    #[test]
    fn test_tokenizer() {
        let s = "function true false 16.666 5 0x01 0b01 \"frametime\"";
        println!("{}", s);
        let tokenizer = Tokenizer::new(s);

        for token in tokenizer {
            match token.kind {
                TokenKind::Unknown => panic!("unknown token found: {:?}", token),
                _ => {}
            }
        }
    }

    #[test]
    fn test_parser() {
        let s = "function true false 16.666 5 0x01 0b01 \"frametime\" [true,false,true,true,true,true]";
        let mut parser = Parser::new(s);
        let r = parser.parse_command();
        let e = Command {
            name: "function".into(),
            values: vec![
                Value::Boolean(true),
                Value::Boolean(false),
                Value::Float(16.666),
                Value::Integer(5),
                Value::Integer(0x01),
                Value::Integer(0b01),
                Value::String("frametime".into()),
                Value::Array(vec![
                    Value::Boolean(true),
                    Value::Boolean(false),
                    Value::Boolean(true),
                    Value::Boolean(true),
                    Value::Boolean(true),
                    Value::Boolean(true),
                ]),
            ],
        };
        assert_eq!(r, Some(e));
        
        let s = "function";
        let mut parser = Parser::new(s);
        let r = parser.parse_command();
        let e = Command {
            name: "function".into(),
            values: vec![],
        };
        assert_eq!(r, Some(e));
        
    }
}
