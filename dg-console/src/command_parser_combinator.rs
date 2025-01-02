use crate::{Error, ErrorKind};

type ParseResult<'a, T> = Result<(&'a str, T), Error<'a>>;
trait Parser<'a, T> = Fn(&'a str) -> ParseResult<'a, T>;


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
}

fn offset(a: &str, b: &str) -> usize {
    let a = a.as_ptr();
    let b = b.as_ptr();

    b as usize - a as usize
}

fn p_char<'a>(c: char) -> impl Parser<'a, char> {
    move |s: &str| {
        let mut iter = s.chars();
        match iter.next() {
            Some(cc) => {
                if cc == c {
                    Ok((&s[1..], cc))
                } else {
                    Error::parsing(ErrorKind::Char(c))
                }
            }
            None => Err(Error::Eoi),
        }
    }
}

fn p_char_predicate<'a, P>(p: P) -> impl Parser<'a, char>
where
    P: Fn(char) -> bool,
{
    move |s: &str| {
        let mut iter = s.chars();
        match iter.next() {
            Some(c) => {
                if p(c) {
                    Ok((&s[1..], c))
                } else {
                    Error::parsing(
                        ErrorKind::Char(c),
                    )
                }
            }
            None => Err(Error::Eoi),
        }
    }
}

fn p_string<'a>(st: &'a str) -> impl Parser<'a, &'a str> {
    move |s: &str| {
        if s.starts_with(st) {
            Ok((&s[st.len()..], st))
        } else {
            Error::parsing(
                ErrorKind::String(st),
            )
        }
    }
}

fn p_or<'a, T>(a: impl Parser<'a, T>, b: impl Parser<'a, T>) -> impl Parser<'a, T> {
    move |s: &str| {
        a(s).or(b(s)).map_err(|_| Error::Parsing {
            kind: ErrorKind::None,
        })
    }
}

fn p_char_many<'a, P>(p: P) -> impl Parser<'a, &'a str>
where
    P: Fn(char) -> bool,
{
    move |s: &str| {
        let mut iter = s.chars();

        let mut i = 0;
        while let Some(c) = iter.next() {
            if p(c) {
                i += 1;
            } else {
                break;
            }
        }

        Ok((&s[i..], &s[..i]))
    }
}

fn p_char_many_one<'a, P>(p: P) -> impl Parser<'a, &'a str>
where
    P: Fn(char) -> bool,
{
    move |s: &str| {
        let mut iter = s.chars();

        iter.next()
            .and_then(|c| if p(c) { Some(()) } else { None })
            .ok_or(Error::Parsing {
                kind: ErrorKind::Incomplete,
            })?;

        let mut i = 1;
        while let Some(c) = iter.next() {
            if p(c) {
                i += 1;
            } else {
                break;
            }
        }

        Ok((&s[i..], &s[..i]))
    }
}

fn p_many<'a, T, F>(f: F) -> impl Parser<'a, Vec<T>>
where
    F: Parser<'a, T>,
{
    move |mut s: &str| {
        let mut v: Vec<T> = Vec::new();

        while let Ok((cs, r)) = f(s) {
            s = cs;
            v.push(r);
        }

        Ok((s, v))
    }
}

// fn p_many_one<'a, T, F>(f: F) -> impl Parser<'a, Vec<T>>
// where
//     F: Parser<'a, T>,
// {
//     move |s: &str| {
//         let mut v: Vec<T> = Vec::new();
//         let (mut s, r) = f(s)?;
//         v.push(r);

//         while let Ok((cs, r)) = f(s) {
//             s = cs;
//             v.push(r);
//         }

//         Ok((s, v))
//     }
// }

fn p_map<'a, A, B, M>(a: impl Parser<'a, A>, m: M) -> impl Parser<'a, B>
where
    M: Fn(A) -> B,
{
    move |s: &str| a(s).map(|(s, v)| (s, m(v)))
}

fn p_optional<'a, T>(a: impl Parser<'a, T>) -> impl Parser<'a, Option<T>> {
    move |s: &str| match a(s) {
        Ok((i, r)) => Ok((i, Some(r))),
        Err(_) => Ok((s, None)),
    }
}

fn p_verify<'a, V, T>(v: impl Parser<'a, V>, p: impl Parser<'a, T>) -> impl Parser<'a, T> {
    move |s: &str| {
        v(s)?;
        p(s)
    }
}

fn p_combine<'a, A, B>(a: impl Parser<'a, A>, b: impl Parser<'a, B>) -> impl Parser<'a, &'a str> {
    move |s: &str| {
        let i0 = a(s).map(|(r, _)| r)?;
        let i1 = b(i0).map(|(r, _)| r)?;
        let index = offset(s, i1);
        Ok((i1, &s[..index]))
    }
}

fn p_delimited_char<'a, A>(l: char, a: impl Parser<'a, A>, r: char) -> impl Parser<'a, A> {
    move |s: &str| {
        let (s, _) = p_char(l)(s)?;
        let (s, v) = a(s)?;
        let (s, _) = p_char(r)(s)?;

        Ok((s, v))
    }
}

