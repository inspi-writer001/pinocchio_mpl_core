#!/bin/bash

# Script to update all mod.rs files in the pinocchio_mpl_core project
# Run this from the root of your project directory

set -e

echo "📝 Updating all mod.rs files..."

# Main lib.rs
echo "Updating src/lib.rs..."
cat > src/lib.rs << 'EOF'
// Pinocchio MPL Core - A high-performance implementation of Metaplex Core

pub mod entrypoint;
pub mod error;
pub mod instruction;
pub mod plugins;
pub mod processor;
pub mod state;
pub mod utils;

// Re-exports for convenience
pub use error::*;
pub use instruction::*;
pub use plugins::*;
pub use processor::*;
pub use state::*;
pub use utils::*;

// Re-export pinocchio for users of this library
pub use pinocchio;
EOF

# Plugins main mod.rs
echo "Updating src/plugins/mod.rs..."
cat > src/plugins/mod.rs << 'EOF'
// Plugin system modules

pub mod external;
pub mod internal;
pub mod external_plugin_adapters;
pub mod lifecycle;
pub mod plugin_header;
pub mod plugin_registry;
pub mod utils;

// Re-exports
pub use external::*;
pub use internal::*;
pub use external_plugin_adapters::*;
pub use lifecycle::*;
pub use plugin_header::*;
pub use plugin_registry::*;
pub use utils::*;
EOF

# Plugins external mod.rs
echo "Updating src/plugins/external/mod.rs..."
cat > src/plugins/external/mod.rs << 'EOF'
// External plugin modules

pub mod app_data;
pub mod data_section;
pub mod lifecycle_hook;
pub mod linked_app_data;
pub mod linked_lifecycle_hook;
pub mod oracle;

// Re-exports
pub use app_data::*;
pub use data_section::*;
pub use lifecycle_hook::*;
pub use linked_app_data::*;
pub use linked_lifecycle_hook::*;
pub use oracle::*;
EOF

# Plugins internal mod.rs
echo "Updating src/plugins/internal/mod.rs..."
cat > src/plugins/internal/mod.rs << 'EOF'
// Internal plugin modules

pub mod authority_managed;
pub mod owner_managed;
pub mod permanent;

// Re-exports
pub use authority_managed::*;
pub use owner_managed::*;
pub use permanent::*;
EOF

# Plugins internal/authority_managed mod.rs
echo "Updating src/plugins/internal/authority_managed/mod.rs..."
cat > src/plugins/internal/authority_managed/mod.rs << 'EOF'
// Authority-managed plugin modules

pub mod add_blockers;
pub mod attributes;
pub mod immutable_metadata;
pub mod master_edition;
pub mod royalties;
pub mod update_delegate;
pub mod verified_creators;

// Re-exports
pub use add_blockers::*;
pub use attributes::*;
pub use immutable_metadata::*;
pub use master_edition::*;
pub use royalties::*;
pub use update_delegate::*;
pub use verified_creators::*;
EOF

# Plugins internal/owner_managed mod.rs
echo "Updating src/plugins/internal/owner_managed/mod.rs..."
cat > src/plugins/internal/owner_managed/mod.rs << 'EOF'
// Owner-managed plugin modules

pub mod autograph;
pub mod burn_delegate;
pub mod freeze_delegate;
pub mod freeze_execute;
pub mod transfer_delegate;

// Re-exports
pub use autograph::*;
pub use burn_delegate::*;
pub use freeze_delegate::*;
pub use freeze_execute::*;
pub use transfer_delegate::*;
EOF

# Plugins internal/permanent mod.rs
echo "Updating src/plugins/internal/permanent/mod.rs..."
cat > src/plugins/internal/permanent/mod.rs << 'EOF'
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
EOF

# Processor mod.rs
echo "Updating src/processor/mod.rs..."
cat > src/processor/mod.rs << 'EOF'
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
EOF

# State mod.rs
echo "Updating src/state/mod.rs..."
cat > src/state/mod.rs << 'EOF'
// State modules for account structures

pub mod asset;
pub mod collection;
pub mod collect;
pub mod compression_proof;
pub mod hashable_plugin_schema;
pub mod hashed_asset;
pub mod hashed_asset_schema;
pub mod traits;
pub mod update_authority;

// Re-exports
pub use asset::*;
pub use collection::*;
pub use collect::*;
pub use compression_proof::*;
pub use hashable_plugin_schema::*;
pub use hashed_asset::*;
pub use hashed_asset_schema::*;
pub use traits::*;
pub use update_authority::*;
EOF

# Utils mod.rs
echo "Updating src/utils/mod.rs..."
cat > src/utils/mod.rs << 'EOF'
// Utility modules

pub mod account;
pub mod compression;

// Re-exports
pub use account::*;
pub use compression::*;
EOF

echo ""
echo "✅ All mod.rs files updated successfully!"
echo ""
echo "Next steps:"
echo "1. Run 'cargo check' to verify the module structure"
echo "2. Start implementing the actual logic in each module"
echo "3. Test the build with 'cargo build'"