use combine::{
    between, choice, error::ParseError, many, optional, parser::char::char as c_char, satisfy, try,
    Parser, Stream,
};
use tokens::{ident_part, Token};

#[derive(Debug, PartialEq, Clone)]
pub struct RegEx {
    pub body: String,
    pub flags: Option<String>,
}

impl RegEx {
    pub fn from_parts(body: &str, flags: Option<String>) -> Self {
        let flags = if let Some(flags) = flags {
            if flags == "" {
                None
            } else {
                Some(flags.to_string())
            }
        } else { 
            None
        };
        RegEx {
            body: body.to_string(),
            flags,
        }
    }
}

/// Parse a regex literal
pub(crate) fn literal<I>() -> impl Parser<Input = I, Output = Token>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        between(c_char('/'), c_char('/'), regex_body()),
        optional(regex_flags()),
    ).map(|(body, flags): (String, Option<String>)| {
        Token::RegEx(RegEx::from_parts(&body, flags))
    })
}

pub(crate) fn regex_tail<I>() -> impl Parser<Input = I, Output = Token>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (
        try(regex_body()),
        c_char('/'),
        optional(try(regex_flags()))
    ).map(|(body, _, flags): (String, _, Option<String>)| Token::RegEx(RegEx::from_parts(&body, flags)))
}
/// Parse the body portion of the regex literal
fn regex_body<I>() -> impl Parser<Input = I, Output = String>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    (regex_first_char(), many(
        regex_char()
    )).map(|(c, s): (String, String)| format!("{}{}", c, s))
}

fn regex_flags<I>() -> impl Parser<Input = I, Output = String>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    many(ident_part()).map(|s: String| s)
}

fn regex_first_char<I>() -> impl Parser<Input = I, Output = String>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        try(regex_body_first_source_char()),
        try(regular_expression_backslash_sequence()),
        try(regular_expression_class()),
    )).map(|c: String| c)
}

fn regex_body_first_source_char<I>() -> impl Parser<Input = I, Output = String>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    satisfy(|c: char| {
        c as u32 <= 4095 && c != '\n' && c != '*' && c != '\\' && c != '/' && c != '['
    })
        .map(|c: char| c.to_string())
}

fn regex_body_source_char<I>() -> impl Parser<Input = I, Output = String>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    satisfy(|c: char| {
        c as u32 <= 4095 && c != '\n' && c != '\\' && c != '/' && c != '['
    })
        .map(|c: char| c.to_string())
}

fn regex_char<I>() -> impl Parser<Input = I, Output = String>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        try(regex_body_source_char()),
        try(regular_expression_backslash_sequence()),
        try(regular_expression_class()),
    )).map(|s: String| s)
}

fn regular_expression_class<I>() -> impl Parser<Input = I, Output = String>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    between(
        c_char('['),
        c_char(']'),
        many(regular_expression_class_char()),
    ).map(|s: String| format!("[{}]", s))
}

fn regular_expression_class_char<I>() -> impl Parser<Input = I, Output = String>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    choice((
        try(
            satisfy(|c: char| c as u32 <= 4095 && c != '\n' && c != '\u{005C}' && c != '\u{005D}')
                .map(|c: char| c.to_string()),
        ),
        try(regular_expression_backslash_sequence()),
    )).map(|s: String| s)
}
pub(crate) fn source_char_not_line_term<I>() -> impl Parser<Input = I, Output = char>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    satisfy(|c: char| is_source_char(c) && !is_line_term(c)).map(|c: char| c)
}

fn regular_expression_backslash_sequence<I>() -> impl Parser<Input = I, Output = String>
where
    I: Stream<Item = char>,
    I::Error: ParseError<I::Item, I::Range, I::Position>,
{
    c_char('\\')
        .and(source_char_not_line_term())
        .map(|(slash, c): (char, char)| format!("{}{}", slash, c))
}

fn is_source_char(c: char) -> bool {
    c as u32 <= 4095
}

fn is_line_term(c: char) -> bool {
    c == '\n' 
    || c == '\r'
    || c == '\u{2028}'
    || c == '\u{2029}'
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn regex_test() {
        let simple = r#"/[a-zA-Z]/"#;
        let s_r = super::literal().parse(simple.clone()).unwrap();
        assert_eq!(s_r, (Token::RegEx(super::RegEx::from_parts(&simple[1..9], None)), ""));
        let flagged = r#"/[0-9]+/g"#;
        let f_r = super::literal().parse(flagged).unwrap();
        assert_eq!(
            f_r,
            (
                Token::RegEx(super::RegEx::from_parts(&flagged[1..7], Some("g".to_string()))),
                ""
            )
        );
        let complex = r#"/^[\s\uFEFF\xA0]+|[\s\uFEFF\xA0]+$/g"#;
        super::literal().parse(complex.clone()).unwrap();
        let escaped = r#"/\D/"#;
        super::literal().parse(escaped).unwrap();
    }

    #[test]
    fn url_regex() {
        let url = r#"/^[a-z][a-z\d.+-]*:\/*(?:[^:@]+(?::[^@]+)?@)?(?:[^\s:/?#]+|\[[a-f\d:]+\])(?::\d+)?(?:\/[^?#]*)?(?:\?[^#]*)?(?:#.*)?$/i"#;
        let _u_r = super::literal().parse(url).unwrap();
    }
}
