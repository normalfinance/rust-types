use soroban_sdk::contracttype;

#[derive(Clone)]
#[contracttype]
pub enum AccessControlDataKey {
    Admin,
    EmergencyAdmin,
    Operator,
    OperationsAdmin,
    PauseAdmin,
    EmPauseAdmins,

    FutureAdmin,
    FutureEmergencyAdmin,

    TransferOwnershipDeadline,
    EmAdminTransferOwnershipDeadline,

    EmergencyMode,
}