fn p_delimited<'a, L, A, R>(
    l: impl Parser<'a, L>,
    a: impl Parser<'a, A>,
    r: impl Parser<'a, R>,
) -> impl Parser<'a, A> {
    move |s: &str| {
        let (s, _) = l(s)?;
        let (s, v) = a(s)?;
        let (s, _) = r(s)?;

        Ok((s, v))
    }
}

fn parse_whitespace<'a, T>(p: impl Parser<'a, T>) -> impl Parser<'a, T> {
    p_delimited(
        p_char_many(|c| c.is_whitespace()),
        p,
        p_char_many(|c| c.is_whitespace()),
    )
}

fn parse_identifier<'a>() -> impl Parser<'a, &'a str> {
    let valid_start = p_char_predicate(|c: char| match c {
        'a'..='z' => true,
        'A'..='Z' => true,
        _ => false,
    });

    let valid_rest = |c: char| match c {
        'a'..='z' => true,
        'A'..='Z' => true,
        '0'..='9' => true,
        '_' => true,
        _ => false,
    };

    p_verify(valid_start, p_char_many_one(valid_rest))
}

fn parse_integer<'a>() -> impl Parser<'a, i64> {
    p_map(
        p_combine(
            p_optional(p_char('-')),
            p_char_many_one(|c: char| match c {
                '0'..='9' => true,
                _ => false,
            }),
        ),
        |v| str::parse::<i64>(v).unwrap(),
    )
}

fn parse_float<'a>() -> impl Parser<'a, f64> {
    let num = |c: char| match c {
        '0'..='9' => true,
        _ => false,
    };

    p_map(
        p_combine(
            p_optional(p_char('-')),
            p_combine(
                p_char_many_one(num),
                p_combine(p_char('.'), p_char_many_one(num)),
            ),
        ),
        |v| str::parse::<f64>(v).unwrap(),
    )
}

fn parse_boolean<'a>() -> impl Parser<'a, bool> {
    p_map(p_or(p_string("true"), p_string("false")), |v| match v {
        "true" => true,
        "false" => false,
        _ => unreachable!(),
    })
}

fn parse_string<'a>() -> impl Parser<'a, &'a str> {
    p_delimited_char('"', p_char_many(|c| c != '"'), '"')
}

fn parse_value<'a>() -> impl Parser<'a, Value> {
    p_or(
        p_map(parse_float(), |f: f64| Value::Float(f)),
        p_or(
            p_map(parse_integer(), |i: i64| Value::Integer(i)),
            p_or(
                p_map(parse_boolean(), |b: bool| Value::Boolean(b)),
                p_map(parse_string(), |s: &str| Value::String(s.to_owned())),
            ),
        ),
    )
}

pub fn parse_command(s: &str) -> ParseResult<Command> {
    let (s, ident) = parse_whitespace(parse_identifier())(s)?;
    let (s, values) = p_many(parse_whitespace(parse_value()))(s)?;

    Ok((
        s,
        Command {
            name: ident.to_owned(),
            values,
        },
    ))
}

#[cfg(test)]
mod tests {
    use crate::command_parser_combinator::{
        p_string, parse_boolean, parse_float, parse_identifier, parse_integer, parse_string,
        parse_value, ParseResult, Value,
    };

    use super::{p_char, p_or, parse_command, Command};

    #[test]
    fn test_char() {
        let p = p_char('t');

        let s = "testing";
        let r: ParseResult<char> = p(s);
        assert!(r.is_ok());
        assert_eq!(r.as_ref().unwrap().1, 't');
        assert_eq!(r.as_ref().unwrap().0, "esting");
    }

    #[test]
    fn text_str() {
        let p = p_string("test");

        let s = "testing";
        let r: ParseResult<&str> = p(s);
        assert!(r.is_ok());
        assert_eq!(r.as_ref().unwrap().1, "test");
        assert_eq!(r.as_ref().unwrap().0, "ing");
    }

    #[test]
    fn test_either() {
        let p = p_or(p_string("true"), p_string("false"));

        let s = "true";
        let r: ParseResult<&str> = p(s);
        assert!(r.is_ok());
        assert_eq!(r.as_ref().unwrap().1, "true");
        assert_eq!(r.as_ref().unwrap().0, "");

        let s = "false";
        let r: ParseResult<&str> = p(s);
        assert!(r.is_ok());
        assert_eq!(r.as_ref().unwrap().1, "false");
        assert_eq!(r.as_ref().unwrap().0, "");

        let s = "neither";
        let r: ParseResult<&str> = p(s);
        assert!(r.is_err());
    }

