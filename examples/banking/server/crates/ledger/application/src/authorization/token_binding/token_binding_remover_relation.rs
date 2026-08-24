use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;
use banking_ledger_domain::token_binding::TokenBinding;

use super::TokenBindingCurrencyRelation;
use crate::authorization::CurrencyManagerRelation;

pub struct TokenBindingRemoverRelation;

impl Relation for TokenBindingRemoverRelation {
    const REF: RelationRef = RelationRef::new(TokenBinding::TYPE, RelationName::new("remover"));
    const EXPR: UsersetExpr = UsersetExpr::TupleToUserset {
        tupleset_relation: TokenBindingCurrencyRelation::REF,
        computed_userset: CurrencyManagerRelation::REF,
    };
}
