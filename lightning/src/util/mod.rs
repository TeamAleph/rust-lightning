//! Some utility modules live here. See individual sub-modules for more info.

#[macro_use]
pub(crate) mod fuzz_wrappers;

pub mod events;
pub mod errors;
pub mod ser;

pub(crate) mod byte_utils;
pub(crate) mod chacha20;
#[cfg(not(feature = "fuzztarget"))]
pub(crate) mod poly1305;
pub(crate) mod chacha20poly1305rfc;
pub(crate) mod transaction_utils;

#[macro_use]
pub(crate) mod ser_macros;
#[macro_use]
pub(crate) mod macro_logger;

// These have to come after macro_logger to build
pub mod logger;
pub mod config;

#[cfg(test)]
pub(crate) mod test_utils;

/// impls of traits that add exra enforcement on the way they're called. Useful for detecting state
/// machine errors and used in fuzz targets and tests.
#[cfg(any(test, feature = "fuzztarget"))]
pub mod enforcing_trait_impls;