    #[test]
    fn test_identifier() {
        let p = parse_identifier();

        let s = "thing";
        let r: ParseResult<&str> = p(s);
        assert!(r.is_ok());
        assert_eq!(r.as_ref().unwrap().1, "thing");
        assert_eq!(r.as_ref().unwrap().0, "");

        let s = "two things";
        let r: ParseResult<&str> = p(s);
        assert!(r.is_ok());
        assert_eq!(r.as_ref().unwrap().1, "two");
        assert_eq!(r.as_ref().unwrap().0, " things");

        let s = "one_thing";
        let r: ParseResult<&str> = p(s);
        assert!(r.is_ok());
        assert_eq!(r.as_ref().unwrap().1, "one_thing");
        assert_eq!(r.as_ref().unwrap().0, "");

        let s = "ok_for_now...";
        let r: ParseResult<&str> = p(s);
        assert!(r.is_ok());
        assert_eq!(r.as_ref().unwrap().1, "ok_for_now");
        assert_eq!(r.as_ref().unwrap().0, "...");

        let s = " <- not thing";
        let r: ParseResult<&str> = p(s);
        assert!(r.is_err());

        let s = "! <- also not thing";
        let r: ParseResult<&str> = p(s);
        assert!(r.is_err());
    }

    #[test]
    fn test_integer() {
        let p = parse_integer();

        let s = "128";
        let r: ParseResult<i64> = p(s);
        assert!(r.is_ok());
        assert_eq!(r.as_ref().unwrap().1, 128);
        assert_eq!(r.as_ref().unwrap().0, "");

        let s = "-9999";
        let r: ParseResult<i64> = p(s);
        assert!(r.is_ok());
        assert_eq!(r.as_ref().unwrap().1, -9999);
        assert_eq!(r.as_ref().unwrap().0, "");

        let s = "integer";
        let r: ParseResult<i64> = p(s);
        assert!(r.is_err());
    }

    #[test]
    fn test_float() {
        let p = parse_float();

        let s = "9000.0";
        let r: ParseResult<f64> = p(s);
        assert!(r.is_ok());
        assert_eq!(r.as_ref().unwrap().1, 9000.0);
        assert_eq!(r.as_ref().unwrap().0, "");

        let s = "-100.001";
        let r: ParseResult<f64> = p(s);
        assert!(r.is_ok());
        assert_eq!(r.as_ref().unwrap().1, -100.001);
        assert_eq!(r.as_ref().unwrap().0, "");

        let s = "points don't float...";
        let r: ParseResult<f64> = p(s);
        assert!(r.is_err());
    }

    #[test]
    fn test_bool() {
        let p = parse_boolean();

        let s = "true";
        let r: ParseResult<bool> = p(s);
        assert!(r.is_ok());
        assert_eq!(r.as_ref().unwrap().1, true);
        assert_eq!(r.as_ref().unwrap().0, "");

        let s = "false";
        let r: ParseResult<bool> = p(s);
        assert!(r.is_ok());
        assert_eq!(r.as_ref().unwrap().1, false);
        assert_eq!(r.as_ref().unwrap().0, "");

        let s = "neither";
        let r: ParseResult<bool> = p(s);
        assert!(r.is_err());
    }

    #[test]
    fn test_string() {
        let p = parse_string();

        let s = "\"Hello, World!\"";
        let r: ParseResult<&str> = p(s);
        assert!(r.is_ok());
        assert_eq!(r.as_ref().unwrap().1, "Hello, World!");
        assert_eq!(r.as_ref().unwrap().0, "");

        let s = "\"But wait there's more!\" and it's over here :)";
        let r: ParseResult<&str> = p(s);
        assert!(r.is_ok());
        assert_eq!(r.as_ref().unwrap().1, "But wait there's more!");
        assert_eq!(r.as_ref().unwrap().0, " and it's over here :)");
    }

    #[test]
    fn test_value() {
        let p = parse_value();

        let s = "true";
        let r: ParseResult<Value> = p(s);
        assert!(r.is_ok());
        assert_eq!(r.as_ref().unwrap().1, Value::Boolean(true));
        assert_eq!(r.as_ref().unwrap().0, "");

        let s = "1";
        let r: ParseResult<Value> = p(s);
        assert!(r.is_ok());
        assert_eq!(r.as_ref().unwrap().1, Value::Integer(1));
        assert_eq!(r.as_ref().unwrap().0, "");

        let s = "1.0";
        let r: ParseResult<Value> = p(s);
        assert!(r.is_ok());
        assert_eq!(r.as_ref().unwrap().1, Value::Float(1.0));
        assert_eq!(r.as_ref().unwrap().0, "");

        let s = "\"Hello, World!\"";
        let r: ParseResult<Value> = p(s);
        assert!(r.is_ok());
        assert_eq!(r.as_ref().unwrap().1, Value::String("Hello, World!".into()));
        assert_eq!(r.as_ref().unwrap().0, "");
    }

    #[test]
    fn test_command() {
        let p = parse_command;

        let s = "function true false 16.666 5 \"frametime\"";

        let r: ParseResult<Command> = p(s);
        let e = Command {
            name: "function".into(),
            values: vec![
                Value::Boolean(true),
                Value::Boolean(false),
                Value::Float(16.666),
                Value::Integer(5),
                Value::String("frametime".into()),
            ],
        };
        assert_eq!(r.as_ref().unwrap().1, e);
    }
}
