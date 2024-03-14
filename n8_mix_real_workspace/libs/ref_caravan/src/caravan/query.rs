mod internal;

pub use super::*;
use internal::*;

enum SingleQueryStep {
    Get,
    GetMut
}

pub fn query_step(mut caravan: Caravan, entity_input: TokenStream) -> Result<Caravan, CaravanError> {
    let token = caravan.next();
    let Some(token) = token else {
        return Ok(caravan);
    };

    match token {
        TokenTree::Group(group) => { // one entity clause to many queries
            // Unpack into nested caravan
            let unpack = caravan.unpack();
            let nested = Caravan::new(group.stream().into_iter(), unpack, caravan.deeper());
            let nested = multi_query_step(nested, entity_input);
            let mut nested = match nested {
                Ok(ok) => ok,
                Err(err) => return Err(err),
            };

            // Repack and continue
            caravan.repack(nested.unpack());
            return query_next(caravan)
        },
        TokenTree::Ident(_) => {
            return single_query_step(caravan, token, entity_input)
        },
        TokenTree::Punct(_) => {
            return single_query_step(caravan, token, entity_input)
        },
        TokenTree::Literal(_) => {
            return Err(CaravanError::UnexpectedLiteral)
        },
    }
}

fn multi_query_step(mut caravan: Caravan, entity_input: TokenStream) -> Result<Caravan, CaravanError> {
    let token = caravan.next();
    let Some(token) = token else {
        caravan.escape();
        return Ok(caravan);
    };

    match token {
        TokenTree::Group(_) => { // one entity clause to many queries
            return Err(CaravanError::UnexpectedGroup)
        },
        TokenTree::Ident(_) => {
            let caravan = single_query_step(caravan, token, entity_input.clone());
            let caravan = match caravan {
                Ok(ok) => ok,
                Err(err) => return Err(err),
            };

            return multi_query_step(caravan, entity_input)
        },
        TokenTree::Punct(_) => {
            return Err(CaravanError::UnexpectedPunct)
        },
        TokenTree::Literal(_) => {
            return Err(CaravanError::UnexpectedLiteral)
        },
    }
}

fn single_query_step(caravan: Caravan, current: TokenTree, entity_input: TokenStream) -> Result<Caravan, CaravanError> {
    // Walk to end of query statement
    let query = collect_query(caravan, current);
    let (mut caravan, query, bindings) = match query {
        Ok(ok) => ok,
        Err(err) => return Err(err),
    };

    // Let
    let Ok(mut let_token) = TokenStream::from_str("let Ok") else {
        return Err(CaravanError::Undefined)
    };
    // Binding
    let result = bindings_step(bindings);
    let (bindings, kind) = match result {
        Ok(ok) => ok,
        Err(err) => return Err(err),
    };
    let bindings = TokenStream::from_str(&bindings.to_string());
    // Equal
    let Ok(eq_token) = TokenStream::from_str(" = ") else {
        return Err(CaravanError::Undefined)
    };
    // Query
    // Get
    let get = match kind {
        SingleQueryStep::Get => {
            TokenStream::from_str(".get")
        },
        SingleQueryStep::GetMut => {
            TokenStream::from_str(".get_mut")
        },
    };
    let Ok(get) = get else {
        return Err(CaravanError::Undefined)
    };
    // Entity
    let entity_input = Group::new(Delimiter::Parenthesis, entity_input);
    let entity_input = TokenStream::from_str(&entity_input.to_string());
    // Else return
    let Ok(end) = TokenStream::from_str(" else { return; }; \n ") else {
        return Err(CaravanError::Undefined)
    };

    let_token.extend(bindings);
    let_token.extend(eq_token);
    let_token.extend(query);
    let_token.extend(get);
    let_token.extend(entity_input);
    let_token.extend(end);

    caravan.pack(let_token);
    return query_next(caravan)
}

fn bindings_step(group: Group) -> Result<(Group, SingleQueryStep), CaravanError> {
    let mut detection = SingleQueryStep::Get;

    let iter = group.clone().stream().into_iter();
    for token in iter {
        // detect mut
        let token = token.to_string();
        if token == "mut" {
            detection = SingleQueryStep::GetMut;
        }
    }

    return Ok((group, detection));   
}

fn query_next(caravan: Caravan) -> Result<Caravan, CaravanError> {
    if caravan.at_surface() {
        return query_surface_next(caravan);
    } else {
        return query_nested_next(caravan);
    }
}