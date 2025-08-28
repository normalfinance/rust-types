use soroban_sdk::contracttype;

#[derive(Clone)]
#[contracttype]
pub enum InsuranceFundDataKey {
    Token,
    Router,
    UnstakingPeriod,
    OptimalInsurance,
    TotalShares,
    SharesBase,
    OptimalUtilization,
    BaseRate,
    RateSlopeA,
    RateSlopeB,
    IsKilledDeposit,
    IsKilledRequestWithdraw,
    IsKilledWithdraw,
}
