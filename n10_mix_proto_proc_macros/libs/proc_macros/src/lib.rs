#![feature(proc_macro_quote)]

use std::{
    process::Output, 
    str::FromStr
};

use proc_macro::*;

//https://github.com/dtolnay/proc-macro-workshop?tab=readme-ov-file#function-like-macro-seq
//https://www.youtube.com/watch?v=RfhkCdu3iYs

//#[proc_macro]
//pub fn print_as_string(input: TokenStream) -> TokenStream {
//    let string = input.to_string();
//    println!("{}", string);
//
//    return input;
//}
//
//#[proc_macro]
//pub fn double(statement: TokenStream) -> TokenStream {
//    let s = statement.to_string();
//    let s = s.clone() + &s;
//    let return_v = TokenStream::from_str(&s);
//    let Ok(return_v) = return_v else {
//        return TokenStream::new();
//    };
//    return return_v
//}
//
//#[proc_macro]
//pub fn turn_into_print(statement: TokenStream) -> TokenStream {
//    let s = statement.to_string();
//    let s = "println!(".to_owned() + "\"" + &s + "\"" + ")";
//    let return_v = TokenStream::from_str(&s);
//    let Ok(return_v) = return_v else {
//        return TokenStream::new();
//    };
//    return return_v
//}

#[proc_macro]
pub fn print_spans(statement: TokenStream) -> TokenStream {
    let iter = statement.into_iter();

    let mut full_msg = "spans: \n".to_owned();

    for token in iter {
        let s = token.span().source_text();
        let Some(s) = s else { continue; };
        let s = s + "\n";
        full_msg = plus(full_msg.clone(), &s);
    }

    let print = "println!(\"".to_owned() + &full_msg + "\")";
    let output = TokenStream::from_str(&print);
    let Ok(output) = output else {
        return TokenStream::new();
    };
    return output;
}

#[proc_macro]
pub fn print_tokens(statement: TokenStream) -> TokenStream {
    let iter = statement.into_iter();

    let mut full_msg = "tokens: \n".to_owned();

    for token in iter {
        let s = token.to_string() + "\n";
        full_msg = plus(full_msg.clone(), &s);
    }

    let print = "println!(\"".to_owned() + &full_msg + "\")";
    let output = TokenStream::from_str(&print);
    let Ok(output) = output else {
        return TokenStream::new();
    };
    return output;
}

#[proc_macro]
pub fn print_kind(statement: TokenStream) -> TokenStream {
    let iter = statement.into_iter();

    let mut full_msg = "token kinds: \n".to_owned();

    for token in iter {
        let s = "".to_owned() + token_to_matched_str(&token) + "\n";
        full_msg = plus(full_msg.clone(), &s);
    }

    let print = "println!(\"".to_owned() + &full_msg + "\")";
    let output = TokenStream::from_str(&print);
    let Ok(output) = output else {
        return TokenStream::new();
    };
    return output;
}

fn token_to_matched_str<'a>(token: &TokenTree) -> &'a str {
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

fn plus(s1: String, s2: &str) -> String {
    return s1 + s2
}