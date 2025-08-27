use soroban_sdk::{contracttype, Address};
use crate::types::enums::DexProvider;


#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SwapExecutedEventData {
    pub provider: DexProvider,
    pub token_in: Address,
    pub token_out: Address,
    pub amount_in: u128,
    pub amount_out: u128,
    pub user: Address,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SwapFailedEventData {
    pub provider: DexProvider,
    pub token_in: Address,
    pub token_out: Address,
    pub amount_in: u128,
    pub error_code: u32,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderConfigEventData {
    pub provider: DexProvider,
    pub contract_address: Address,
    pub admin: Address,
}