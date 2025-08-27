use soroban_sdk::{contracttype, Address, Vec};
use crate::types::enums::Role;


#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommitTransferOwnershipEventData {
    pub role: Role,
    pub new_address: Address,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ApplyTransferOwnershipEventData {
    pub role: Role,
    pub new_owner: Address,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RevertTransferOwnershipEventData {
    pub role: Role,
}


#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SetPrivilegedAddressesEventData {
    pub rewards_admin: Address,
    pub operations_admin: Address,
    pub pause_admin: Address,
    pub emergency_pause_admins: Vec<Address>,
}

// Emergency Mode Events (no data needed - flag determined by event name)
// Events: "enable_emergency_mode" and "disable_emergency_mode" with no data payload