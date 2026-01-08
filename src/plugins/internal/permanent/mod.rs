// Permanent plugin modules

pub mod bubblegum_v2;
pub mod edition;
pub mod permanent_burn_delegate;
pub mod permanent_freeze_delegate;
pub mod permanent_freeze_execute;
pub mod permanent_transfer_delegate;

// Re-exports
pub use bubblegum_v2::*;
pub use edition::*;
pub use permanent_burn_delegate::*;
pub use permanent_freeze_delegate::*;
pub use permanent_freeze_execute::*;
pub use permanent_transfer_delegate::*;
