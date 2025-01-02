use std::collections::HashMap;

use crate::tokenizer::{Token, TokenKind, Tokenizer};

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Ident {
    ident: String,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct TypeName {
    name: String,
}

#[derive(Debug)]
pub struct Import {
    script: Ident,
}

#[derive(Debug, Clone)]
pub struct Variable {
    name: Ident,
    ty: TypeName,
}

#[derive(Debug)]
pub enum ExpressionKind {
    Call { ty: TypeName },
    Match { on: Vec<Ident> },
}

#[derive(Debug)]
pub struct Statement {
    kind: ExpressionKind,
}

// TODO: This...
#[derive(Debug)]
pub struct Body {
    statements: Vec<Statement>,
}

#[derive(Debug)]
pub struct AliasType {
    name: TypeName,
    ty: TypeName,
}

#[derive(Debug)]
pub struct SumType {
    name: TypeName,
    variants: Vec<ProductType>,
}

#[derive(Debug)]
pub struct ProductType {
    name: TypeName,
    variables: Vec<Variable>,
}

#[derive(Debug)]
pub struct ExponentType {
    name: TypeName,
    parameters: Vec<Variable>,
    ret_ty: Option<TypeName>,
    body: Option<Body>,
}

#[derive(Debug)]
pub enum Item {
    Import(Import),
    Alias(AliasType),
    Sum(SumType),
    Product(ProductType),
    Exponent(ExponentType),
}

#[derive(Debug)]
pub struct Script {
    items: Vec<Item>,
}

#[derive(Debug)]
pub struct ExponentSignature<'a> {
    tokens: Vec<Token<'a>>,
    parameters: Vec<Variable>,
}

pub struct Parser<'a> {
    exponent_signatures: HashMap<TypeName, ExponentSignature<'a>>,
    tokenizer: Tokenizer<'a>,
    peek: Token<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(src: &'a str) -> Self {
        let tokenizer = Tokenizer::new(src);
        // let cur = Token::default();
        let peek = Token::default();

        let mut s = Self {
            exponent_signatures: HashMap::new(),
            tokenizer,
            // cur,
            peek,
        };
        s.next_token();
        s
    }

    pub fn parse_script(&mut self) -> Option<Script> {
        let mut items = Vec::new();

        // First pass: to parse everything except for exponent bodies
        // as they require information about other exponent items parameters
        // for when calling them.
        while let Some(item) = self.parse_item() {
            items.push(item);
        }

        // Second pass: Parse exponent bodies using the now known exponent parameters.
        items.iter_mut().filter_map(|i| match i {
            Item::Exponent(e) => Some(e),
            _ => None,
        }).for_each(|e| {
            e.body = self.parse_body();
        });

        Some(Script { items })
    }

    pub fn parse_item(&mut self) -> Option<Item> {
        let token = self.next_token()?;
        let item = match token.kind {
            TokenKind::Caret => Item::Exponent(self.parse_exponent_type()?),
            TokenKind::Asterisk => Item::Product(self.parse_product_type()?),
            TokenKind::Plus => Item::Sum(self.parse_sum_type()?),
            TokenKind::Equals => Item::Alias(self.parse_alias_type()?),
            TokenKind::Percent => Item::Import(self.parse_import()?),
            _ => return None,
        };
        self.expect_token(TokenKind::Semicolon);

        Some(item)
    }

    pub fn parse_exponent_type(&mut self) -> Option<ExponentType> {
        let name = self.parse_type_name()?;
        let mut parameters = Vec::new();
        while let Some(parameter) = self.parse_variable() {
            parameters.push(parameter);
        }
        let ret_ty = match self.expect_token(TokenKind::Returns) {
            Some(_) => self.parse_type_name(),
            None => None,
        };

        self.expect_token(TokenKind::WithBody)?;
        
        // Skip body to process later
        let mut body_tokens = Vec::new();
        while self.peek_token()?.kind != TokenKind::Semicolon {
            body_tokens.push(self.next_token().unwrap());
        }

        self.exponent_signatures.insert(name.clone(), ExponentSignature{ 
            tokens: body_tokens,
            parameters: parameters.clone(),
        });

        Some(ExponentType {
            name,
            parameters,
            ret_ty,
            body: None,
        })
    }

    pub fn parse_body(&mut self) -> Option<Body> {
        None
    }

    pub fn parse_product_type(&mut self) -> Option<ProductType> {
        let name = self.parse_type_name()?;
        let mut variables = Vec::new();
        while let Some(variable) = self.parse_variable() {
            variables.push(variable);
        }

        Some(ProductType { name, variables })
    }

    pub fn parse_variable(&mut self) -> Option<Variable> {
        let name = self.parse_ident()?;
        let ty = self.parse_type_name()?;

        Some(Variable { name, ty })
    }

    pub fn parse_sum_type(&mut self) -> Option<SumType> {
        let name = self.parse_type_name()?;
        let mut variants = Vec::new();
        while let Some(variant) = self.parse_product_type() {
            variants.push(variant);
        }

        Some(SumType { name, variants })
    }

    pub fn parse_alias_type(&mut self) -> Option<AliasType> {
        let name = self.parse_type_name()?;
        let ty = self.parse_type_name()?;
        Some(AliasType { name, ty })
    }

    pub fn parse_import(&mut self) -> Option<Import> {
        let script = self.parse_ident()?;

        Some(Import { script })
    }

    pub fn parse_ident(&mut self) -> Option<Ident> {
        let ident = self.expect_token(TokenKind::Ident)?;
        Some(Ident {
            ident: ident.s.to_owned(),
        })
    }

    pub fn parse_type_name(&mut self) -> Option<TypeName> {
        let type_name = self.expect_token(TokenKind::Type)?;
        Some(TypeName {
            name: type_name.s.to_owned(),
        })
    }

    pub fn expect_token(&mut self, kind: TokenKind) -> Option<Token> {
        if self.peek_token()?.kind == kind {
            self.next_token()
        } else {
            None
        }
    }

    pub fn peek_token(&mut self) -> Option<&Token> {
        if self.peek.kind == TokenKind::Eoi {
            None
        } else {
            Some(&self.peek)
        }
    }

    pub fn next_token(&mut self) -> Option<Token<'a>> {
        let mut token = self.tokenizer.next()?;
        while match token.kind {
            TokenKind::Comment(_) => true,
            TokenKind::Whitespace => true,
            _ => false,
        } {
            token = self.tokenizer.next()?;
        }

        let cur = std::mem::replace(&mut self.peek, token);
        // self.cur = std::mem::replace(&mut self.peek, token);

        if cur.kind == TokenKind::Eoi {
            None
        } else {
            Some(cur)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Parser;

    #[test]
    fn test_parser() {
        let s = include_str!("scripts/example.dg");
        let mut parser = Parser::new(s);
        let script = parser.parse_script();

        // println!("{:?}", script);
        println!("{:#?}", parser.exponent_signatures);
    }
}
