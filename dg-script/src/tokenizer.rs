use std::{fmt::Display, iter::Peekable, ops::Range, str::Chars};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Base {
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LiteralKind {
    Int(Base),
    Float,
    String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenKind {
    Type,
    Ident,
    Call,
    Comment(bool),
    Whitespace,
    Literal(LiteralKind),

    Semicolon,      // ;
    OpeningBracket, // [
    ClosingBracket, // ]

    Dot,      // .
    Equals,   // =
    Plus,     // +
    Minus,    // -
    Asterisk, // *
    Slash,    // /
    Greater,  // >
    Less,     // <
    Not,      // !
    Neg,      // ~
    Caret,    // ^
    Percent,  // %
    Dollar,   // $

    DoubleEquals,  // ==
    NotEquals,     // !=
    GreaterEquals, // >=
    LessEquals,    // <=
    PlusEquals,    // +=
    MinusEquals,   // -=
    TimesEquals,   // *=
    DivideEquals,  // /=
    Or,            // ||
    And,           // &&

    Returns,  // ->
    WithBody, // =>

    Unknown,

    Eoi,
}

#[derive(Debug)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub span: Range<usize>,
    pub s: &'a str,
}

impl Default for Token<'static> {
    fn default() -> Self {
        Self {
            kind: TokenKind::Eoi,
            span: Default::default(),
            s: Default::default(),
        }
    }
}

impl<'a> Display for Token<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} = `{}`", self.kind, self.s.escape_debug())
    }
}

#[derive(Clone)]
pub struct Tokenizer<'a> {
    s: &'a str,
    chars: Peekable<Chars<'a>>,
    current: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(s: &'a str) -> Self {
        Self {
            s,
            chars: s.chars().peekable(),
            current: 0,
        }
    }

    pub fn next_token(&mut self) -> Token<'a> {
        let start = self.current;
        let next = match self.next_char() {
            Some(c) => c,
            None => {
                return Token {
                    kind: TokenKind::Eoi,
                    s: "",
                    span: Range::default(),
                }
            }
        };

        let kind = match (next, self.look_ahead()) {
            ('#', _) => self.consume_comment(),
            c if Self::is_whitespace(c.0) => self.consume_whitespace(),
            c if Self::is_type_start(c.0) => self.consume_type(),
            c if Self::is_ident(c.0) => self.consume_ident(),
            c if c.0.is_numeric() => self.consume_number(c.0),
            ('=', Some('=')) => {
                self.next();
                TokenKind::DoubleEquals
            }
            ('!', Some('=')) => {
                self.next();
                TokenKind::NotEquals
            }
            ('>', Some('=')) => {
                self.next();
                TokenKind::GreaterEquals
            }
            ('<', Some('=')) => {
                self.next();
                TokenKind::LessEquals
            }
            ('+', Some('=')) => {
                self.next();
                TokenKind::PlusEquals
            }
            ('-', Some('=')) => {
                self.next();
                TokenKind::MinusEquals
            }
            ('*', Some('=')) => {
                self.next();
                TokenKind::TimesEquals
            }
            ('/', Some('=')) => {
                self.next();
                TokenKind::DivideEquals
            }
            ('-', Some('>')) => {
                self.next();
                TokenKind::Returns
            }
            ('=', Some('>')) => {
                self.next();
                TokenKind::WithBody
            }
            ('|', Some('|')) => {
                self.next();
                TokenKind::Or
            }
            ('&', Some('&')) => {
                self.next();
                TokenKind::And
            }
            ('@', _) => self.consume_call(),
            ('"', _) => self.consume_string(),
            ('.', _) => TokenKind::Dot,
            ('=', _) => TokenKind::Equals,
            ('+', _) => TokenKind::Plus,
            ('-', _) => TokenKind::Minus,
            ('*', _) => TokenKind::Asterisk,
            ('/', _) => TokenKind::Slash,
            ('>', _) => TokenKind::Greater,
            ('<', _) => TokenKind::Less,
            ('!', _) => TokenKind::Not,
            ('~', _) => TokenKind::Neg,
            ('^', _) => TokenKind::Caret,
            ('%', _) => TokenKind::Percent,
            ('$', _) => TokenKind::Dollar,
            (';', _) => TokenKind::Semicolon,
            ('[', _) => TokenKind::OpeningBracket,
            (']', _) => TokenKind::ClosingBracket,
            _ => TokenKind::Unknown,
        };

        let end = self.current;

        Token {
            kind,
            span: Range { start, end },
            s: &self.s[start..end],
        }
    }

    fn next_char(&mut self) -> Option<char> {
        let c = self.chars.next();
        if c.is_some() {
            self.current += 1;
        }
        c
    }

    fn look_ahead(&mut self) -> Option<char> {
        self.chars.peek().copied()
    }

    fn consume_whitespace(&mut self) -> TokenKind {
        self.consume_while(Self::is_whitespace);
        TokenKind::Whitespace
    }

    fn is_whitespace(c: char) -> bool {
        c.is_whitespace()
    }

    fn is_type_start(c: char) -> bool {
        match c {
            'A'..='Z' => true,
            _ => false,
        }
    }

    fn is_type(c: char) -> bool {
        match c {
            'a'..='z' => true,
            'A'..='Z' => true,
            '0'..='9' => true,
            '.' => true,
            _ => false,
        }
    }

    fn is_ident(c: char) -> bool {
        match c {
            'a'..='z' => true,
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

    fn consume_comment(&mut self) -> TokenKind {
        let doc = match self.look_ahead() {
            Some('#') => true,
            _ => false,
        };

        self.consume_while(|c: char| match c {
            '\n' => false,
            _ => true,
        });
        self.next();

        TokenKind::Comment(doc)
    }

    fn consume_type(&mut self) -> TokenKind {
        self.consume_while(Self::is_type);
        TokenKind::Type
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

    fn consume_call(&mut self) -> TokenKind {
        self.consume_while(|c: char| match c {
            'A'..='Z' => true,
            'a'..='z' => true,
            '.' => true,
            _ => false,
        });
        TokenKind::Call
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
        Some(self.next_token())
    }
}

#[cfg(test)]
mod tests {
    use super::{TokenKind, Tokenizer};

    #[test]
    fn test_example() {
        let s = include_str!("scripts/example.dg");
        let tokenizer = Tokenizer::new(s);

        for token in tokenizer {
            match token.kind {
                TokenKind::Unknown => panic!("unknown token: {}", token),
                TokenKind::Whitespace => {}
                TokenKind::Eoi => break,
                _ => println!("{}", token),
            }
        }
    }
}
