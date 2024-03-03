#![feature(proc_macro_span)]

use proc_macro::*;

//#[proc_macro]
//pub fn write_pathway(input: TokenStream) -> TokenStream {
//    return TokenStream::new()
//}

fn entity_step(mut iter: token_stream::IntoIter) {
    let entity = iter.next();

    // add check for tuple literal

    let Some(entity) = entity else {
        // exit_step();
        return;
    };
    if TokenTree::Ident(entity) != entity {
        // err_step();
        return;
    }

    entity_query_punct_step(iter)
}

fn entity_query_punct_step(mut iter: token_stream::IntoIter) {
    let puncts = iter.next_chunk::<2>();
    let Ok(puncts) = puncts else {
        // error
        return;
    };

    let span = puncts[0].span().join(puncts[1].span());
    let Some(span) = span else {
        // error
        return;
    };
    let src = span.source_text();
    if src != "::" {
        // error
        return;
    }

    query_step(iter);
}

fn query_step(mut iter: token_stream::IntoIter) {
    let query = iter.next();
    let Some(query) = query else {
        // err_step();
        return;
    };

    match query {
        TokenTree::Punct(_) => {
            // error
            return;
        },
        TokenTree::Literal(_) => {
            // error
            return;
        },
        TokenTree::Group(group) => {
            let group = group.stream().into_iter();
            multi_query_step(group);
        },
        TokenTree::Ident(_) => {

        },
    }
}

fn multi_query_step(mut iter: token_stream::IntoIter) {
    let query = iter.next();
    let Some(query) = query else {
        // err_step();
        return;
    };

    binding_step(iter);
}

fn binding_step(mut iter: token_stream::IntoIter) {
    let local_iter = iter.clone();

    let punct = iter.next();
    let Some(punct) = punct else {
        // error
        return;
    };



}

fn bindings_step(group: Group) {

}

fn multi_bindings_step()

fn next_entity_punct_step1(punct: Option<TokenTree>) {
    let Some(punct) = punct else {
        // exit
        return;
    };
    if TokenTree::Punct(punct) != punct {
        // error
        return;
    }

    // add check for if is -
}

fn next_entity_punct_step2(punct: Option<TokenTree>) {
    let Some(punct) = punct else {
        // errpr
        return;
    };
    if TokenTree::Punct(punct) != punct {
        // error
        return;
    }

    // add check for if is >
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

// entity::query<>;

// entity::query<> -> entity::query<> -> ...;

// entity::query<> -> {
//     entity::query<> -> ...,
//     entity::query<>
// };

// entity::{
//     query<>,
//     query<> -> ... 
// };

// entity1::query1<>;
// entity1::query2<> -> {
//    entity2::query3<> -> ...,
//    entity3::query4<>
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