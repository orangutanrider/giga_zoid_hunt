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

fn entity_step(mut iter: TokenIter, mut output: String) -> Result<(TokenIter, String), PathwayError> {
    let token = iter.next();
    let Some(token) = token else {
        return Ok((iter, output)); 
    };

    match token {
        TokenTree::Group(group) => {
            let group = group.stream().into_iter();
            let result = multi_entity_step(group, output.clone());
            if let Err(result) = result {
                return Err(result);
            }
            let Ok((iter, pathway_step)) = result else {
                return Err(PathwayError::Undefined)
            };

            output = output + &pathway_step;
            return Ok((iter, output));
        },
        TokenTree::Ident(_) => {
            return single_entity_step(iter, token.span(), EntityKind::ToOther);
        },
        TokenTree::Punct(_) => {
            if token.to_string() == "@" {
                return punct_at_entity_step(iter)
            }
            if token.to_string() == "^" {
                return punct_lift_entity_step(iter)
            }
            return Err(PathwayError::Undefined)
        },
        TokenTree::Literal(_) => {
            return Err(PathwayError::Undefined); 
        },
    }
}

fn punct_at_entity_step(mut iter: TokenIter) -> Result<(TokenIter, String), PathwayError> {
    let token = iter.next();
    let Some(token) = token else {
        return Err(PathwayError::Undefined)
    };
    
    match token {
        TokenTree::Group(_) => {
            return Err(PathwayError::Undefined)
        },
        TokenTree::Ident(_) => {
            return single_entity_step(iter, token.span(), EntityKind::AtSelf);
        },
        TokenTree::Punct(_) => {
            return Err(PathwayError::Undefined)
        },
        TokenTree::Literal(_) => {
            return Err(PathwayError::Undefined)
        },
    }
}

fn punct_lift_entity_step(mut iter: TokenIter) -> Result<(TokenIter, String), PathwayError> {
    let token = iter.next();
    let Some(token) = token else {
        return Err(PathwayError::Undefined)
    };
    
    match token {
        TokenTree::Group(_) => {
            return Err(PathwayError::Undefined)
        },
        TokenTree::Ident(_) => {
            return single_entity_step(iter, token.span(), EntityKind::LiftedToOther);
        },
        TokenTree::Punct(_) => {
            return Err(PathwayError::Undefined)
        },
        TokenTree::Literal(_) => {
            return Err(PathwayError::Undefined)
        },
    }
}

enum EntityKind {
    AtSelf,
    ToOther,
    LiftedToOther
}

fn single_entity_step(iter: TokenIter, current: Span, kind: EntityKind) -> Result<(TokenIter, String), PathwayError> {
    let result = walk_to_end_of_entity_binding(iter, current); 
    if let Err(result) = result {
        return Err(result);
    };
    let Ok((iter, span, additional_puncts)) = result else {
        return Err(PathwayError::Undefined);
    };
    
    let entity_binding = span.source_text();
    let Some(mut entity_binding) = entity_binding else {
        return Err(PathwayError::Undefined)
    };

    match kind {
        EntityKind::AtSelf => { }
        EntityKind::ToOther => {
            entity_binding = entity_binding + ".go()"
        },
        EntityKind::LiftedToOther => {

        }
    }

    // return query_step(iter, entity_binding)
    return Err(PathwayError::Undefined);
}

fn lift_entity_binding(entity_binding: String, is_there: AdditionalPuncts) -> Result<String, PathwayError> {
    match is_there {
        AdditionalPuncts::Found => {
            return Err(PathwayError::Undefined)
        },
        AdditionalPuncts::NoneFound => { },
    }

    // check if it begins with to_
    // if so remove it
    // otherwise add _dest to the end

    return
}

fn multi_entity_step(mut group: TokenIter, mut output: String) -> Result<(TokenIter, String), PathwayError> {
    let token = group.next();
    let Some(token) = token else {
        return Ok((group, output));
    };

    match token {
        TokenTree::Group(_) => {
            return Err(PathwayError::Undefined)
        },
        TokenTree::Ident(_) => {
            let result = single_entity_step(group, token.span());
            if let Err(result) = result {
                return Err(result);
            };
            let Ok((iter, pathway_step)) = result else {
                return Err(PathwayError::Undefined);
            };

            group = iter;
            output = output + &pathway_step;
        },
        TokenTree::Punct(_) => {
            return Err(PathwayError::Undefined)
        },
        TokenTree::Literal(_) => {
            return Err(PathwayError::Undefined); // tuple cannot start with a literal
        },
    }

    let token = group.next();
    let Some(token) = token else {
        return Ok((group, output));
    };

    if token.to_string() == "," {
        return multi_entity_step(group, output);
    }

    return Err(PathwayError::Undefined);
}

