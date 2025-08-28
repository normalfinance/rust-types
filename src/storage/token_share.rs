use soroban_sdk::contracttype;

#[derive(Clone)]
#[contracttype]
pub enum TokenShareDataKey {
    TokenShare,
    TotalShares,
}
