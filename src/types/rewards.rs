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

// Incentives configuration for a specific pool.
#[derive(Clone)]
#[contracttype]
pub struct PoolIncentiveConfig {
    pub reward_tps: u128,
    pub reward_expired_at: u64,
}

// Mutable pool incentive data that evolves over time.
#[derive(Clone)]
#[contracttype]
pub struct PoolIncentiveData {
    // rewards
    pub block: u64,
    pub accumulated_rewards: u128,
    pub claimed_rewards: u128,
    pub rewards_last_time: u64,
    // lp fees - Tracks how much of token_b has been collected as fees per unit of LP token, cumulatively.
    pub fee_growth_per_lp: u128,
}

// Per-user incentive data.
#[derive(Clone)]
#[contracttype]
pub struct UserIncentiveData {
    // rewards
    pub pool_accumulated_rewards: u128,
    pub rewards_to_claim: u128,
    pub last_block: u64,
    // lp fees
    pub fee_checkpoint: u128,
}
