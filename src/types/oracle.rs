use super::enums::{OracleSource, OracleValidity};
use soroban_sdk::{contracttype, Address};

#[contracttype]
#[derive(Default, Clone, Copy, Debug)]
pub struct OraclePriceData {
    pub price: u128,
    pub delay: u64,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OracleInfo {
    pub address: Address,
    pub decimals: u32,
    pub frozen: bool,
    pub sanitize_clamp_denominator: i64,
    pub last_updated: u64,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct MutableOracleInfo {
    pub address: Option<Address>,
    pub decimals: Option<u32>,
    pub frozen: Option<bool>,
    pub sanitize_clamp_denominator: Option<i64>,
}

#[contracttype]
#[derive(Default, Clone, Copy, Eq, PartialEq, Debug)]
pub struct HistoricalOracleData {
    pub last_oracle_price: u128,
    pub last_oracle_delay: u64,
    pub last_oracle_price_twap: u128,
    pub last_oracle_price_twap_ts: u64,
}

#[contracttype]
#[derive(Copy, Clone, Debug)]
pub struct PriceDivergenceGuardRails {
    pub oracle_twap_percent_divergence: u64,
}

#[contracttype]
#[derive(Copy, Clone, Default, Debug)]
pub struct ValidityGuardRails {
    pub seconds_before_stale_for_pool: u64,
    pub too_volatile_ratio: i64,
}

#[contracttype]
#[derive(Copy, Clone, Debug)]
pub struct OracleGuardRails {
    pub price_divergence: PriceDivergenceGuardRails,
    pub validity: ValidityGuardRails,
}

#[derive(Default, Clone, Copy, Debug)]
pub struct OracleStatus {
    pub price_data: OraclePriceData,
    pub oracle_reserve_price_spread_pct: i64,
    pub price_too_divergent: bool,
    pub oracle_validity: OracleValidity,
}
