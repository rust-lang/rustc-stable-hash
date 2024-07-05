//! A stable hashing algorithm used by rustc

#![cfg_attr(feature = "nightly", feature(hasher_prefixfree_extras))]

mod int_overflow;
mod sip128;
mod stable_hasher;

/// Hashers collection
pub mod hashers {
    #[doc(inline)]
    pub use super::sip128::SipHasher128;

    /// Stable 128-bits Sip Hasher
    ///
    /// [`StableHasher`] version of [`SipHasher128`].
    ///
    /// [`StableHasher`]: super::StableHasher
    pub type StableSipHasher128 = super::StableHasher<SipHasher128>;
}

#[doc(inline)]
pub use stable_hasher::StableHasher;

#[doc(inline)]
pub use stable_hasher::StableHasherResult;

#[doc(inline)]
pub use stable_hasher::ExtendedHasher;

#[doc(inline)]
pub use hashers::StableSipHasher128;
