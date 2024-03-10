use proc_macro::*;
use proc_macro::token_stream::IntoIter as TokenIter;

use super::*;

fn single_query_step(caravan: Caravan, query: &str, entity: &str) -> Result<Caravan, CaravanError> {
    let result = binding_step(iter);
    if let Err(result) = result {
        return Err(result);
    }
    let Ok((iter, binding_decleration, mutability)) = result else {
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

enum BindingMutability {
    IsMutable,
    IsNotMutable
}

fn binding_step(mut iter: TokenIter) -> Result<(TokenIter, String, BindingMutability), CaravanError> {
    let group = iter.next();
    let Some(group) = group else {
        return Err(CaravanError::Undefined);
    };
    let TokenTree::Group(group) = group else {
        return Err(CaravanError::Undefined);
    };
    if group.delimiter() != Delimiter::Parenthesis {
        return Err(CaravanError::Undefined);
    }
    
    let (binding, mutability) = bindings_step(group);
    let binding = "let ".to_owned() + &binding + " = ";
    return Ok((iter, binding, mutability));
}

fn bindings_step(group: Group) -> (String, BindingMutability) {
    let mut output = "".to_owned();
    let mut detection = BindingMutability::IsNotMutable;

    let group = group.stream().into_iter();
    for token in group {
        let s = token.to_string();
        if s == "mut" {
            detection = BindingMutability::IsMutable;
        }

        output = output + &s;
    }

    return (output, detection);   
}