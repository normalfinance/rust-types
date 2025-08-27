use soroban_sdk::contracttype;
use crate::types::enums::DexProvider;

#[contracttype]
#[derive(Clone)]
pub enum SwapUtilityDataKey {
    
    ProviderConfig(DexProvider),
    AdminAddress,
    Initialized,

    DefaultProvider,

    XlmTokenAddress,
}