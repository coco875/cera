use std::iter::once;

use num_bigint::BigUint;

use crate::{
    literals::{
        escape_str, parse_float, parse_int, FloatParseError, FloatValue, IntParseError,
        StringEscapeError,
    },
    parsing::{Parsable, PositionnedErr},
    text_region::TextPosition,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Identifier(Identifier),
    Literal(Literal),
    SpecialChar(SpecialChar),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenParseError {
    EmptyStr,
    NoMatch(TextPosition),
    StringEscapeError(StringEscapeError),
    FloatParseError(FloatParseError),
    IntParseError(IntParseError),
    UnexpectedEOF,
    UnexpectedChar(TextPosition),
}

impl PositionnedErr for TokenParseError {
    fn offset(&mut self, offset: usize) {
        match self {
            TokenParseError::NoMatch(pos) => pos.idx += offset,
            TokenParseError::StringEscapeError(err) => err.offset(offset),
            TokenParseError::FloatParseError(err) => err.offset(offset),
            TokenParseError::IntParseError(err) => err.offset(offset),
            TokenParseError::UnexpectedChar(pos) => pos.idx += offset,
            _ => {}
        }
    }
}

impl Parsable for Token {
    type Error = TokenParseError;

    fn try_parse(str: &str) -> Result<(Option<Self>, usize), Self::Error> {
        if str.is_empty() {
            return Err(TokenParseError::EmptyStr);
        }

        if let Some(len) = Self::trim(str) {
            return Ok((None, len));
        }

        if let (Some(ident), len) = Identifier::try_parse(str)? {
            return Ok((Some(Self::Identifier(ident)), len));
        }

        if let (Some(literal), len) = Literal::try_parse(str)? {
            return Ok((Some(Self::Literal(literal)), len));
        }

        if let (Some(special_char), len) = SpecialChar::try_parse(str)? {
            return Ok((Some(Self::SpecialChar(special_char)), len));
        }

        Err(TokenParseError::NoMatch(TextPosition { idx: 0 }))
    }
}

impl Token {
    fn trim(str: &str) -> Option<usize> {
        enum CommentState {
            None,
            LineComment,
            MultiLineComment,
        }

        let mut skipped = 0;
        let mut comment_state = CommentState::None;

        while skipped != str.len() {
            match comment_state {
                CommentState::None => {
                    if str.get(skipped..)?.chars().next()?.is_ascii_whitespace() {
                        skipped += 1;
                        continue;
                    }
                    match str.get(skipped..(skipped + 2)) {
                        Some("//") => comment_state = CommentState::LineComment,
                        Some("/*") => comment_state = CommentState::MultiLineComment,
                        _ => break,
                    }
                    skipped += 2;
                }
                CommentState::LineComment => {
                    let char = str.get(skipped..)?.chars().next()?;
                    if char == '\n' {
                        comment_state = CommentState::None;
                    }
                    skipped += char.len_utf8();
                }
                CommentState::MultiLineComment => {
                    if let Some("*/") = str.get(skipped..(skipped + 2)) {
                        comment_state = CommentState::None;
                        skipped += 2;
                        continue;
                    }
                    let char = str.get(skipped..)?.chars().next()?;
                    skipped += char.len_utf8()
                }
            }
        }

        if skipped != 0 {
            Some(skipped)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identifier {
    name: Box<str>,
}

impl Parsable for Identifier {
    type Error = TokenParseError;

    fn try_parse(str: &str) -> Result<(Option<Self>, usize), Self::Error> {
        let mut iter = str.char_indices().chain(once((str.len(), '\0')));
        let char = iter.next().unwrap().1;
        if !((char.is_ascii_alphabetic()) | (char == '_')) {
            return Ok((None, 0));
        }
        while let Some((idx, char)) = iter.next() {
            if !((char.is_ascii_alphanumeric()) | (char == '_')) {
                return Ok((
                    Some(Identifier {
                        name: str.get(0..idx).unwrap().into(),
                    }),
                    idx,
                ));
            }
        }
        unreachable!()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Literal {
    String(Box<str>),
    Int(BigUint),
    Float(FloatValue),
}

impl Parsable for Literal {
    type Error = TokenParseError;

    fn try_parse(str: &str) -> Result<(Option<Self>, usize), Self::Error> {
        let mut iter = str.char_indices().chain(once((str.len(), '\0')));
        match iter.next().ok_or(TokenParseError::EmptyStr)?.1 {
            '\"' => {
                let mut is_next_escaped = false;

                while let Some((idx, char)) = iter.next() {
                    if is_next_escaped {
                        is_next_escaped = false;
                        continue;
                    }
                    match char {
                        '\\' => is_next_escaped = true,
                        '\"' => match escape_str(str.get(1..idx).unwrap()) {
                            Ok(val) => return Ok((Some(Self::String(val)), idx + 1)),
                            Err(mut err) => {
                                err.offset(1);
                                return Err(TokenParseError::StringEscapeError(err));
                            }
                        },
                        _ => continue,
                    }
                }
                Err(TokenParseError::UnexpectedEOF)
            }
            '0'..='9' => {
                let parse_float_cloj = |idx_float: usize| {
                    return match parse_float(str.get(0..idx_float).unwrap()) {
                        Ok(int) => Ok((Some(Self::Float(int)), idx_float)),
                        Err(err) => Err(TokenParseError::FloatParseError(err)),
                    };
                };
                let parse_int_cloj = |idx: usize| match parse_int(str.get(0..idx).unwrap()) {
                    Ok(int) => Ok((Some(Self::Int(int)), idx)),
                    Err(err) => Err(TokenParseError::IntParseError(err)),
                };
                while let Some((idx, char)) = iter.next() {
                    match char {
                        '0'..='9' | '_' => {
                            continue;
                        }
                        '.' => {
                            let mut is_prev_e = false;
                            let mut is_float = false;
                            while let Some((idx_float, char)) = iter.next() {
                                match char {
                                    'e' => {
                                        is_prev_e = true;
                                        continue;
                                    }
                                    '0'..='9' => {
                                        is_float = true;
                                    }
                                    '_' => {}
                                    '+' | '-' => {
                                        if !is_prev_e {
                                            return parse_float_cloj(idx_float);
                                        }
                                    }
                                    _ => {
                                        if !is_float {
                                            return parse_int_cloj(idx);
                                        }
                                        return parse_float_cloj(idx_float);
                                    }
                                }
                                is_prev_e = false;
                            }
                        }
                        _ => return parse_int_cloj(idx),
                    }
                }
                Err(TokenParseError::UnexpectedEOF)
            }
            _ => Ok((None, 0)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpecialChar {
    Dot,
    SemiColon,
    Colon,
    Comma,
    Plus,
    Minus,
    Equal,
    GreaterThan,
    LessThan,
    Slash,
    Star,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    AtSign,
    Bang,
    QuestionMark,
}

impl Parsable for SpecialChar {
    type Error = TokenParseError;

    fn try_parse(str: &str) -> Result<(Option<Self>, usize), Self::Error> {
        if let Some(elem) = str.chars().next() {
            Ok((
                Some(match elem {
                    '.' => Self::Dot,
                    ';' => Self::SemiColon,
                    ':' => Self::Colon,
                    ',' => Self::Comma,
                    '+' => Self::Plus,
                    '-' => Self::Minus,
                    '=' => Self::Equal,
                    '>' => Self::GreaterThan,
                    '<' => Self::LessThan,
                    '/' => Self::Slash,
                    '*' => Self::Star,
                    '(' => Self::OpenParen,
                    ')' => Self::CloseParen,
                    '{' => Self::OpenBrace,
                    '}' => Self::CloseBrace,
                    '[' => Self::OpenBracket,
                    ']' => Self::CloseBrace,
                    '@' => Self::AtSign,
                    '!' => Self::Bang,
                    '?' => Self::QuestionMark,
                    _ => return Ok((None, 0)),
                }),
                1,
            ))
        } else {
            Err(TokenParseError::UnexpectedEOF)
        }
    }
}
