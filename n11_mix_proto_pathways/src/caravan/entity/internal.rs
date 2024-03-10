use proc_macro::*;
use proc_macro::token_stream::IntoIter as TokenIter;

use super::*;

fn lift_entity_clause(mut entity_clause: String) -> Result<String, CaravanError> {
    // if format is "to_entity", removes the "to_"
    let to = &entity_clause[..3];
    if to == "to_" {
        entity_clause.replace_range(..3, "");
        return Ok(entity_clause)
    }

    // otherwise adds "_dest" to the end
    entity_clause = entity_clause + "_dest";
    return Ok(entity_clause);
}

fn walk_to_entity_clause_end(mut caravan: Caravan, span: Span,) -> Result<(Caravan, Span), CaravanError> {
    return join_until_seperator(iter, span)
}

fn join_until_seperator(mut caravan: Caravan, span: Span) -> Result<(Caravan, Span), CaravanError> {
    let token = iter.next();
    let Some(token) = token else {
        return Err(CaravanError::Undefined);
    };
    
    match token {
        TokenTree::Group(_) => {
            return Err(CaravanError::Undefined);
        },
        TokenTree::Punct(_) => {
            return end_at_seperator(token, iter, span);
        },
        TokenTree::Ident(_) => {
            let span = span.join(token.span());
            let Some(span) = span else {
                return Err(CaravanError::Undefined);
            };

            return join_until_seperator(iter, span);
        },
        TokenTree::Literal(_) => {
            let span = span.join(token.span());
            let Some(span) = span else {
                return Err(CaravanError::Undefined);
            };

            return join_until_seperator(iter, span);
        },
    }
}

fn end_at_seperator(mut caravan: Caravan, current: TokenTree, span: Span) -> Result<(Caravan, Span), CaravanError> {
    // If colon expect :: and end
    if current.to_string() == ":" {
        let next = iter.next();
        let Some(next) = next else {
            return Err(CaravanError::Undefined);
        };
        
        let seperator = current.span().join(next.span());
        let Some(seperator) = seperator else {
            return Err(CaravanError::Undefined);
        };
        
        let seperator = seperator.source_text();
        let Some(seperator) = seperator else {
            return Err(CaravanError::Undefined);
        };

        if seperator != "::" {
            return Err(CaravanError::Undefined);
        }
        return Ok((iter, span, is_there));
    }

    // if no colon, continue
    let span = span.join(current.span());
    let Some(span) = span else {
        return Err(CaravanError::Undefined);
    };
    return join_until_seperator(iter, span);
}
