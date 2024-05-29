use std::ops::{AddAssign, MulAssign};

use num_bigint::{BigInt, BigUint};

use crate::text_region::{TextPosition, TextSpan};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StringEscapeError {
    UnexpectedStrEnd,
    InvalidEscapedChar(TextSpan),
    /// Escaped unicode values have brackets that must be close after at most 6 characters,
    /// this is returned if that isn't upheld
    ExpectedCloseBracket(TextPosition),
}

impl StringEscapeError {
    pub fn offset(&mut self, offset: usize) {
        match self {
            StringEscapeError::InvalidEscapedChar(span) => span.idx += offset,
            StringEscapeError::ExpectedCloseBracket(position) => position.idx += offset,
            _ => {}
        }
    }
}

pub fn escape_str(str: &str) -> Result<Box<str>, StringEscapeError> {
    let mut output = String::new();
    let mut iter = str.char_indices();
    loop {
        match iter.next() {
            Some((idx, '\\')) => {
                let (next_idx, next_char) = if let Some((idx, char)) = iter.next() {
                    (idx, char)
                } else {
                    return Err(StringEscapeError::UnexpectedStrEnd);
                };

                match next_char {
                    'x' => {
                        if let Some(str) = str.get((idx + 2)..(idx + 4)) {
                            let val = if let Ok(val) = parse_hexadecimal(str) {
                                for _ in 0..2 {
                                    let _ = iter.next();
                                }
                                val
                            } else {
                                return Err(StringEscapeError::InvalidEscapedChar(TextSpan {
                                    len: 4,
                                    idx,
                                }));
                            };
                            if val > 0x7Fu8.into() {
                                return Err(StringEscapeError::InvalidEscapedChar(TextSpan {
                                    len: 4,
                                    idx,
                                }));
                            }
                            output.push(char::from_u32(val.try_into().unwrap()).unwrap())
                        }
                    }
                    'n' => output.push('\n'),
                    'r' => output.push('\r'),
                    't' => output.push('\t'),
                    '\\' => output.push('\\'),
                    '0' => output.push('\0'),
                    'u' => {
                        if let Some(str) = str.get((idx + 2)..(idx + 8)) {
                            let val = if let Ok(val) = parse_hexadecimal(str) {
                                for _ in 0..6 {
                                    let _ = iter.next();
                                }
                                val
                            } else {
                                return Err(StringEscapeError::InvalidEscapedChar(TextSpan {
                                    len: 8,
                                    idx,
                                }));
                            };
                            if let Some(char) = char::from_u32(val.try_into().unwrap()) {
                                output.push(char);
                            } else {
                                return Err(StringEscapeError::InvalidEscapedChar(TextSpan {
                                    len: 8,
                                    idx,
                                }));
                            }
                        }
                    }
                    '\'' => output.push('\''),
                    '\"' => output.push('\"'),
                    _ => {
                        return Err(StringEscapeError::InvalidEscapedChar(TextSpan {
                            len: next_idx - idx,
                            idx,
                        }))
                    }
                }
            }
            Some((_, char)) => output.push(char),
            None => break Ok(output.into_boxed_str()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FloatValue {
    pub value: BigUint,
    pub exponent: BigInt,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FloatParseError {
    NoDecimalDot(TextSpan),
    NumberParseError(IntParseError),
    ExponentTooLarge(TextSpan),
}

impl FloatParseError {
    pub fn offset(&mut self, offset: usize) {
        match self {
            FloatParseError::NoDecimalDot(value) => value.idx += offset,
            FloatParseError::NumberParseError(err) => err.offset(offset),
            FloatParseError::ExponentTooLarge(value) => value.idx += offset,
        }
    }
}

impl From<IntParseError> for FloatParseError {
    fn from(value: IntParseError) -> Self {
        Self::NumberParseError(value)
    }
}

pub fn parse_float(str: &str) -> Result<FloatValue, FloatParseError> {
    let dot_idx = str
        .find('.')
        .ok_or(FloatParseError::NoDecimalDot(TextSpan {
            len: str.len(),
            idx: 0,
        }))?;
    let e_idx = str
        .get((dot_idx + 1)..)
        .ok_or(IntParseError::ZeroLength(TextPosition { idx: dot_idx }))?
        .find(|char| (char == 'e') | (char == 'E'))
        .map(|idx| idx + dot_idx + 1);
    let mut value = parse_decimal(str.get(0..dot_idx).unwrap())?;
    let decimal_str = str
        .get((dot_idx + 1)..(e_idx.unwrap_or(str.len())))
        .unwrap();
    value.mul_assign(BigUint::from(10u8).pow(decimal_str.len() as u32));
    value.add_assign(parse_decimal(decimal_str)?);

    let exponent = -(BigInt::from(
        decimal_str.len()
            - decimal_str
                .chars()
                .fold(0, |acc, char| if char == '_' { acc + 1 } else { acc }),
    )) + BigInt::from(match e_idx {
        Some(idx) => {
            let mut exp_str = str
                .get((idx + 1)..)
                .ok_or(IntParseError::ZeroLength(TextPosition { idx: str.len() }))?;
            let mut exp = match exp_str
                .chars()
                .next()
                .ok_or(IntParseError::ZeroLength(TextPosition { idx: str.len() }))?
            {
                '+' => {
                    exp_str = exp_str
                        .get(1..)
                        .ok_or(IntParseError::ZeroLength(TextPosition { idx: str.len() }))?;
                    1
                }
                '-' => {
                    exp_str = exp_str
                        .get(1..)
                        .ok_or(IntParseError::ZeroLength(TextPosition { idx: str.len() }))?;
                    -1
                }
                _ => 1,
            };
            exp.mul_assign(isize::try_from(parse_decimal(exp_str)?).map_err(|_| {
                FloatParseError::ExponentTooLarge(TextSpan {
                    len: exp_str.len(),
                    idx: idx + 1,
                })
            })?);

            exp
        }
        None => 0,
    });

    Ok(FloatValue { value, exponent })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IntParseError {
    InvalidChar(TextPosition),
    ZeroLength(TextPosition),
}

impl IntParseError {
    pub fn offset(&mut self, offset: usize) {
        match self {
            IntParseError::InvalidChar(pos) => pos.idx += offset,
            IntParseError::ZeroLength(pos) => pos.idx += offset,
        }
    }
}

pub fn parse_int(str: &str) -> Result<BigUint, IntParseError> {
    if let Some("0") = str.get(0..1) {
        let str_res = if let Some(s) = str.get(2..) {
            Ok(s)
        } else {
            Err(IntParseError::ZeroLength(TextPosition { idx: 2 }))
        };
        match str.get(1..2) {
            Some("b") => {
                return parse_binary(str_res?).map_err(|mut err| {
                    err.offset(2);
                    err
                })
            }
            Some("o") => {
                return parse_octal(str_res?).map_err(|mut err| {
                    err.offset(2);
                    err
                })
            }
            Some("d") => {
                return parse_decimal(str_res?).map_err(|mut err| {
                    err.offset(2);
                    err
                })
            }
            Some("x") => {
                return parse_hexadecimal(str_res?).map_err(|mut err| {
                    err.offset(2);
                    err
                })
            }
            _ => {}
        }
    }
    parse_decimal(str)
}

pub fn parse_binary(str: &str) -> Result<BigUint, IntParseError> {
    if str.is_empty() {
        return Err(IntParseError::ZeroLength(TextPosition { idx: 0 }));
    }
    if str.chars().next() == Some('_') {
        return Err(IntParseError::InvalidChar(TextPosition { idx: 0 }));
    }
    if str.chars().next_back() == Some('_') {
        return Err(IntParseError::InvalidChar(TextPosition {
            idx: str.len() - 1,
        }));
    }
    let mut output = BigUint::ZERO;
    for (idx, char) in str.char_indices() {
        if char == '_' {
            continue;
        }
        output.mul_assign(2usize);
        match char {
            '0' => {}
            '1' => output.add_assign(1usize),
            _ => return Err(IntParseError::InvalidChar(TextPosition { idx })),
        }
    }
    return Ok(output);
}

pub fn parse_octal(str: &str) -> Result<BigUint, IntParseError> {
    if str.is_empty() {
        return Err(IntParseError::ZeroLength(TextPosition { idx: 0 }));
    }
    if str.chars().next() == Some('_') {
        return Err(IntParseError::InvalidChar(TextPosition { idx: 0 }));
    }
    if str.chars().next_back() == Some('_') {
        return Err(IntParseError::InvalidChar(TextPosition {
            idx: str.len() - 1,
        }));
    }
    let mut output = BigUint::ZERO;
    for (idx, char) in str.char_indices() {
        if char == '_' {
            continue;
        }
        output.mul_assign(8usize);
        match char {
            '0' => {}
            '1' => output.add_assign(1usize),
            '2' => output.add_assign(2usize),
            '3' => output.add_assign(3usize),
            '4' => output.add_assign(4usize),
            '5' => output.add_assign(5usize),
            '6' => output.add_assign(6usize),
            '7' => output.add_assign(7usize),
            _ => return Err(IntParseError::InvalidChar(TextPosition { idx })),
        }
    }
    return Ok(output);
}

pub fn parse_decimal(str: &str) -> Result<BigUint, IntParseError> {
    if str.is_empty() {
        return Err(IntParseError::ZeroLength(TextPosition { idx: 0 }));
    }
    if str.chars().next() == Some('_') {
        return Err(IntParseError::InvalidChar(TextPosition { idx: 0 }));
    }
    if str.chars().next_back() == Some('_') {
        return Err(IntParseError::InvalidChar(TextPosition {
            idx: str.len() - 1,
        }));
    }
    let mut output = BigUint::ZERO;
    for (idx, char) in str.char_indices() {
        if char == '_' {
            continue;
        }
        output.mul_assign(10usize);
        match char {
            '0' => {}
            '1' => output.add_assign(1usize),
            '2' => output.add_assign(2usize),
            '3' => output.add_assign(3usize),
            '4' => output.add_assign(4usize),
            '5' => output.add_assign(5usize),
            '6' => output.add_assign(6usize),
            '7' => output.add_assign(7usize),
            '8' => output.add_assign(8usize),
            '9' => output.add_assign(9usize),
            _ => return Err(IntParseError::InvalidChar(TextPosition { idx })),
        }
    }
    return Ok(output);
}

pub fn parse_hexadecimal(str: &str) -> Result<BigUint, IntParseError> {
    if str.is_empty() {
        return Err(IntParseError::ZeroLength(TextPosition { idx: 0 }));
    }
    if str.chars().next() == Some('_') {
        return Err(IntParseError::InvalidChar(TextPosition { idx: 0 }));
    }
    if str.chars().next_back() == Some('_') {
        return Err(IntParseError::InvalidChar(TextPosition {
            idx: str.len() - 1,
        }));
    }
    let mut output = BigUint::ZERO;
    for (idx, char) in str.char_indices() {
        if char == '_' {
            continue;
        }
        output.mul_assign(16usize);
        match char {
            '0' => {}
            '1' => output.add_assign(1usize),
            '2' => output.add_assign(2usize),
            '3' => output.add_assign(3usize),
            '4' => output.add_assign(4usize),
            '5' => output.add_assign(5usize),
            '6' => output.add_assign(6usize),
            '7' => output.add_assign(7usize),
            '8' => output.add_assign(8usize),
            '9' => output.add_assign(9usize),
            'a' | 'A' => output.add_assign(10usize),
            'b' | 'B' => output.add_assign(11usize),
            'c' | 'C' => output.add_assign(12usize),
            'd' | 'D' => output.add_assign(13usize),
            'e' | 'E' => output.add_assign(14usize),
            'f' | 'F' => output.add_assign(15usize),
            _ => return Err(IntParseError::InvalidChar(TextPosition { idx })),
        }
    }
    return Ok(output);
}
