use super::enums::{DexProvider, SwapDirection};
use soroban_sdk::{contracttype, Address, String, Symbol, Vec};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SwapParams {
    pub provider: DexProvider,
    pub token_in: Address,
    pub token_out: Address,
    pub amount_in: u128,
    pub amount_out_min: u128,
    pub to: Address,
    pub asset: Symbol,
    pub direction: SwapDirection,
    pub fee_enabled: Option<bool>,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SwapResult {
    pub provider_used: DexProvider,
    pub amount_in: u128,
    pub amount_out: u128,
    pub success: bool,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderConfig {
    pub contract_address: Address,
    pub is_active: bool,
    pub max_slippage: u64,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DexDistribution {
    pub protocol_id: String,
    pub path: Vec<Address>,
    pub parts: u32,
}
