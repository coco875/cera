use cera_parse::{
    parsing::{parse, ParsingError},
    src_token::{Token, TokenParseError},
};

fn main() {
    let text = std::fs::read_to_string("./sample-code/main.cera").unwrap();
    let res = parse::<Token>(text.as_str());
    if let Err(ParsingError::ParsableError(TokenParseError::NoMatch(res))) = &res {
        dbg!(&text.as_str()[res.idx..]);
    }
    println!("{:?}", res);
}
