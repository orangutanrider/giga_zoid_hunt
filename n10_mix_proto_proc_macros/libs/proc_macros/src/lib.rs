use std::any::Any;
use std::{
    process::Output, 
    str::FromStr
};

use proc_macro::*;
use proc_macro::TokenTree::*;

mod non_macro;
use non_macro::*;

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


//span.join(other) is an unstable feature
#[proc_macro]
pub fn print_resolved_spans(statement: TokenStream) -> TokenStream {
    let mut span_iter = statement.into_iter();

    let full_span = span_iter.next();
    let Some(full_span) = full_span else {
        return TokenStream::new();
    };
    let mut full_span = full_span.span();

    for token in span_iter {
        full_span = full_span.resolved_at(token.span());
    }

    let msg = "resolved_spans: \n".to_owned();
    let full_span = full_span.source_text();
    let Some(full_span) = full_span else {
        return TokenStream::new();
    };
    let msg = msg + &full_span;

    let print = "println!(\"".to_owned() + &msg + "\")";
    let output = TokenStream::from_str(&print);
    let Ok(output) = output else {
        return TokenStream::new();
    };
    return output;
}

#[proc_macro]
pub fn print_located_spans(statement: TokenStream) -> TokenStream {
    let mut span_iter = statement.into_iter();

    let full_span = span_iter.next();
    let Some(full_span) = full_span else {
        return TokenStream::new();
    };
    let mut full_span = full_span.span();

    for token in span_iter {
        full_span = full_span.located_at(token.span());
    }

    let msg = "located_spans: \n".to_owned();
    let full_span = full_span.source_text();
    let Some(full_span) = full_span else {
        return TokenStream::new();
    };
    let msg = msg + &full_span;

    let print = "println!(\"".to_owned() + &msg + "\")";
    let output = TokenStream::from_str(&print);
    let Ok(output) = output else {
        return TokenStream::new();
    };
    return output;
}

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

// Hmm... This would be cool, but I suppose there's a reason that the parameters has to just be a TokenStream
// The macro can't use type information from the code, I can understand that this is a limitation
// Maybe one day it becomes possible? Should it be possible though? Hmm...
// https://forums.swift.org/t/why-arent-macros-given-type-information/66893/5 Hmm...
//#[proc_macro]
//pub fn create_let(value: u32) -> TokenStream {
//    ...
//}

#[proc_macro]
pub fn print_nesting_count(statement: TokenStream) -> TokenStream {
    let iter = statement.into_iter();
    let mut count = 0;
    for token in iter {
        count = count + count_nesting(&token);
    }

    let print = "println!(\"".to_owned() + &count.to_string() + "\")";
    let output = TokenStream::from_str(&print);
    let Ok(output) = output else {
        return TokenStream::new();
    };
    return output;
}

// it could be done, but you'd have to read for the word mut I think
// it isn't data that gets packaged along with an ident or anything like that
//#[proc_macro]
//pub fn print_mutability(statement: TokenStream) -> TokenStream {
//    let iter = statement.into_iter();
//
//    let mut full_msg = "detect mutability: \n".to_owned();
//
//    for token in iter {
//        let s = token.to_string() + "\n";
//        full_msg = plus(full_msg.clone(), &s);
//
//        let Ident(token) = token else {
//            continue;
//        };
//
//        token.
//    }
//
//    let print = "println!(\"".to_owned() + &full_msg + "\")";
//    let output = TokenStream::from_str(&print);
//    let Ok(output) = output else {
//        return TokenStream::new();
//    };
//    return output;
//}

fn count_nesting(token: &TokenTree) -> u32 {
    let mut count = 0;
    match token {
        TokenTree::Group(g) => {
            count = count + 1;
            let iter = g.stream().into_iter();
            for iter_token in iter {
                count = count + count_nesting(&iter_token);
            }
        },
        TokenTree::Ident(_) => {},
        TokenTree::Punct(_) => {},
        TokenTree::Literal(_) => {},
    }

    return count;
}