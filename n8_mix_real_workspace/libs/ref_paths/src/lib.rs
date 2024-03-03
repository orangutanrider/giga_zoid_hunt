use proc_macro::*;

//#[proc_macro]
//pub fn write_pathway(input: TokenStream) -> TokenStream {
//    return TokenStream::new()
//}

fn entity_step(entity: Option<TokenTree>) {
    let Some(entity) = entity else {
        // exit_step();
        return;
    };
    if TokenTree::Ident(entity) != entity {
        // err_step();
        return;
    }
}

fn entity_query_punct_step(punct: Option<TokenTree>) {
    let Some(punct) = punct else {
        // err_step();
        return;
    };
    if TokenTree::Punct(punct) != punct {
        // err_step();
        return;
    }

    // add check for if is :
}

fn query_step(query: Option<TokenTree>) {
    let Some(query) = query else {
        // err_step();
        return;
    };
    if TokenTree::Ident(query) != query {
        // err_step();
        return;
    }
}

fn binding_group_step(group: Option<TokenTree>) {
    let Some(group) = group else {
        // err_step();
        return;
    };
    if TokenTree::Group(group) != group {
        // err_step();
        return;
    }

    // add check for if is ()
}

fn optional_binding_step(mut group: token_stream::IntoIter) {
    // add check for mutability decleration
    // add check for reference symbol decleration

    let binding = group.next();
    let Some(binding) = binding else {
        // exit
        return;
    };
    if TokenTree::Ident(binding) != binding {
        // error
        return;
    }

    let comma = group.next();
    let Some(comma) = comma else {
        // exit
        return;  
    };
    if TokenTree::Punct(comma) != comma {
        // error
        return;
    }

    // add check for if is ,

    optional_binding_step(group);
}

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