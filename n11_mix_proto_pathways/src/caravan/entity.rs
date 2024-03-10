mod internal;

use proc_macro::*;
use proc_macro::token_stream::IntoIter as TokenIter;

pub use super::*;

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
                return Err(CaravanError::Undefined)
            };

            output = output + &pathway_step;
            return Ok((iter, output));
        },
        TokenTree::Ident(_) => {
            // Direct entity step
            return single_entity_step(caravan, token.span(), SingleEntityStep::Direct)
        },
        TokenTree::Punct(_) => {
            return punct_to_entity_wildcard(iter, token)
        },
        TokenTree::Literal(_) => {
            return Err(CaravanError::Undefined)
        },
    }
}

fn punct_to_entity_wildcard(mut caravan: Caravan, current: TokenTree) -> Result<Caravan, CaravanError> {
    if current.to_string() == "@" {
        return entity_wildcard_step(iter, SingleEntityStep::Literal)
    }
    
    if current.to_string() == "^" {
        return entity_wildcard_step(iter, SingleEntityStep::Lifted)
    }

    if current.to_string() == "~" {
        return entity_wildcard_step(iter, SingleEntityStep::Overlap)
    }

    return Err(CaravanError::Undefined)
}

fn entity_wildcard_step(mut caravan: Caravan, kind: SingleEntityStep) -> Result<Caravan, CaravanError> {
    let token = iter.next();
    let Some(token) = token else {
        return Err(CaravanError::Undefined)
    };
    
    match token {
        TokenTree::Group(_) => {
            return Err(CaravanError::Undefined)
        },
        TokenTree::Ident(_) => {
            return single_entity_step(iter, token.span(), kind);
        },
        TokenTree::Punct(_) => {
            return Err(CaravanError::Undefined)
        },
        TokenTree::Literal(_) => {
            return Err(CaravanError::Undefined)
        },
    }
}

fn single_entity_step(mut caravan: Caravan, current: Span, kind: SingleEntityStep) -> Result<Caravan, CaravanError> {
    let result = walk_to_end_of_entity_binding(iter, current); 
    if let Err(result) = result {
        return Err(result);
    };
    let Ok((iter, span, additional_puncts)) = result else {
        return Err(CaravanError::Undefined);
    };
    
    let entity_binding = span.source_text();
    let Some(mut entity_binding) = entity_binding else {
        return Err(CaravanError::Undefined)
    };

    let mut query_input = "".to_owned();
    let mut entity_let = "".to_owned();

    match kind {
        SingleEntityStep::Literal => {
            query_input = entity_binding;
        },
        SingleEntityStep::Direct => {
            query_input = entity_binding + ".go()";
        },
        SingleEntityStep::Overlap => {
            query_input = entity_binding.clone();
            entity_let = "let ".to_owned() + &entity_binding + " = " + &entity_binding + ".go();" + "\n";
        },
        SingleEntityStep::Lifted => {
            let lift = lift_entity_binding(entity_binding, additional_puncts);
            if let Err(lift) = lift {
                return Err(lift)
            }
            let Ok(lift) = lift else {
                return Err(CaravanError::Undefined)
            };
            entity_binding = lift;

            query_input = entity_binding.clone();
            entity_let = "let ".to_owned() + &entity_binding + " = " + &entity_binding + ".go();" + "\n";
        }
    }

    // return query_step(iter, entity_binding)
    return Err(CaravanError::Undefined);
}

/// Caravan iter recieved as group
fn multi_entity_step(mut caravan: Caravan) -> Result<Caravan, CaravanError> {
    let token = group.next();
    let Some(token) = token else {
        return Ok((group, output));
    };

    match token {
        TokenTree::Group(_) => {
            return Err(CaravanError::Undefined)
        },
        TokenTree::Ident(_) => {
            let result = single_entity_step(group, token.span(), SingleEntityStep::ToOther);
            if let Err(result) = result {
                return Err(result);
            };
            let Ok((iter, pathway_step)) = result else {
                return Err(CaravanError::Undefined);
            };

            group = iter;
            output = output + &pathway_step;
        },
        TokenTree::Punct(_) => {
            let result = entity_wildcard_step(group, token);
            if let Err(result) = result {
                return Err(result);
            };
            let Ok((iter, pathway_step)) = result else {
                return Err(CaravanError::Undefined);
            };

            group = iter;
            output = output + &pathway_step;
        },
        TokenTree::Literal(_) => {
            return Err(CaravanError::Undefined); 
        },
    }

    // Check for comma, continue or end
    let token = group.next();
    let Some(token) = token else {
        return Ok((group, output));
    };

    if token.to_string() == "," {
        return multi_entity_step(group, output);
    }

    return Err(CaravanError::Undefined);
}