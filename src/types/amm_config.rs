use super::enums::PoolTier;
use crate::types::config::PrivilegedAddresses;
use soroban_sdk::{contracttype, Address, BytesN, String, Symbol};

#[contracttype]
#[derive(Clone)]
pub struct TokenInitInfo {
    pub token_wasm_hash: BytesN<32>,
    pub name: String,
    pub symbol: String,
}

#[contracttype]
#[derive(Clone)]
pub struct RewardConfig {
    pub reward_token: Address,
}

#[contracttype]
#[derive(Clone)]
pub struct InitializeParams {
    pub admin: Address,
    pub privileged_addrs: PrivilegedAddresses,
    pub router: Address,
    pub oracle_registry: Address,
    pub assets: (Symbol, Symbol),
    pub synthetic_token_info: TokenInitInfo,
    pub lp_token_info: TokenInitInfo,
    pub token_b: Address,
    pub synthetic_sac_address: Address,
    pub fee_fraction: u32,
    pub tier: PoolTier,
    pub quote_max_insurance: u128,
}

#[contracttype]
#[derive(Clone)]
pub struct InitializeAllParams {
    pub base: InitializeParams,
    pub reward_config: RewardConfig,
    pub plane: Address,
}
