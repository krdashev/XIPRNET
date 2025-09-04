//! Storage implementation for XIPRNET
//! 
//! Provides encrypted local storage and message synchronization

pub mod local;
pub mod sync;

pub use local::*;
pub use sync::*;