mod entity;
mod query;

use proc_macro::*;
use proc_macro::token_stream::IntoIter as TokenIter;

pub use entity::entity_step;

pub struct Caravan<'o> {
    iter: TokenIter,
    output: &'o mut String, // The collected code output, from all recursions
    depth: u32,
}
impl<'o> Caravan<'o> {
    fn dig(iter: TokenIter, output: &'o mut String, depth: u32) -> Self {
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

    fn escape(&mut self) {
        self.depth = self.depth - 1;
    }

    pub fn unpack(&self) -> String {
        return self.output.clone()
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

pub enum CaravanError {
    Undefined,
}