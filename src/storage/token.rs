use soroban_sdk::{contracttype, Address};

#[derive(Clone)]
#[contracttype]
pub struct AllowanceDataKey {
    pub from: Address,
    pub spender: Address,
}

#[derive(Clone)]
#[contracttype]
pub enum AllowanceDataKeyEnum {
    Allowance(AllowanceDataKey),
}

#[derive(Clone)]
#[contracttype]
pub enum BalanceDataKey {
    Balance(Address),
}
