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

// Once quote becomes stable, replace these with token streams using that.

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
            // Unpack into nested caravan
            let unpack = caravan.unpack();
            let nested = Caravan::new(group.stream().into_iter(), unpack, caravan.deeper());
            let nested = multi_entity_step(nested);
            let mut nested = match nested {
                Ok(ok) => ok,
                Err(err) => return Err(err),
            };

            // Repack and continue
            caravan.repack(nested.unpack());
            return entity_step(caravan)
        },
        TokenTree::Ident(_) => {
            // Direct entity step
            return single_entity_step(caravan, token, SingleEntityStep::Direct)
        },
        TokenTree::Punct(_) => {
            return punct_to_entity_wildcard(caravan, token)
        },
        TokenTree::Literal(_) => {
            return Err(CaravanError::UnexpectedLiteral)
        },
    }
}

fn single_entity_step(caravan: Caravan, current: TokenTree, kind: SingleEntityStep) -> Result<Caravan, CaravanError> {
    let result = collect_entity_clause(caravan, current); 
    let (mut caravan, entity_clause) = match result {
        Ok(ok) => ok,
        Err(err) => return Err(err),
    };

    match kind {
        SingleEntityStep::Literal => {
            return query_step(caravan, entity_clause)
        },
        SingleEntityStep::Direct => {
            let Ok(to_entity) = TokenStream::from_str(TO_ENTITY_FN) else {
                return Err(CaravanError::Undefined)
            };
            entity_clause.extend(to_entity);
            return query_step(caravan, entity_clause);
        },
        SingleEntityStep::Overlap => {
            let Ok(mut let_token) = TokenStream::from_str("let ") else {
                return Err(CaravanError::Undefined)
            };
            let Ok(eq_token) = TokenStream::from_str(" = ") else {
                return Err(CaravanError::Undefined)
            };
            let Ok(to_entity) = TokenStream::from_str(TO_ENTITY_FN) else {
                return Err(CaravanError::Undefined)
            };

            let_token.extend(entity_clause.clone());
            let_token.extend(eq_token);
            let_token.extend(entity_clause.clone());
            let_token.extend(to_entity);

            caravan.pack(let_token);
            return query_step(caravan, entity_clause);
        },
        SingleEntityStep::Lifted => {
            let lift = match lift_entity_clause(entity_clause.clone()) {
                Ok(ok) => ok,
                Err(err) => return Err(err),
            };

            let Ok(mut let_token) = TokenStream::from_str("let ") else {
                return Err(CaravanError::Undefined)
            };
            let Ok(eq_token) = TokenStream::from_str(" = ") else {
                return Err(CaravanError::Undefined)
            };
            let Ok(to_entity) = TokenStream::from_str(TO_ENTITY_FN) else {
                return Err(CaravanError::Undefined)
            };

            let_token.extend(lift);
            let_token.extend(eq_token);
            let_token.extend(entity_clause.clone());
            let_token.extend(to_entity);

            caravan.pack(let_token);
            return query_step(caravan, entity_clause);
        }
    }
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
            let result = single_entity_step(caravan, token, SingleEntityStep::Direct);
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