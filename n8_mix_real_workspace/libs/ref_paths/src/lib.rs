use proc_macro::*;

#[proc_macro]
pub fn write_pathway(input: TokenStream) -> TokenStream {
    return TokenStream::new()
}

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
// 1
// Get entity 
// :: 
// If {group}, move to 2, otherwise continue
// Get query
// Enter (group)
// Get optional bindings, if there are any
// Exit (group)
// If ; End
// If -> {group}, move to 3
// If -> Restart

// 2
// Get query
// Enter (group)
// Get optional bindings, if there are any
// Exit (group)
// If ,
    // Restart 2, expect query 
// If -> 
    // Go to 3

// 3
// Get entity 
// :: 
// If {group}, move to 2, otherwise continue
// Get query
// Enter (group)
// Get optional bindings, if there are any
// Exit (group)
// If ,
    // Restart 3, expect entity 
// If -> 
    // Restart 3, expect entity