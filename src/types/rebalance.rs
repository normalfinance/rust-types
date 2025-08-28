use super::enums::ComponentAction;
use super::index::Component;
use soroban_sdk::{contracttype, Address, Vec};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ComponentUpdate {
    pub token: Address,
    pub new_weight: u128,
    pub action: ComponentAction,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ComponentAllocation {
    pub component: Component,
    pub current_balance: u128,
    pub target_balance: u128,
    pub percentage_of_nav: u128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RebalanceStatus {
    pub can_rebalance: bool,
    pub time_until_next_rebalance: u64,
    pub last_rebalance_ts: u64,
    pub rebalance_threshold: u64,
    pub is_public: bool,
    pub authorized_rebalancers: Vec<Address>,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RefactorParams {
    pub component_updates: Vec<ComponentUpdate>,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RebalanceParams {
    pub target_nav: Option<i128>,
}
