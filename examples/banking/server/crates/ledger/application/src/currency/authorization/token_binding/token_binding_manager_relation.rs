use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;
use banking_ledger_domain::token_binding::TokenBinding;

use super::TokenBindingCurrencyRelation;
use crate::currency::authorization::CurrencyManagerRelation;

pub struct TokenBindingManagerRelation;

impl Relation for TokenBindingManagerRelation {
    const REF: RelationRef = RelationRef::new(TokenBinding::TYPE, RelationName::new("manager"));

    fn expr(&self) -> UsersetExpr {
        UsersetExpr::TupleToUserset {
            tupleset_relation: TokenBindingCurrencyRelation::REF.into(),
            computed_userset: CurrencyManagerRelation::REF.into(),
        }
    }
}
