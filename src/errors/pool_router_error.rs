use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone)]
#[repr(u32)]
pub enum PoolRouterError {
    PoolNotFound = 301,
    BadFee = 302,
    PathIsEmpty = 307,
    TokensAreNotForReward = 308,
    LiquidityNotFilled = 309,
    LiquidityAlreadyFilled = 310,
    LiquidityCalculationError = 312,
    RewardsNotConfigured = 313,
    RewardsAlreadyConfigured = 314,
    DuplicatesNotAllowed = 315,
    TokensNotSorted = 2002,
    InMaxNotSatisfied = 2020,
}