fn walk_to_end_of_entity_binding(iter: TokenIter, span: Span,) -> Result<(TokenIter, Span, AdditionalPuncts), PathwayError> {
    return join_until_seperator(iter, span, AdditionalPuncts::NoneFound)
}

fn join_until_seperator(mut iter: TokenIter, span: Span, is_there: AdditionalPuncts) -> Result<(TokenIter, Span, AdditionalPuncts), PathwayError> {
    let token = iter.next();
    let Some(token) = token else {
        return Err(PathwayError::Undefined);
    };
    
    match token {
        TokenTree::Group(_) => {
            return Err(PathwayError::Undefined);
        },
        TokenTree::Punct(_) => {
            return end_at_seperator(token, iter, span, is_there);
        },
        TokenTree::Ident(_) => {
            let span = span.join(token.span());
            let Some(span) = span else {
                return Err(PathwayError::Undefined);
            };

            return join_until_seperator(iter, span, is_there);
        },
        TokenTree::Literal(_) => {
            let span = span.join(token.span());
            let Some(span) = span else {
                return Err(PathwayError::Undefined);
            };

            return join_until_seperator(iter, span, is_there);
        },
    }
}

/// If there are additional puncts, it cannot be lifted, as it's evident of a tuple
enum AdditionalPuncts {
    Found,
    NoneFound,
}

fn end_at_seperator(current: TokenTree, mut iter: TokenIter, span: Span, is_there: AdditionalPuncts) -> Result<(TokenIter, Span, AdditionalPuncts), PathwayError> {
    // If colon expect :: and end
    if current.to_string() == ":" {
        let next = iter.next();
        let Some(next) = next else {
            return Err(PathwayError::Undefined);
        };
        
        let seperator = current.span().join(next.span());
        let Some(seperator) = seperator else {
            return Err(PathwayError::Undefined);
        };
        
        let seperator = seperator.source_text();
        let Some(seperator) = seperator else {
            return Err(PathwayError::Undefined);
        };

        if seperator != "::" {
            return Err(PathwayError::Undefined);
        }
        return Ok((iter, span, is_there));
    }

    // if no colon, continue
    let span = span.join(current.span());
    let Some(span) = span else {
        return Err(PathwayError::Undefined);
    };
    return join_until_seperator(iter, span, AdditionalPuncts::Found);
}

fn single_query_step(iter: TokenIter, query: &str, entity: &str) -> Result<(TokenIter, String), PathwayError> {
    let result = binding_step(iter);
    if let Err(result) = result {
        return Err(result);
    }
    let Ok((iter, binding_decleration, mutability)) = result else {
        return Err(PathwayError::Undefined)
    };

    let mut output = binding_decleration + query + ".get";

    match mutability {
        BindingMutability::IsMutable => {
            output = output + "_mut(";
        },
        BindingMutability::IsNotMutable => {
            output = output + "(";
        },
    }
    output = output + entity + ");";

    return e;
}

enum BindingMutability {
    IsMutable,
    IsNotMutable
}

fn binding_step(mut iter: TokenIter) -> Result<(TokenIter, String, BindingMutability), PathwayError> {
    let group = iter.next();
    let Some(group) = group else {
        return Err(PathwayError::Undefined);
    };
    let TokenTree::Group(group) = group else {
        return Err(PathwayError::Undefined);
    };
    if group.delimiter() != Delimiter::Parenthesis {
        return Err(PathwayError::Undefined);
    }
    
    let (binding, mutability) = bindings_step(group);
    let binding = "let ".to_owned() + &binding + " = ";
    return Ok((iter, binding, mutability));
}

fn bindings_step(group: Group) -> (String, BindingMutability) {
    let mut output = "".to_owned();
    let mut detection = BindingMutability::IsNotMutable;

    let group = group.stream().into_iter();
    for token in group {
        let s = token.to_string();
        if s == "mut" {
            detection = BindingMutability::IsMutable;
        }

        output = output + &s;
    }

    return (output, detection);   
}

/*
fn query_step(mut iter: TokenIter) -> Result<TokenIter, PathwayError> {
    let query = iter.next();
    let Some(query) = query else {
        return Err(PathwayError::Undefined);
    };

    match query {
        TokenTree::Punct(_) => {
            return Err(PathwayError::Undefined);
        },
        TokenTree::Literal(_) => {
            return Err(PathwayError::Undefined);
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

*/
/* 
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