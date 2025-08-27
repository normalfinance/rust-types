use soroban_sdk::{contracttype, Address, Map};
use crate::types::{Component, ComponentAction};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MintEventData {
    pub ts: u64,
    pub user: Address,
    pub token_in: Address,
    pub amount_in: u128,
    pub shares_minted: u128,
    pub share_price: u128,
    pub nav_before: u128,
    pub nav_after: u128,
    pub total_shares_before: u128,
    pub total_shares_after: u128,
    pub fees_collected: u128,
    pub destination: Option<Address>,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RedemptionEventData {
    pub ts: u64,
    pub user: Address,
    pub shares_redeemed: u128,
    pub share_price: u128,
    pub nav_before: u128,
    pub nav_after: u128,
    pub total_shares_before: u128,
    pub total_shares_after: u128,
    pub component_payouts: Map<Address, u128>,
    pub fees_deducted: u128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RebalanceEventData {
    pub ts: u64,
    pub caller: Address,
    pub nav_before: u128,
    pub nav_after: u128,
    pub components_before: Map<Address, Component>,
    pub components_after: Map<Address, Component>,
    pub total_swaps: u32,
    pub gas_cost: u128,
    pub performance_impact: i128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RefactorEventData {
    pub ts: u64,
    pub caller: Address,
    pub components_before: Map<Address, Component>,
    pub components_after: Map<Address, Component>,
    pub components_updated: u32,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ComponentAddedEventData {
    pub ts: u64,
    pub admin: Address,
    pub token: Address,
    pub weight: u128,
    pub initial_balance: u128,
    pub nav_impact: u128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ComponentRemovedEventData {
    pub ts: u64,
    pub admin: Address,
    pub token: Address,
    pub final_balance: u128,
    pub proceeds_distributed: u128,
    pub nav_impact: u128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ComponentWeightUpdatedEventData {
    pub ts: u64,
    pub admin: Address,
    pub token: Address,
    pub old_weight: u128,
    pub new_weight: u128,
    pub balance_before: u128,
    pub balance_after: u128,
    pub nav_impact: u128,
}