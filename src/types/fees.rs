use soroban_sdk::{contracttype, Address, Map};

#[contracttype]
pub struct UserFeeState {
    pub balance: i128,
    pub last_fee_update: u64,
    pub accrued_manager_fees: u128,
    pub accrued_protocol_fees: u128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeeTierConfig {
    pub tier_rates: Map<u128, u32>,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserVolumeEntry {
    pub timestamp: u64,
    pub usd_amount: u128,
    pub index_address: Address,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserTierData {
    pub current_tier_threshold: u128,
    pub current_fee_rate_bps: u32,
    pub total_30_day_volume: u128,
    pub last_calculated: u64,
    pub last_volume_update: u64,
}
