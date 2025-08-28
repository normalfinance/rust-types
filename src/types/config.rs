use soroban_sdk::{contracttype, Address, BytesN, Symbol, Vec};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FactoryConfig {
    pub swap_utility: Address,
    pub protocol_fee_amount: u128,
    pub max_manager_fee_amount: u128,
    pub protocol_fee_recipient: Address,
    pub minimum_fee_threshold: u128,
    pub index_contract_wasm: BytesN<32>,
    pub token_contract_wasm: BytesN<32>,
}

#[contracttype]
#[derive(Clone)]
pub struct PrivilegedAddresses {
    pub emergency_admin: Address,
    pub rewards_admin: Address,
    pub operations_admin: Address,
    pub pause_admin: Address,
    pub emergency_pause_admins: Vec<Address>,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WhitelistToken {
    pub address: Address,
    pub symbol: Symbol,
    pub active: bool,
}
