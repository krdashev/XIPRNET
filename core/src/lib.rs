//! XIPRNET Core Library
//! 
//! This library provides the core cryptography and protocol implementation
//! for the XIPRNET secure messaging system.

pub mod crypto;
pub mod protocol;
pub mod storage;
pub mod utils;

pub use crypto::*;
pub use protocol::*;
pub use storage::*;
pub use utils::*;

/// Re-export common types
pub mod prelude {
    pub use crate::crypto::{HpkeCipher, KeyPair, PublicKey};
    pub use crate::protocol::{Message, Session, User};
    pub use crate::storage::{EncryptedStorage};
    pub use crate::utils::{Error, Result};
}
