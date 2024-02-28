use proc_macro::*;

// It'll be like this
// write_pathway(ident, (ident, "binding"), (ident, "bindi...
// the first ident is the first entity being passed into the pathway
// then, the thing in brackets will be repeated any number of times
// the ident in the brackets represents a field that has declared a query
// the "binding" is a literal, that represents the binding that is going to be created by the query

#[proc_macro]
pub fn write_pathway(input: TokenStream) -> TokenStream {
    let return_v = TokenStream::new();
    let mut stream = input.into_iter();

    // Expect Ident of an entity field for first token
    let pathway_entrance = stream.next();
    let Some(pathway_entrance) = pathway_entrance else {
        // ERROR
        println!("1"); // As a macro? When and where does this get printed?
        return return_v;
    };
    let Ident(pathway_entrance) = pathway_entrance else {
        // ERROR
        println!("2");
        return return_v;
    };

    // Proto
    // This'll need to be done for every element, and will also need to handle the case where there is no group, just an ident
    // But, to help myself, I'm doing it as one element for now; Coding it all at once, although I know what I want, I don't have the expertise to do that.

    // get and expect as group (even though it's valid as a single ident)
    let group = stream.next();
    let Some(group) = group else {
        println!("3");
        return return_v;
    };
    let Group(group) = group else {
        println!("4");
        return return_v;
    };

    // token stream from group
    let group = group.stream;
    let Some(group) = group else {
        println!("5"); 
        return return_v;
    };

    let mut group_stream = group.into_iter();
    let query = group_stream.next();

    let Some(query) = query else {
        println!("6");
        return return_v;
    };
    let Ident(query) = query else {
        println!("7");
        return return_v;
    };

    let comma = group_stream.next();

    let binding = group_stream.next();
    let Some(binding) = binding else {
        println!("6");
        return return_v;
    };
    let Literal(query) = query else {
        println!("7");
        return return_v;
    };

    return return_v
}