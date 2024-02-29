use proc_macro::*;

// It'll be like this
// write_pathway(ident, (ident, "binding"), (ident, "bindi...
// the first ident is the first entity being passed into the pathway
// then, the thing in brackets will be repeated any number of times
// the ident in the brackets represents a field that has declared a query
// the "binding" is a literal, that represents the binding that is going to be created by the query

#[proc_macro]
pub fn write_pathway(input: TokenStream) -> TokenStream {
    return TokenStream::new()
}

// entrance_entity::query1::(optional bindings, ...)::entity_binding::query2::...
// entrance_entity::{
//    query1::(optional bindings, ...)::entity_binding::query3::... ,
//    query2::(optional bindings, ...)::entity_binding::query4::... 
// }

// Okay, so it can be fully recursive then can't it?
// restart at every entity I think?, or no?

// Okay, forget groups for now...

// Algorithm
// Get entity
// :: Seperation
// Get query
// :: Seperation
// Enter () group
// Get optional bindings, if there are any
// Exit () group
// if there is a :: Seperation 
// Restart and expect an entity binding

// Hold up though
// Okay yeah, the entity binding is from the query, it shows which field of the query tuple is expected as an entity

// So then expanding to keywords and symbols
// mut, a query can be defined as mut, if so get_mut is used
// mut, an optional binding can be defined as mut, if so it is written as a mutable let
// numbers previous to any binding ident dennote the tuple field being targeted

// Okay, and now, expanding to further group declerations

// 1
// Get entity
// :: Seperation
// Match to either a {group} or a single ident

// 2
// Get query
// :: Seperation
// Enter (group)
// Get optional bindings, if there are any
// Exit (group)
// if there is a :: seperation
    // Go back to 1 and expect an entity binding
// if there is a ,
    // Go back to 2 and expect a query

// Okay...
// I think that's it
// I can barely believe it

// Okay no, no, no...
// It is at the end of optional bindings that the comma check should happen

// 2
// Get query
// :: Seperation
// Enter (group)
// Get optional bindings, if there are any
// Exit (group)
// if there is a ,
    // Go back to 2 and expect a query
// if there is a :: seperation
    // Go back to 1 and expect an entity binding

// Okay yeah no, what I wrote is right
// jesus man...
