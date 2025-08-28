use soroban_sdk::{contracttype, U256};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GlobalRewardsConfig {
    pub tps: u128,
    pub expired_at: u64,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PoolRewardInfo {
    pub processed: bool,
    pub total_liquidity: U256,
}