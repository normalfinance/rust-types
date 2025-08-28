use soroban_sdk::{contracttype, Address};

// AMM Pool Operation Events
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DepositLiquidityEventData {
    pub token: Address,
    pub user: Address,
    pub amount: u128,
    pub share_amount: u128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WithdrawLiquidityEventData {
    pub token: Address,
    pub user: Address,
    pub share_amount: u128,
    pub amount: u128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AmmSwapEventData {
    pub user: Address,
    pub token_in: Address,
    pub token_out: Address,
    pub in_amount: u128,
    pub out_amount: u128,
    pub fee_amount: u128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RebalanceEventData {
    pub reserve_a: u128,
    pub reserve_b: u128,
    pub new_reserve_a: u128,
    pub new_reserve_b: u128,
    pub delta_a: i128,
}

// Buffer Events
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BufferDepositEventData {
    pub token: Address,
    pub user: Address,
    pub amount: u128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolveLiquidityDeficitEventData {
    pub pool: Address,
    pub token: Address,
    pub user: Address,
    pub amount: u128,
    pub paid: u128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WithdrawSurplusEventData {
    pub token: Address,
    pub user: Address,
    pub amount: u128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SkimEventData {
    pub token: Address,
    pub user: Address,
    pub amount: i128,
}