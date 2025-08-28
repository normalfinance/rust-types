use super::enums::{PoolStatus, PoolTier};
use soroban_sdk::{contracttype, Address, Symbol, Vec};

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
    pub user: Address,
    pub token: Address,
    pub shares: u128,
    pub base: u128,
    pub if_shares: u128,
    pub last_withdraw_request_shares: u128,
    pub if_base: u128,
    pub last_withdraw_request_value: u128,
    pub last_withdraw_request_ts: u64,
    pub cost_basis: u128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PoolPlaneType {
    pub init_args: Vec<u128>,
    pub reserves: Vec<u128>,
}

impl Pool {
    /// Returns true if the pool is in ReduceOnly status
    pub fn is_reduce_only(&self) -> bool {
        self.status == PoolStatus::ReduceOnly
    }

    /// Returns true if the pool is in Settlement or Delisted status
    pub fn is_in_settlement(&self, _now: u64) -> bool {
        matches!(self.status, PoolStatus::Settlement | PoolStatus::Delisted)
    }

    /// Gets the sanitize clamp denominator based on pool tier
    pub fn get_sanitize_clamp_denominator(&self) -> Option<i64> {
        match self.tier {
            PoolTier::A => Some(10_i64),         // 10%
            PoolTier::B => Some(5_i64),          // 20%
            PoolTier::C => Some(2_i64),          // 50%
            PoolTier::Speculative => None,       // DEFAULT_MAX_TWAP_UPDATE_PRICE_BAND_DENOMINATOR
            PoolTier::HighlySpeculative => None, // DEFAULT_MAX_TWAP_UPDATE_PRICE_BAND_DENOMINATOR
            PoolTier::Isolated => None,          // DEFAULT_MAX_TWAP_UPDATE_PRICE_BAND_DENOMINATOR
        }
    }

    /// Gets the insurance coverage multiplier based on pool tier
    pub fn get_insurance_coverage_multiplier(&self) -> u64 {
        match self.tier {
            PoolTier::A => 10_u64, // 10%
            PoolTier::B => 5_u64,  // 20%
            PoolTier::C => 2_u64,  // 50%
            PoolTier::Speculative => 10_u64,
            PoolTier::HighlySpeculative => 10_u64,
            PoolTier::Isolated => 10_u64,
        }
    }

    /// Calculates the output amount and fee for a given input amount in a swap
    /// Returns (output_amount, fee_amount)
    pub fn get_amount_out(
        &self,
        e: &soroban_sdk::Env,
        in_amount: u128,
        reserve_sell: u128,
        reserve_buy: u128,
    ) -> (u128, u128) {
        use soroban_fixed_point_math::SorobanFixedPoint;
        
        if in_amount == 0 {
            return (0, 0);
        }

        const FEE_MULTIPLIER: u128 = 10000;
        
        // in * reserve_buy / (reserve_sell + in) - fee
        let result = in_amount.fixed_mul_floor(e, &reserve_buy, &(reserve_sell + in_amount));
        let fee = result.fixed_mul_ceil(e, &(self.fee_fraction as u128), &FEE_MULTIPLIER);
        (result - fee, fee)
    }
}

impl PoolTier {
    /// Returns true if this tier is as safe as the other tier (A is safest)
    pub fn is_as_safe_as(&self, other: &PoolTier) -> bool {
        // Pool Tier A safest
        self <= other
    }
}
