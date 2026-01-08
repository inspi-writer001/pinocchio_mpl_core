use bytemuck::{Pod, Zeroable};
use pinocchio::pubkey::Pubkey;

use crate::UpdateAuthorityType;

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct UpdateAuthority {
    pub tag: UpdateAuthorityType,
    // We allocate space for the LARGEST variant (Pubkey = 32 bytes).
    // If tag == NONE, this is just 32 bytes of zeros (padding).
    pub payload: Pubkey,
}

impl UpdateAuthority {
    // --- Constructors ---
    pub fn none() -> Self {
        Self {
            tag: UpdateAuthorityType::None,
            payload: [0; 32],
        }
    }

    pub fn address(key: Pubkey) -> Self {
        Self {
            tag: UpdateAuthorityType::Address,
            payload: key,
        }
    }

    pub fn collection(key: Pubkey) -> Self {
        Self {
            tag: UpdateAuthorityType::Collection,
            payload: key,
        }
    }

    // --- Accessors ---

    /// Get the key if it exists, otherwise default
    pub fn key(&self) -> Pubkey {
        match self.tag {
            UpdateAuthorityType::Address | UpdateAuthorityType::Collection => self.payload,
            _ => Pubkey::default(),
        }
    }

    pub fn is_some(&self) -> bool {
        self.tag != UpdateAuthorityType::None
    }
}
