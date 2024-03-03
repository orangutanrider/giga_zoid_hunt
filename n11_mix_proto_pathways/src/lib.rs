#![feature(proc_macro_span)]
#![feature(iter_next_chunk)]

use proc_macro::*;
use proc_macro::token_stream::IntoIter as TokenIter;

enum PathwayError {
    Undefined,
}

#[proc_macro]
pub fn write_pathway(input: TokenStream) -> TokenStream {
    let mut iter = input.into_iter();
    entity_step(iter);
    return TokenStream::new();
}

fn entity_step(mut iter: TokenIter) -> Result<TokenIter, PathwayError> {
    let token = iter.next();

    // add check for tuple literal

    let Some(token) = token else {
        return Ok(iter); // return empty iter, exit
    };

    match token {
        TokenTree::Group(group) => {
            let group = group.stream().into_iter();
            return multi_entity_step(group);
        },
        TokenTree::Ident(_) => {
            return single_entity_step(iter);
        },
        TokenTree::Punct(_) => {
            return Err(PathwayError::Undefined);
        },
        TokenTree::Literal(_) => {
            return entity_tuple_id_step(iter);
        },
    }
}

// entity::query(,) -> 0.entity::query(,) -> ...;
// without a tuple decleration, it'll either assume 0 or assume that there is no tuple (to be added)
fn entity_tuple_id_step(mut iter: TokenIter) -> Result<TokenIter, PathwayError> {
    let dot = iter.next();
    let Some(dot) = dot else {
        return Err(PathwayError::Undefined)
    };
    if dot.to_string() != "." {
        return Err(PathwayError::Undefined);
    }
    return entity_step(iter);
}

fn single_entity_step(iter: TokenIter) -> Result<TokenIter, PathwayError> {
    return entity_query_punct_step(iter); 
}

fn multi_entity_step(mut group: TokenIter) -> Result<TokenIter, PathwayError> {
    let token = group.next();
    let Some(token) = token else {
        return Ok(group); // exit with empty
    };

    match token {
        TokenTree::Group(_) => {
            return Err(PathwayError::Undefined)
        },
        TokenTree::Ident(_) => {
            let result = entity_query_punct_step(group);
            let Ok(result) = result else {
                return Err(PathwayError::Undefined);
            };
            group = result;
        },
        TokenTree::Punct(_) => {
            return Err(PathwayError::Undefined)
        },
        TokenTree::Literal(_) => {
            return entity_tuple_id_step(group);
        },
    }

    let comma = group.next();
    let Some(comma) = comma else {
        return multi_entity_step(group);
    };

    if comma.to_string() != "," {
        return Err(PathwayError::Undefined)
    }
    return Ok(group)
}

fn entity_query_punct_step(mut iter: TokenIter) -> Result<TokenIter, PathwayError> {
    let puncts = iter.next_chunk::<2>();
    let Ok(puncts) = puncts else {
        return Err(PathwayError::Undefined);
    };

    let span = puncts[0].span().join(puncts[1].span());
    let Some(span) = span else {
        return Err(PathwayError::Undefined);
    };
    let src = span.source_text();
    let Some(src) = src else {
        return Err(PathwayError::Undefined);
    };
    if src != "::" {
        return Err(PathwayError::Undefined);
    }

    //query_step(iter);
    return Ok(iter);
}

/* 
fn query_step(mut iter: token_stream::IntoIter) -> Result<token_stream::IntoIter> {
    let query = iter.next();
    let Some(query) = query else {
        // err_step();
        return;
    };

    match query {
        TokenTree::Punct(_) => {
            return Err;
        },
        TokenTree::Literal(_) => {
            return Err;
        },
        TokenTree::Group(group) => {
            let group = group.stream().into_iter();
            return multi_query_step(group);
        },
        TokenTree::Ident(_) => {
            return single_query_step(iter);
        },
    }
}

fn single_query_step(mut iter: token_stream::IntoIter) -> Result<token_stream::IntoIter> {
    iter = binding_step(iter);
    let Ok(iter) = iter else {
        return Err;
    };

    return Ok(iter);
}

fn multi_query_step(mut iter: token_stream::IntoIter) -> Result<()> {
    let query = iter.next();
    let Some(query) = query else {
        return Err;
    };

    let mut iter = binding_step(iter);
    let Ok(iter) = iter else {
        return Err;
    };

    let comma = iter.next();
    let Some(comma) = comma else {
        multi_query_step(iter);
    };

    return Ok;
}

fn binding_step(mut iter: token_stream::IntoIter) -> Result<token_stream::IntoIter> {
    let group = iter.next();
    let Some(group) = group else {
        return Err(_);
    };
    let TokenTree::Group(group) = group else {
        return Err(_);
    };
    if group.delimiter() != Delimiter::Parenthesis {
        return Err(_);
    }
    
    bindings_step(group);
    next_entity_punct_step(iter);
    return Ok(iter);
}

fn bindings_step(group: Group) {
    // take as string
    // also needs to detect if anything was declared as mutable, so it knows to do get_mut on entity
}

fn next_entity_punct_step(mut iter: token_stream::IntoIter) -> Result<token_stream::IntoIter> {
    let puncts = iter.next_chunk::<2>();
    let Ok(puncts) = puncts else {
        return Ok(iter); // return empty iter, exit
    };

    let span = puncts[0].span().join(puncts[1].span());
    let Some(span) = span else {
        return Err(_);
    };
    let src = span.source_text();
    if src != "->" {
        return Err(_);
    }

    entity_step(iter);
    return Ok(iter);
}

/* 
fn entity_step(mut iter: token_stream::IntoIter) {
    // let tuple_index = iter.next();

    

    //let entity = iter.next();
    //let Some(entity) = entity else {
    //    //exit_step();
    //    return;
    //};
    //
    //let punct1 = iter.next();
    //let Some(punct1) = punct1 else {
    //    //exit_step();
    //    return;
    //};
    //let punct2 = iter.next();
    //let Some(punct2) = punct2 else {
    //    //exit_step();
    //    return;
    //};

    let query = iter.next();
    match
}
*/

// ================================
// Format
// "..." continues, restarting the pattern

// entity::query(,);

// entity::query(,) -> entity::query(,) -> ...;

// entity::query(,) -> {
//     entity::query(,) -> ...,
//     entity::query(,)
// };

// entity::{
//     query(,),
//     query(,) -> ... 
// };

// entity1::query1(,);
// entity1::query2(,) -> {
//    entity2::query3(,) -> ...,
//    entity3::query4(,)
// };

// ================================
// Wildcards
// tuple index (also logic for when there is no tuple index declared, for queries that aren't querying for tuples)
// query mutability (inferred from binding mutability)
// binding mutability, binding reference declerations

// ================================
// Pseudocode

// ENTITY STEP
// END or
// IDENT, expect entity binding
// :: PUNCTUATION
// IDENT query or {GROUP}

// QUERY STEP
// IDENT, expect query
// Enter (GROUP)
// Get comma seperated optional bindings
// Exit (GROUP)
// END or -> 
// IDENT entity or {GROUP}

// END STEP
// If not nested, semi-colon will EXIT
// If nested, comma continues the parent's loop
// If nothing and nested, END to parent

// EXIT STEP
// If nothing, STOP
// Otherwise IDENT, expect entity binding
*/