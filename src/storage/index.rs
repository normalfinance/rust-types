use soroban_sdk::{contracttype, Address};

#[derive(Clone)]
#[contracttype]
pub enum IndexDataKey {
    Factory,
    SwapUtility, 
    TokenIndex,

    BaseNAV, 
    InitialPrice, 

    Component(Address), 
    ComponentBalance(Address),

    Public, 

    ManagerFeeFraction, 

    
    ManagerFeeAmount,
    ProtocolFeeAmount,
    MinimumSharesForFeeCollection,

    
    ManagerAddress,
    ProtocolFeeRecipient,
    AccumulatedManagerFees,
    AccumulatedProtocolFees,
    LastFeeCollection,

    Whitelist(Address), 
    Blacklist(Address), 

    RebalanceThreshold,

    LastRebalanceTs,
    LastUpdatedTs,

    
    TotalMints,
    TotalRedemptions,
    TotalFees,

    
    IsKilledMint,
    IsKilledRedeem,
    IsKilledRebalance,

    
    ComponentRegistry, 

    
    RebalanceAuthority(Address),
    RebalanceAuthorityRegistry,

    
    SwapUtilityAddress,
}

#[derive(Clone)]
#[contracttype]
pub enum FeeDataKey {
    UserFeeState(Address),
    LastBatchCollection,
}