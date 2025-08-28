use soroban_sdk::{contracttype, Address};

#[derive(Clone)]
#[contracttype]
pub enum BufferDataKey {
    Reserve(Address),
    MinTimeBetweenPayouts,
    LastPayoutTimestamp,
    MinReserveRatio,
    IsKilledDeposit,
    IsKilledResolveLiquidityDeficit,
}
