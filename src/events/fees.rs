use soroban_sdk::{contracttype, Address};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccruedFeesCollectedEventData {
    pub ts: u64,
    pub user: Address,
    pub shares_before: u128,
    pub shares_after: u128,
    pub fee_period_start: u64,
    pub fee_period_end: u64,
    pub annual_fee_amount: u128,
    pub total_fee_collected: u128,
    pub manager_fee_portion: u128,
    pub protocol_fee_portion: u128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeesDistributedToManagerEventData {
    pub ts: u64,
    pub manager: Address,
    pub amount: u128,
    pub total_accumulated_before: u128,
    pub total_accumulated_after: u128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FeesDistributedToProtocolEventData {
    pub ts: u64,
    pub protocol_recipient: Address,
    pub amount: u128,
    pub total_accumulated_before: u128,
    pub total_accumulated_after: u128,
}
