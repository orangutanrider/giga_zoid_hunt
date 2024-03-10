use proc_macro::*;
use proc_macro::token_stream::IntoIter as TokenIter;

use super::*;


pub fn bindings_step(group: Group) -> Result<(Span, SingleQueryStep), CaravanError> {
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

pub fn till_query_fin(caravan: Caravan, current: Span,) -> Result<(Caravan, Span), CaravanError> {
    return join_until_seperator(caravan, current)
}

fn join_until_seperator(mut caravan: Caravan, current: Span) -> Result<(Caravan, Span), CaravanError> {
    let token = caravan.next();
    let Some(token) = token else {
        return Err(CaravanError::Undefined);
    };
    
    match token {
        TokenTree::Group(_) => {
            return Err(CaravanError::Undefined);
        },
        TokenTree::Punct(_) => {
            return end_if_seperator(caravan, current, token);
        },
        TokenTree::Ident(_) => {
            let current = current.join(token.span());
            let Some(current) = current else {
                return Err(CaravanError::Undefined);
            };

            return join_until_seperator(caravan, current);
        },
        TokenTree::Literal(_) => {
            let current = current.join(token.span());
            let Some(current) = current else {
                return Err(CaravanError::Undefined);
            };

            return join_until_seperator(caravan, current);
        },
    }
}

fn end_if_seperator(mut caravan: Caravan, output: Span, current: TokenTree) -> Result<(Caravan, Span), CaravanError> {
    // If colon expect :: and end
    if current.to_string() == ":" {
        let next = caravan.next();
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
        return Ok((caravan, output));
    }

    // if no colon, continue
    let output = output.join(current.span());
    let Some(output) = output else {
        return Err(CaravanError::Undefined);
    };
    return join_until_seperator(caravan, output);
}
