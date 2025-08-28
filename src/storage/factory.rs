use soroban_sdk::{contracttype, Address};

#[derive(Clone)]
#[contracttype]
pub enum FactoryDataKey {
    SwapUtility,

    OracleRegistry,

    FeeTierConfig,

    UserVolumeHistory(Address),
    UserTierCache(Address),

    ProtocolFeeFraction,
    MaxManagerFeeFraction,

    ProtocolFeeRecipient,
    MinimumFeeThreshold,

    ProtocolFeeAmount,
    MaxManagerFeeAmount,
    MinimumSharesForFeeCollection,

    IndexContractWASM,
    TokenContractWASM,

    ContractSequence(Address),

    DeployedIndexes(Address),
    AllDeployedIndexes,

    IndexFeeEnabled(Address),

    IsKilledCreate,
}
