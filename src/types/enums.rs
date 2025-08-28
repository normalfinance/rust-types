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

// AMM-specific enums
#[contracttype]
#[derive(Default, Clone, PartialEq, Eq, Debug)]
pub enum PoolStatus {
    #[default]
    Initialized,
    Active,
    Frozen,
    ReduceOnly,
    Settlement,
    Delisted,
}

#[contracttype]
#[derive(Clone, Copy, PartialEq, Debug, Eq, PartialOrd, Ord, Default)]
pub enum PoolTier {
    A,
    B,
    C,
    Speculative,
    #[default]
    HighlySpeculative,
    Isolated,
}

#[contracttype]
#[derive(Clone, Copy, PartialEq, Debug, Eq)]
pub enum NormalAction {
    PoolInit,
    AddLiquidity,
    RemoveLiquidity,
    Swap,
    UpdateTwap,
    Rebalance,
    ClaimInsurance,
}

#[derive(Clone, Copy, PartialEq, Debug, Eq, Default)]
pub enum OracleValidity {
    NonPositive,
    TooVolatile,
    StaleForPool,
    Frozen,
    #[default]
    Valid,
}

#[derive(Clone, Copy, Eq, PartialEq, Debug, Default)]
pub enum OracleSource {
    #[default]
    Reflector,
    DIA,
}

#[contracttype]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum StakeAction {
    Deposit,
    WithdrawRequest,
    WithdrawCancelRequest,
    Withdraw,
}