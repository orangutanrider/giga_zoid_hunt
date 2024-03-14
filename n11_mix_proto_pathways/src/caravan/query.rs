mod internal;

pub use super::*;
use internal::*;

enum SingleQueryStep {
    Get,
    GetMut
}

pub fn query_step(mut caravan: Caravan, entity_input: String) -> Result<Caravan, CaravanError> {
    let token = caravan.next();
    let Some(token) = token else {
        return Ok(caravan);
    };

    match token {
        TokenTree::Group(group) => { // one entity clause to many queries
            // Unpack into nested caravan
            let unpack = &mut caravan.unpack();
            let nested = Caravan::new(group.stream().into_iter(), unpack, caravan.deeper());
            let nested = multi_query_step(nested, entity_input);
            let mut nested = match nested {
                Ok(ok) => ok,
                Err(err) => return Err(err),
            };

            // Repack and continue
            caravan.repack(&nested.unpack());
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

fn multi_query_step(mut caravan: Caravan, entity_input: String) -> Result<Caravan, CaravanError> {
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
            let result = single_query_step(caravan, token, entity_input.clone());
            let result = match result {
                Ok(ok) => ok,
                Err(err) => return Err(err),
            };

            caravan = result;
        },
        TokenTree::Punct(_) => {
            return Err(CaravanError::UnexpectedPunct)
        },
        TokenTree::Literal(_) => {
            return Err(CaravanError::UnexpectedLiteral)
        },
    }

    // Check for comma, continue or end
    let token = caravan.next();
    let Some(token) = token else {
        caravan.escape();
        return Ok(caravan);
    };

    if token.to_string() == "," {
        return multi_query_step(caravan, entity_input);
    }

    return Err(CaravanError::ExpectedComma);
}

fn single_query_step(caravan: Caravan, current: TokenTree, entity_input: String) -> Result<Caravan, CaravanError> {
    // Walk to end of query statement
    let query = collect_query(caravan, current);
    let (caravan, query, bindings) = match query {
        Ok(ok) => ok,
        Err(err) => return Err(err),
    };

    // Get binding decleration
    let result = bindings_step(bindings);
    let (binding, kind) = match result {
        Ok(ok) => ok,
        Err(err) => return Err(err),
    };
    
    // Create query get to bindings statement
    let mut output = "let Ok(".to_owned() + &binding + ") = " + &query;
    match kind {
        SingleQueryStep::Get => {
            output = output + ".get(";
        },
        SingleQueryStep::GetMut => {
            output = output + ".get_mut(";
        },
    }
    output = output + &entity_input + ") else { return; }; \n";

    caravan.output.push_str(&output);
    return query_next(caravan)
}

fn bindings_step(group: Group) -> Result<(String, SingleQueryStep), CaravanError> {
    let mut output = "".to_owned();
    let mut detection = SingleQueryStep::Get;

    let group = group.stream().into_iter();
    for token in group {
        // detect mut
        let token = token.to_string();
        if token == "mut" {
            detection = SingleQueryStep::GetMut;
        }
        output.push_str(&token);
    }

    return Ok((output, detection));   
}

fn query_next(caravan: Caravan) -> Result<Caravan, CaravanError> {
    if caravan.at_surface() {
        return query_surface_next(caravan);
    } else {
        return query_nested_next(caravan);
    }
}