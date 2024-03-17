mod entity;
mod query;

use std::str::FromStr;

use proc_macro::*;
use proc_macro::token_stream::IntoIter as TokenIter;

pub use entity::entity_step;

pub struct Caravan {
    iter: TokenIter,
    output: TokenStream,
    depth: u32,
}
impl Caravan {
    fn new(iter: TokenIter, output: TokenStream, depth: u32) -> Self {
        return Self {
            iter,
            output,
            depth,
        }
    }

    pub fn start(iter: TokenIter) -> Self {
        return Self {
            iter,
            output: TokenStream::new(),
            depth: 0,
        }
    }
}

impl Caravan {
    fn next(&mut self) -> Option<TokenTree> {
        return self.iter.next()
    }

    fn pack(&mut self, stream: TokenStream) {
        self.output.extend(stream)
    }

    fn repack(&mut self, stream: TokenStream) {
        self.output = stream;
    }

    fn escape(&mut self) {
        self.depth = self.depth - 1;
    }

    pub fn unpack(&mut self) -> TokenStream {
        return self.output.to_owned()
    }
}

impl Caravan {
    fn deeper(&self) -> u32 {
        return self.depth + 1
    }

    fn at_surface(&self) -> bool {
        return self.depth == 0
    }
}

#[derive(Debug)]
pub enum CaravanError {
    Undefined,
    UnexpectedGroup,
    UnexpectedPunct,
    UnexpectedLiteral,
    ExpectedComma,
    ExpectedArrow,
    ExpectedSeperator,
    ExpectedBindings,
    ExpectedEntityClause,
    IncorrectDelimiter,
    SpanToStringError,
    JoinSpansError,
    NoMatchingWildcard,
}

use std::fmt;
impl fmt::Display for CaravanError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl CaravanError {
    pub fn to_stream(&self) -> Result<TokenStream, LexError> {
        return TokenStream::from_str(&self.to_string())
    }
}