// State modules for account structures

pub mod asset;
pub mod collect;
pub mod collection;
pub mod compression_proof;
pub mod hashable_plugin_schema;
pub mod hashed_asset;
pub mod hashed_asset_schema;
pub mod traits;
pub mod update_authority;

// Re-exports
pub use asset::*;
use bytemuck::{Pod, Zeroable};
pub use collect::*;
pub use collection::*;
pub use compression_proof::*;
pub use hashable_plugin_schema::*;
pub use hashed_asset::*;
pub use hashed_asset_schema::*;
pub use traits::*;
pub use update_authority::*;

#[macro_export]
macro_rules! enumify {
    (
        $vis:vis enum $name:ident {
            $(
                // Capture attributes (like doc comments) here
                $(#[$meta:meta])* $variant:ident $(= $val:expr)?
            ),* $(,)?
        }
    ) => {
        // The main wrapper struct (remains the same)
        #[repr(transparent)]
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, bytemuck::Pod, bytemuck::Zeroable)]
        $vis struct $name(u8);

        const _: () = {
            // Shadow enum purely for auto-increment math
            #[repr(u8)]
            #[allow(non_camel_case_types, dead_code)]
            enum __InnerShadow {
                $(
                    // Pass attributes here too (optional, but good for internal consistency)
                    $(#[$meta])*
                    $variant $(= $val)?
                ),*
            }

            // Inject Constants
            impl $name {
                $(
                    // Apply the captured doc comments to the public constants
                    $(#[$meta])*
                    #[allow(non_upper_case_globals)]
                    pub const $variant: Self = Self(__InnerShadow::$variant as u8);
                )*
            }
        };
    };
}

enumify! {
 pub enum DataState {
    /// The data is stored in account state.
    AccountState,
    /// The data is stored in the ledger history (compressed).
    LedgerState,
}
 }

enumify! {
    pub enum AuthorityType {
    /// No authority, used for immutability
    None,
    /// The owner of the core asset.
    Owner,
    /// The update authority of the core asset.
    UpdateAuthority,
    /// A pubkey that is the authority over a plugin.
    Address,
}}

enumify! {
    pub enum UpdateAuthorityType {
    /// No update authority, used for immutability.
    None,
    /// A standard address or PDA.
    Address,
    /// Authority delegated to a collection.
    Collection,
}}

enumify! {
    pub enum Key {
    /// Uninitialized or invalid account.
    Uninitialized,
    /// An account holding an uncompressed asset.
    AssetV1,
    /// An account holding a compressed asset.
    HashedAssetV1,
    /// A discriminator indicating the plugin header.
    PluginHeaderV1,
    /// A discriminator indicating the plugin registry.
    PluginRegistryV1,
    /// A discriminator indicating the collection.
    CollectionV1,
}
}

/// A fixed-size string wrapper for ZeroCopy structs.
/// N = Maximum bytes (e.g., 32).
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct PodStr<const N: usize> {
    pub len: u8,
    pub value: [u8; N],
}

// Safety: The struct consists entirely of u8s. Zero is a valid u8.
unsafe impl<const N: usize> Zeroable for PodStr<N> {}

// Safety:
// - It is #[repr(C)].
// - It has no padding (u8 alignment is 1, so fields are packed tight).
// - All bit patterns are valid for u8.
unsafe impl<const N: usize> Pod for PodStr<N> {}

impl<const N: usize> PodStr<N> {
    pub fn from_str(s: &str) -> Self {
        let mut value = [0u8; N];
        let bytes = s.as_bytes();
        let len = bytes.len().min(N); // Cap length to avoid panic
        value[..len].copy_from_slice(&bytes[..len]);

        Self {
            len: len as u8,
            value,
        }
    }

    pub fn as_str(&self) -> &str {
        let len = (self.len as usize).min(N);
        std::str::from_utf8(&self.value[..len]).unwrap_or("")
    }
}
