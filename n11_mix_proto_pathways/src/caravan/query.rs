mod internal;

use proc_macro::*;
use proc_macro::token_stream::IntoIter as TokenIter;

pub use super::*;
use internal::*;

enum SingleQueryStep {
    Get,
    GetMut
}

fn query_step<'o>(mut caravan: Caravan<'o>, entity_input: &'o str) -> Result<Caravan<'o>, CaravanError> {
    let token = caravan.next();
    let Some(token) = token else {
        return Ok(caravan);
    };

    match token {
        TokenTree::Group(group) => { // one entity clause to many queries
            let group = Caravan::new(group.stream().into_iter(), caravan.output);
            
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
            return single_query_step(caravan, entity_input)
        },
        TokenTree::Punct(_) => {
            return Err(CaravanError::Undefined)
        },
        TokenTree::Literal(_) => {
            return Err(CaravanError::Undefined)
        },
    }
}

fn multi_query_step(caravan: Caravan, entity_input: &str) -> Result<Caravan, CaravanError> {

}

fn single_query_step<'o>(mut caravan: Caravan<'o>, entity_input: &'o str) -> Result<Caravan<'o>, CaravanError> {
    let bindings = binding_step(iter);
    if let Err(bindings) = bindings {
        return Err(bindings);
    }
    let Ok() = result else {
        return Err(CaravanError::Undefined)
    };

    let mut output = binding_decleration + query + ".get";

    match mutability {
        BindingMutability::IsMutable => {
            output = output + "_mut(";
        },
        BindingMutability::IsNotMutable => {
            output = output + "(";
        },
    }
    output = output + entity + ");";

    return e;
}

/// Outputs the bindings statement and infers query kind from them.
fn binding_step(mut caravan: Caravan) -> Result<(Caravan, Span, SingleQueryStep), CaravanError> {
    // enter ()
    let group = caravan.next();
    let Some(group) = group else {
        return Err(CaravanError::Undefined);
    };
    let TokenTree::Group(group) = group else {
        return Err(CaravanError::Undefined);
    };
    if group.delimiter() != Delimiter::Parenthesis {
        return Err(CaravanError::Undefined);
    }
    
    let result = bindings_step(group);
    if let Err(result) = result {
        return Err(result)
    }
    let Ok((binding, kind)) = result else {
        return Err(CaravanError::Undefined)
    };

    return Ok((caravan, binding, kind));
}