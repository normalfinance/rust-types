use soroban_sdk::{contracttype, Address};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InsuranceFundReserve {
    pub token: Address,
    pub balance: u128,
    pub total_shares: u128,
    pub shares_base: u128,
    pub total_deposits: u128,
    pub total_withdrawals: u128,
    pub total_claims: u128,
    pub last_claim: u128,
    pub last_claim_ts: u64,
    pub last_update_ts: u64,
}