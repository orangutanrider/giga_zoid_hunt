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
            let group = Caravan::dig(group.stream().into_iter(), caravan.output, caravan.deeper());
            
            let result = multi_query_step(group, entity_input);
            if let Err(result) = result {
                return Err(result);
            }
            let Ok(mut result) = result else {
                return Err(CaravanError::Undefined)
            };

            result.iter = caravan.iter;
            return Ok(result);
        },
        TokenTree::Ident(_) => {
            return single_query_step(caravan, token, entity_input)
        },
        TokenTree::Punct(_) => {
            return single_query_step(caravan, token, entity_input)
        },
        TokenTree::Literal(_) => {
            return Err(CaravanError::Undefined)
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
            return Err(CaravanError::Undefined)
        },
        TokenTree::Ident(_) => {
            let result = single_query_step(caravan, token, entity_input.clone());
            if let Err(result) = result {
                return Err(result);
            }
            let Ok(result) = result else {
                return Err(CaravanError::Undefined)
            };

            caravan = result;
        },
        TokenTree::Punct(_) => {
            return Err(CaravanError::Undefined)
        },
        TokenTree::Literal(_) => {
            return Err(CaravanError::Undefined)
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

    return Err(CaravanError::Undefined);
}

fn single_query_step(caravan: Caravan, current: TokenTree, entity_input: String) -> Result<Caravan, CaravanError> {
    // Walk to end of query statement
    let query = till_query_fin(caravan, current.span());
    if let Err(query) = query {
        return Err(query)
    }
    let Ok((caravan, query, bindings)) = query else {
        return Err(CaravanError::Undefined)
    };
    let query = query.source_text();
    let Some(query) = query else {
        return Err(CaravanError::Undefined)
    };

    // Get binding decleration
    let result = bindings_step(bindings);
    if let Err(result) = result {
        return Err(result)
    }
    let Ok((binding, kind)) = result else {
        return Err(CaravanError::Undefined)
    };
    let binding = binding.source_text();
    let Some(binding) = binding else {
        return Err(CaravanError::Undefined)
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
    output = output + &entity_input + ") else { return; } \n";

    caravan.output.push_str(&output);
    return query_next(caravan)
}

fn bindings_step(group: Group) -> Result<(Span, SingleQueryStep), CaravanError> {
    let mut output = group.span().end();
    let mut detection = SingleQueryStep::Get;

    let group = group.stream().into_iter();
    for token in group {
        // add to output
        let joined = output.join(token.span());
        let Some(joined) = joined else {
            return Err(CaravanError::Undefined)
        };
        output = joined;

        // detect mut
        let token = token.to_string();
        if token == "mut" {
            detection = SingleQueryStep::GetMut;
        }
    }

    return Ok((output, detection));   
}

fn query_next(caravan: Caravan) -> Result<Caravan, CaravanError> {
    if caravan.at_surface() {
        return query_surface_next(caravan);
    } else {
        return query_deep_next(caravan);
    }
}