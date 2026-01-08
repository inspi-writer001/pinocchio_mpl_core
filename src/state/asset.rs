use bytemuck::{Pod, Zeroable};
use pinocchio::{ProgramResult, account_info::AccountInfo, pubkey::Pubkey};

use crate::{Key, UpdateAuthority, state::PodStr};

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
    /// The base length of the asset account with an empty name and uri and no seq.
    const BASE_LEN: usize = core::mem::size_of::<Self>();

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
    pub fn increment_seq_and_save(&mut self, account: &AccountInfo) -> ProgramResult {
        if let Some(seq) = &mut self.get_seq() {
            *seq = seq.saturating_add(1);
            self.save(account, 0)?;
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
}
