// Processor modules for handling instructions

pub mod add_external_plugin_adapter;
pub mod add_plugin;
pub mod approve_plugin_authority;
pub mod burn;
pub mod collect;
pub mod compress;
pub mod create;
pub mod create_collection;
pub mod decompress;
pub mod execute;
pub mod remove_external_plugin_adapter;
pub mod remove_plugin;
pub mod revoke_plugin_authority;
pub mod transfer;
pub mod update;
pub mod update_collection_info;
pub mod update_external_plugin_adapter;
pub mod update_plugin;
pub mod write_external_plugin_adapter_data;

// Re-exports
pub use add_external_plugin_adapter::*;
pub use add_plugin::*;
pub use approve_plugin_authority::*;
pub use burn::*;
pub use collect::*;
pub use compress::*;
pub use create::*;
pub use create_collection::*;
pub use decompress::*;
pub use execute::*;
pub use remove_external_plugin_adapter::*;
pub use remove_plugin::*;
pub use revoke_plugin_authority::*;
pub use transfer::*;
pub use update::*;
pub use update_collection_info::*;
pub use update_external_plugin_adapter::*;
pub use update_plugin::*;
pub use write_external_plugin_adapter_data::*;
