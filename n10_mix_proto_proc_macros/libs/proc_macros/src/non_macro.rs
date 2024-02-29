use std::{
    process::Output, 
    str::FromStr
};

use proc_macro::*;

pub fn token_to_matched_str<'a>(token: &TokenTree) -> &'a str {
    match token {
        TokenTree::Group(_) => {
            return "Group"
        },
        TokenTree::Ident(_) => {
            return "Ident"
        },
        TokenTree::Punct(_) => {
            return "Punct"
        },
        TokenTree::Literal(_) => {
            return "Literal"
        },
    }
}

pub fn plus(s1: String, s2: &str) -> String {
    return s1 + s2
}