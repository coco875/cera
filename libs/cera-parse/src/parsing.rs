use crate::text_region::{Text, TextPosition, TextSpan};

pub trait PositionnedErr {
    fn offset(&mut self, offset: usize);
}

pub trait Parsable: Sized {
    type Error: PositionnedErr;
    /// On success, returns the amount of bytes consumed in the given str
    /// Returns Ok(None) if the result should be discarded (IE for white space)
    fn try_parse(str: &str) -> Result<(Option<Self>, usize), Self::Error>;
}

#[derive(Debug, Clone)]
pub enum ParsingError<T> {
    ParsableError(T),
    OutOfCharBoundError(TextPosition),
    /// This indicates that the Parsable implementation asked to skip 0 bytes
    /// This errors because it would otherwise cause an infinite loop
    ZeroLenSkip(TextPosition),
}

pub fn parse<T: Parsable>(text: &str) -> Result<(Vec<T>, Vec<TextSpan>), ParsingError<T::Error>> {
    let mut output = (Vec::new(), Vec::new());
    let mut curr_start = 0;
    let mut curr_str = text;
    while !curr_str.is_empty() {
        let (res, skipped) = match T::try_parse(curr_str) {
            Ok(res) => res,
            Err(mut err) => {
                err.offset(curr_start);
                return Err(ParsingError::ParsableError(err));
            }
        };
        if skipped == 0 {
            return Err(ParsingError::ZeroLenSkip(TextPosition { idx: curr_start }));
        }
        if let Some(res) = res {
            output.0.push(res);
            output.1.push(TextSpan {
                len: skipped,
                idx: curr_start,
            })
        }
        if let Some(new_str) = curr_str.get(skipped..) {
            curr_str = new_str;
            curr_start += skipped;
        } else {
            return Err(ParsingError::OutOfCharBoundError(TextPosition {
                idx: curr_start,
            }));
        }
    }
    Ok(output)
}
