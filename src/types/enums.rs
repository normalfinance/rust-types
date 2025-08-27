use soroban_sdk::contracttype;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DexProvider {
    Normal,
    Soroswap,
}

#[contracttype]
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum SwapDirection {
    Buy,
    Sell,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ComponentAction {
    Add,
    Remove,
    UpdateWeight,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Role {
    Admin,
    EmergencyAdmin,
    RewardsAdmin,
    OperationsAdmin,
    PauseAdmin,
    EmergencyPauseAdmin,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Operation {
    Mint,
    Redeem,
    Rebalance,
}