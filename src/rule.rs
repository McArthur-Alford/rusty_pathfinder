use crate::entity_store::{EntityStore, Action};

pub enum RuleResult {
    Success,
    Reject
}

pub type Rule = dyn Fn(&EntityStore, &Action) -> RuleResult;

pub fn test_rule(entity: &EntityStore, action: &Action) -> RuleResult {
    RuleResult::Success,
}