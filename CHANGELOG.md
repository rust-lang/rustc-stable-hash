# Unreleased

- Rename `StableHasherResult` to `FromStableHash` (#8)
- Use new-type for returned-hash of `SipHasher128`(`Hash`) (#8)
- Introduce multi hasher support (#8)
- `StableHasher::finish` now returns a small hash instead of being fatal (#6)
- Remove `StableHasher::finalize` (#4)
- Import stable hasher implementation from rustc ([db8aca48129](https://github.com/rust-lang/rust/blob/db8aca48129d86b2623e3ac8cbcf2902d4d313ad/compiler/rustc_data_structures/src/))
