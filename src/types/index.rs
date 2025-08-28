use soroban_sdk::{contracttype, Address, String, Symbol, Vec};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Component {
    pub asset: Symbol,
    pub weight: u128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexInfo {
    pub address: Address,
    pub token_address: Address,
    pub total_shares: u128,
    pub base_nav: u128,
    pub initial_price: u128,
    pub is_public: bool,
    pub manager_fee_amount: u128,
    pub manager_address: Address,
    pub protocol_fee_recipient: Address,
    pub accumulated_manager_fees: u128,
    pub accumulated_protocol_fees: u128,
    pub last_rebalance_ts: u64,
    pub last_updated_ts: u64,
    pub total_mints: u128,
    pub total_redemptions: u128,
    pub total_fees: u128,
    pub is_killed_mint: bool,
    pub is_killed_redeem: bool,
    pub is_killed_rebalance: bool,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexMetrics {
    pub total_shares: u128,
    pub total_mints: u128,
    pub total_redemptions: u128,
    pub total_fees: u128,
    pub accumulated_manager_fees: u128,
    pub accumulated_protocol_fees: u128,
    pub current_nav: u128,
    pub share_price: u128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexStatus {
    pub is_killed_mint: bool,
    pub is_killed_redeem: bool,
    pub is_killed_rebalance: bool,
    pub is_public: bool,
    pub can_rebalance: bool,
    pub last_rebalance_ts: u64,
    pub rebalance_threshold: u64,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexParams {
    pub admin: Address,
    pub rebalance_authorities: Vec<Address>,
    pub whitelist_accounts: Vec<Address>,
    pub blacklist_accounts: Vec<Address>,
    pub public: bool,
    pub name: String,
    pub token_symbol: String,
    pub description: String,
    pub base_nav: u128,
    pub initial_price: u128,
    pub initial_deposit: u128,
    pub manager_fee_amount: u128,
    pub components: Vec<Address>,
}
