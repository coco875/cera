pub mod literals;
pub mod parsing;
pub mod src_token;
pub mod text_region;

pub use num_bigint;

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use num_bigint::BigUint;

    use crate::{literals::FloatValue, parsing::Parsable, src_token::Literal};

    use super::*;

    #[test]
    fn parsing() {
        fn test_parsable<T: Parsable + Eq + Debug>(str: &str, res: (T, usize))
        where
            T::Error: Debug + Eq,
        {
            assert_eq!(T::try_parse(str), Ok((Some(res.0), res.1)));
        }
        test_parsable::<Literal>("1234", (Literal::Int(BigUint::from(1234usize)), 4));
        test_parsable::<Literal>(
            "123.4",
            (
                Literal::Float(FloatValue {
                    value: 1234usize.into(),
                    exponent: (-1isize).into(),
                }),
                5,
            ),
        );
        test_parsable::<Literal>(
            "12.3e4",
            (
                Literal::Float(FloatValue {
                    value: 123usize.into(),
                    exponent: (3isize).into(),
                }),
                6,
            ),
        );
        test_parsable::<Literal>("\"test\"", (Literal::String("test".into()), 6));
        test_parsable::<Literal>("\"test\\\"\"", (Literal::String("test\"".into()), 8));
    }
}
