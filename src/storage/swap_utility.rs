use crate::types::enums::DexProvider;
use soroban_sdk::contracttype;

#[contracttype]
#[derive(Clone)]
pub enum SwapUtilityDataKey {
    ProviderConfig(DexProvider),
    AdminAddress,
    Initialized,

    DefaultProvider,

    XlmTokenAddress,
}
