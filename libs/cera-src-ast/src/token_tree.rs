use cera_parse::{src_token::Token, text_region::TextSpan};

pub struct TokenTree {}

pub enum TokenTreeParseError {}

impl TokenTree {
    pub fn try_from_tokens(
        tokens: &[Token],
        spans: &[TextSpan],
    ) -> Result<Self, TokenTreeParseError> {
        todo!()
    }
}
