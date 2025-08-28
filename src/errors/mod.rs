pub mod access_control_error;
pub mod index_error;
pub mod math_error;
pub mod storage_error;
pub mod swap_error;
pub mod token_error;
pub mod upgrade_error;
pub mod validation_error;

// AMM-specific error modules
pub mod buffer_error;
pub mod insurance_fund_error;
pub mod pool_error;
pub mod pool_router_error;
pub mod liquidity_calculator_error;
pub mod oracle_error;

pub use access_control_error::AccessControlError;
pub use index_error::IndexError;
pub use math_error::MathError;
pub use storage_error::StorageError;
pub use swap_error::SwapError;
pub use token_error::TokenError;
pub use upgrade_error::UpgradeError;
pub use validation_error::ValidationError;

// AMM-specific error exports
pub use buffer_error::BufferError;
pub use insurance_fund_error::InsuranceFundError;
pub use pool_error::{PoolError, PoolValidationError};
pub use pool_router_error::PoolRouterError;
pub use liquidity_calculator_error::LiquidityPoolCalculatorError;
pub use oracle_error::OracleError;