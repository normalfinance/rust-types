use soroban_sdk::{contracttype, Address};
use crate::types::enums::Operation;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ManagerAddressUpdatedEventData {
    pub ts: u64,
    pub admin: Address,
    pub old_manager: Address,
    pub new_manager: Address,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicStatusUpdatedEventData {
    pub ts: u64,
    pub admin: Address,
    pub old_status: bool,
    pub new_status: bool,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WhitelistStatusUpdatedEventData {
    pub ts: u64,
    pub admin: Address,
    pub user: Address,
    pub old_status: bool,
    pub new_status: bool,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BlacklistStatusUpdatedEventData {
    pub ts: u64,
    pub admin: Address,
    pub user: Address,
    pub old_status: bool,
    pub new_status: bool,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OperationStatusUpdatedEventData {
    pub ts: u64,
    pub admin: Address,
    pub operation: Operation,
    pub killed: bool,
}