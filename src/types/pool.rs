use soroban_sdk::{contracttype, Address, Symbol};
use super::enums::{PoolTier, PoolStatus};

#[contracttype]
#[derive(Default, Clone, Eq, PartialEq, Debug)]
pub struct InsuranceClaim {
    pub rev_withdraw_since_last_settle: i128,
    pub quote_max_insurance: u128,
    pub quote_settled_insurance: u128,
    pub last_revenue_withdraw_ts: u64,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Pool {
    pub token_b: Address,
    pub base_asset: Symbol,
    pub quote_asset: Symbol,
    pub tier: PoolTier,
    pub status: PoolStatus,
    pub fee_fraction: u32,
    pub insurance_claim: InsuranceClaim,
    pub liquidity_max_imbalance: u128,
    pub expiry_ts: u64,
    pub expiry_price: u128,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AddressAndAmount {
    pub address: Address,
    pub amount: u128,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PoolResponse {
    pub pool: Pool,
    pub token_a: AddressAndAmount,
    pub token_b: AddressAndAmount,
    pub token_share: AddressAndAmount,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PoolInfo {
    pub pool_address: Address,
    pub pool_response: PoolResponse,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Reserve {
    pub balance: u128,
    pub max_balance: u128,
    pub total_inflow: u128,
    pub total_outflow: u128,
    pub total_withdraw: u128,
    pub last_payout: u128,
    pub last_payout_ts: u64,
    pub last_update_ts: u64,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Stake {
    pub if_shares: u128,
    pub last_withdraw_request_shares: u128,
    pub if_base: u128,
    pub last_withdraw_request_value: u128,
    pub last_withdraw_request_ts: u64,
    pub cost_basis: u128,
}