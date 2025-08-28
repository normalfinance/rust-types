use soroban_sdk::{contracttype, BytesN, Vec};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommitUpgradeEventData {
    pub new_wasms: Vec<BytesN<32>>,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ApplyUpgradeEventData {
    pub new_wasms: Vec<BytesN<32>>,
}

// Revert upgrade has no data - just the event name indicates the action
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RevertUpgradeEventData {
    pub timestamp: u64,
}
