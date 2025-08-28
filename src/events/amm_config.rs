use soroban_sdk::{contracttype, Address, Symbol};
use crate::types::enums::Operation;

// AMM Kill Switch Events (same pattern as index kill switch events)
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AmmOperationStatusUpdatedEventData {
    pub ts: u64,
    pub admin: Address,
    pub operation: Operation, // Using existing Operation enum
    pub killed: bool,
}

// Insurance Fund Events
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InsuranceFundDepositEventData {
    pub user: Address,
    pub amount: u128,
    pub shares_minted: u128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InsuranceFundWithdrawRequestEventData {
    pub user: Address,
    pub shares_requested: u128,
    pub value_at_request: u128,
    pub request_timestamp: u64,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InsuranceFundWithdrawEventData {
    pub user: Address,
    pub shares_burned: u128,
    pub amount_withdrawn: u128,
}

// Oracle Configuration Events
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OracleConfigUpdatedEventData {
    pub ts: u64,
    pub admin: Address,
    pub oracle: Address,
    pub asset: Symbol,
    pub frozen: bool,
}