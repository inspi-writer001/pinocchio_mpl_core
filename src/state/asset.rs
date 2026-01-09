use bytemuck::{Pod, Zeroable};
use pinocchio::{ProgramResult, account_info::AccountInfo, pubkey::Pubkey};

use crate::{CheckResult, Key, UpdateAuthority, state::PodStr};

#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy)]
pub struct AssetV1 {
    /// The account discriminator.
    pub key: Key,
    /// The owner of the asset.
    pub owner: Pubkey,
    /// The update authority of the asset.
    pub update_authority: UpdateAuthority,
    /// The name of the asset.
    pub name: PodStr<32>,
    /// The URI of the asset that points to the off-chain data.
    pub uri: PodStr<128>,
    /// no Options support in Pod storage
    pub seq_present: u8,
    /// The sequence number used for indexing with compression. u64
    pub seq: [u8; 8],
}

impl AssetV1 {
    /// The length of the asset account with exact size for name and uri and no seq.
    const LEN: usize = core::mem::size_of::<Self>();

    /// Create a new `Asset` with correct `Key` and `seq` of None.
    pub fn new(
        owner: Pubkey,
        update_authority: UpdateAuthority,
        name: PodStr<32>,
        uri: PodStr<128>,
    ) -> Self {
        Self {
            key: Key::AssetV1,
            owner,
            update_authority,
            name,
            uri,
            seq_present: 0,
            seq: [0u8; 8],
        }
    }

    /// If `asset.seq` is `Some(_)` then increment and save asset to account space.
    pub fn increment_seq(&mut self) -> ProgramResult {
        // TODO: Assert that account exists onchain before modifying
        if let Some(_seq) = self.get_seq() {
            let mut val = u64::from_le_bytes(self.seq);
            val = val.saturating_add(1);
            self.seq = val.to_le_bytes();
        };

        Ok(())
    }

    pub fn get_seq(&self) -> Option<u64> {
        if self.seq_present == 1 {
            Some(u64::from_le_bytes(self.seq))
        } else {
            None
        }
    }

    /// Check permissions for the create lifecycle event.
    pub fn check_create() -> CheckResult {
        CheckResult::CanApprove
    }

    /// Check permissions for the add plugin lifecycle event.
    pub fn check_add_plugin() -> CheckResult {
        CheckResult::CanApprove
    }

    /// Check permissions for the remove plugin lifecycle event.
    pub fn check_remove_plugin() -> CheckResult {
        CheckResult::CanApprove
    }

    /// Check permissions for the update plugin lifecycle event.
    pub fn check_update_plugin() -> CheckResult {
        CheckResult::None
    }

    /// Check permissions for the approve plugin authority lifecycle event.
    pub fn check_approve_plugin_authority() -> CheckResult {
        CheckResult::CanApprove
    }

    /// Check permissions for the revoke plugin authority lifecycle event.
    pub fn check_revoke_plugin_authority() -> CheckResult {
        CheckResult::CanApprove
    }

    /// Check permissions for the transfer lifecycle event.
    pub fn check_transfer() -> CheckResult {
        CheckResult::CanApprove
    }

    /// Check permissions for the burn lifecycle event.
    pub fn check_burn() -> CheckResult {
        CheckResult::CanApprove
    }

    /// Check permissions for the update lifecycle event.
    pub fn check_update() -> CheckResult {
        CheckResult::CanApprove
    }

    /// Check permissions for the compress lifecycle event.
    pub fn check_compress() -> CheckResult {
        CheckResult::CanApprove
    }

    /// Check permissions for the decompress lifecycle event.
    pub fn check_decompress() -> CheckResult {
        CheckResult::CanApprove
    }

    /// Check permissions for the add external plugin adapter lifecycle event.
    pub fn check_add_external_plugin_adapter() -> CheckResult {
        CheckResult::CanApprove
    }

    /// Check permissions for the remove external plugin adapter lifecycle event.
    pub fn check_remove_external_plugin_adapter() -> CheckResult {
        CheckResult::CanApprove
    }

    /// Check permissions for the update external plugin adapter lifecycle event.
    pub fn check_update_external_plugin_adapter() -> CheckResult {
        CheckResult::None
    }

    /// Check permissions for the execute lifecycle event.
    pub fn check_execute() -> CheckResult {
        CheckResult::CanApprove
    }
}
