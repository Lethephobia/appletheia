use appletheia::application::authorization::{Relation, RelationName, RelationRef, UsersetExpr};
use appletheia::domain::Aggregate;
use banking_ledger_domain::token_binding::TokenBinding;

pub struct TokenBindingCurrencyRelation;

impl Relation for TokenBindingCurrencyRelation {
    const REF: RelationRef = RelationRef::new(TokenBinding::TYPE, RelationName::new("currency"));
    const EXPR: UsersetExpr = UsersetExpr::This;
}
