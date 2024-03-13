mod internal;

pub use super::*;
use super::query::*;
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

/// For the entity pointers, the macro will write this function, to get the entity they are pointing to.
const TO_ENTITY_FN: &str = ".go()";
/// Will remove from an entity binding decleration (a let statement's name), to generate a new binding (resulting from an entity pointer binding).
const LIFT_REMOVE: &str = "to_"; 
/// If the remove text isn't detected at the start of the entity binding decleration, this will be added to the end of the decleration instead, to generate the new binding.
const LIFT_ADD: &str = "_dest";

// Entity clause wildcards
const LIFT: &str = "^";
const OVERLAP: &str = "~";
const LITERAL: &str = "@";

pub fn entity_step(mut caravan: Caravan) -> Result<Caravan, CaravanError> {
    let token = caravan.next();
    let Some(token) = token else {
        return Ok(caravan);
    };

    match token {
        TokenTree::Group(group) => {
            let group = Caravan::dig(group.stream().into_iter(), caravan.output, caravan.deeper());
            
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
            return Err(CaravanError::UnexpectedLiteral)
        },
    }
}

fn single_entity_step(caravan: Caravan, current: Span, kind: SingleEntityStep) -> Result<Caravan, CaravanError> {
    let result = till_entity_clause_fin(caravan, current); 
    let (caravan, entity_clause) = match result {
        Ok(ok) => ok,
        Err(err) => return Err(err),
    };
    
    let entity_clause = entity_clause.source_text();
    let Some(mut entity_clause) = entity_clause else {
        return Err(CaravanError::SpanToStringError)
    };

    let mut query_input = "".to_owned();
    let mut entity_let = "".to_owned();

    match kind {
        SingleEntityStep::Literal => {
            query_input = entity_clause;
        },
        SingleEntityStep::Direct => {
            query_input = entity_clause + TO_ENTITY_FN;
        },
        SingleEntityStep::Overlap => {
            query_input = query_input + &entity_clause;
            entity_let = "let ".to_owned() + &entity_clause + " = " + &entity_clause + TO_ENTITY_FN + "\n";
        },
        SingleEntityStep::Lifted => {
            let lift = lift_entity_clause(entity_clause);
            let lift = match lift {
                Ok(ok) => ok,
                Err(err) => return Err(err),
            };
            entity_clause = lift;

            query_input = query_input + &entity_clause;
            entity_let = "let ".to_owned() + &entity_clause + " = " + &entity_clause + TO_ENTITY_FN + "\n";
        }
    }

    caravan.output.push_str(&entity_let);

    return query_step(caravan, query_input);
}

/// Caravan iter recieved as group
fn multi_entity_step(mut caravan: Caravan) -> Result<Caravan, CaravanError> {
    let token = caravan.next();
    let Some(token) = token else {
        caravan.escape();
        return Ok(caravan);
    };

    match token {
        TokenTree::Group(_) => {
            return Err(CaravanError::UnexpectedGroup)
        },
        TokenTree::Ident(_) => {
            let result = single_entity_step(caravan, token.span(), SingleEntityStep::Direct);
            let result = match result {
                Ok(ok) => ok,
                Err(err) => return Err(err),
            };
            caravan = result;
        },
        TokenTree::Punct(_) => {
            let result = punct_to_entity_wildcard(caravan, token);
            let result = match result {
                Ok(ok) => ok,
                Err(err) => return Err(err),
            };
            caravan = result;
        },
        TokenTree::Literal(_) => {
            return Err(CaravanError::UnexpectedLiteral); 
        },
    }

    // Check for comma, continue or end
    let token = caravan.next();
    let Some(token) = token else {
        caravan.escape();
        return Ok(caravan);
    };

    if token.to_string() == "," {
        return multi_entity_step(caravan);
    }

    return Err(CaravanError::ExpectedComma);
}