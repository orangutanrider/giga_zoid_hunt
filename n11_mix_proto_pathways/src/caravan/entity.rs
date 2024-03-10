mod internal;

use proc_macro::*;
use proc_macro::token_stream::IntoIter as TokenIter;

pub use super::*;
use internal::*;

/// The different kinds of singular entity steps
enum SingleEntityStep {
    /// DEFAULT
    /// A component pointing to an entity.
    /// The component is used directly, so no entity binding is created.
    Direct, 
    /// ~
    /// A component pointing to an entity.
    /// The component is used to create an entity binding that shadows the component binding.
    Overlap,
    /// ^
    /// A component pointing to an entity.
    /// The component is used to create an entity binding, without shadowing the component binding.
    Lifted,
    /// @
    /// A literal entity.
    Literal, 
}

fn entity_step(mut caravan: Caravan) -> Result<Caravan, CaravanError> {
    let token = caravan.next();
    let Some(token) = token else {
        return Ok(caravan);
    };

    match token {
        TokenTree::Group(group) => {
            let group = Caravan::new(group.stream().into_iter(), caravan.output);
            
            let result = multi_entity_step(group);
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
            // Direct entity step
            return single_entity_step(caravan, token.span(), SingleEntityStep::Direct)
        },
        TokenTree::Punct(_) => {
            return punct_to_entity_wildcard(caravan, token)
        },
        TokenTree::Literal(_) => {
            return Err(CaravanError::Undefined)
        },
    }
}

fn single_entity_step(mut caravan: Caravan, current: Span, kind: SingleEntityStep) -> Result<Caravan, CaravanError> {
    let result = till_entity_clause_fin(caravan, current); 
    if let Err(result) = result {
        return Err(result);
    };
    let Ok((caravan, entity_clause)) = result else {
        return Err(CaravanError::Undefined);
    };
    
    let entity_clause = entity_clause.source_text();
    let Some(mut entity_clause) = entity_clause else {
        return Err(CaravanError::Undefined)
    };

    let mut query_input = "".to_owned();
    let mut entity_let = "".to_owned();

    match kind {
        SingleEntityStep::Literal => {
            query_input = entity_clause;
        },
        SingleEntityStep::Direct => {
            query_input = entity_clause + ".go()";
        },
        SingleEntityStep::Overlap => {
            query_input = entity_clause.clone();
            entity_let = "let ".to_owned() + &entity_clause + " = " + &entity_clause + ".go();" + "\n";
        },
        SingleEntityStep::Lifted => {
            let lift = lift_entity_clause(entity_clause);
            if let Err(lift) = lift {
                return Err(lift)
            }
            let Ok(lift) = lift else {
                return Err(CaravanError::Undefined)
            };
            entity_clause = lift;

            query_input = entity_clause.clone();
            entity_let = "let ".to_owned() + &entity_clause + " = " + &entity_clause + ".go();" + "\n";
        }
    }

    caravan.output.push_str(&entity_let);

    // return query_step(iter, entity_clause)
    return Err(CaravanError::Undefined);
}

/// Caravan iter recieved as group
fn multi_entity_step(mut caravan: Caravan) -> Result<Caravan, CaravanError> {
    let token = caravan.next();
    let Some(token) = token else {
        return Ok(caravan);
    };

    match token {
        TokenTree::Group(_) => {
            return Err(CaravanError::Undefined)
        },
        TokenTree::Ident(_) => {
            let result = single_entity_step(caravan, token.span(), SingleEntityStep::Direct);
            if let Err(result) = result {
                return Err(result);
            };
            let Ok(result) = result else {
                return Err(CaravanError::Undefined);
            };
            caravan = result;
        },
        TokenTree::Punct(_) => {
            let result = punct_to_entity_wildcard(caravan, token);
            if let Err(result) = result {
                return Err(result);
            };
            let Ok(result) = result else {
                return Err(CaravanError::Undefined);
            };
            caravan = result;
        },
        TokenTree::Literal(_) => {
            return Err(CaravanError::Undefined); 
        },
    }

    // Check for comma, continue or end
    let token = caravan.next();
    let Some(token) = token else {
        return Ok(caravan);
    };

    if token.to_string() == "," {
        return multi_entity_step(caravan);
    }

    return Err(CaravanError::Undefined);
}