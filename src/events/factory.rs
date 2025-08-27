use soroban_sdk::{contracttype, Address, Vec, BytesN};

// Index Deployment Events
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexDeployedEventData {
    pub ts: u64,
    pub deployer: Address,
    pub index_address: Address,
    pub operator: Address,
    pub manager: Address,
    pub fee_destination: Address,
    pub max_swap_fee_fraction: u32,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexConfigEventData {
    pub ts: u64,
    pub index_address: Address,
    pub max_swap_fee_fraction: u32,
    pub base_nav: u128,
    pub initial_price: u128,
    pub is_public: bool,
    pub deployment_cost: u128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IndexComponentsEventData {
    pub ts: u64,
    pub index_address: Address,
    pub initial_components: Vec<Address>,
    pub initial_weights: Vec<u128>,
}

// Factory Configuration Events
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProtocolFeeUpdatedEventData {
    pub ts: u64,
    pub admin: Address,
    pub old_fee: u128,
    pub new_fee: u128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WasmUpdatedEventData {
    pub ts: u64,
    pub admin: Address,
    pub old_wasm: BytesN<32>,
    pub new_wasm: BytesN<32>,
    pub version: u32,
}