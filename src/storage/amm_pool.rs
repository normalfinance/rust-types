use soroban_sdk::contracttype;

#[derive(Clone)]
#[contracttype]
pub enum AmmPoolDataKey {
    ReserveA,
    ReserveB,
    Pool,
    Plane,
    Router,
    OracleRegistry,
    LastOracleValid,
    LastTradeTs,
    LastUpdateTs,
    Volume30d,
    IsKilledSwap,
    IsKilledDeposit,
    IsKilledWithdraw,
    IsKilledClaim,
    TokenFutureWASM,
}