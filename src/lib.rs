//! A stable hashing algorithm used by rustc

#![cfg_attr(feature = "nightly", feature(hasher_prefixfree_extras))]

mod int_overflow;
mod sip128;
mod stable_hasher;

#[doc(inline)]
pub use stable_hasher::StableHasher;

#[doc(inline)]
pub use stable_hasher::StableHasherResult;

#[doc(inline)]
pub use stable_hasher::ExtendedHasher;

pub use sip128::SipHasher128; // TODO: Should SipHasher128 be exposed?

/// Stable Sip Hasher 128
pub type StableSipHasher128 = StableHasher<SipHasher128>;
