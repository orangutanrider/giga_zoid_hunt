mod entity;
mod query;

use std::str::FromStr;

use proc_macro::*;
use proc_macro::token_stream::IntoIter as TokenIter;

pub use entity::entity_step;

pub struct Caravan<'o> {
    iter: TokenIter,
    output: &'o mut String, // The collected code output, from all recursions
    depth: u32,
}
impl<'o> Caravan<'o> {
    fn new(iter: TokenIter, output: &'o mut String, depth: u32) -> Self {
        return Self {
            iter,
            output,
            depth,
        }
    }

    pub fn start(iter: TokenIter, output: &'o mut String) -> Self {
        return Self {
            iter,
            output,
            depth: 0,
        }
    }
}

impl<'o> Caravan<'o> {
    fn next(&mut self) -> Option<TokenTree> {
        return self.iter.next()
    }

    fn pack(&mut self, string: &str) {
        self.output.push_str(string)
    }

    fn repack(&mut self, string: &str) {
        self.output.clear();
        self.output.push_str(string)
    }

    fn escape(&mut self) {
        self.depth = self.depth - 1;
    }

    pub fn unpack(&mut self) -> String {
        return self.output.to_owned()
    }
}

impl<'o> Caravan<'o> {
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