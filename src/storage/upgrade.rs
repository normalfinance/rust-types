use soroban_sdk::contracttype;

#[derive(Clone)]
#[contracttype]
pub enum UpgradeDataKey {
    UpgradeDeadline,
    FutureWASM,
}
