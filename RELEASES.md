# Release 0.2.1

- [The new `FloatCore` trait][32] offers a subset of `Float` for `#![no_std]` use.
  [This includes everything][41] except the transcendental functions and FMA.
- [The new `Inv` trait][37] returns the multiplicative inverse, or reciprocal.
- [The new `Pow` trait][37] performs exponentiation, much like the existing `pow`
  function, but with generic exponent types.
- [The new `One::is_one` method][39] tests if a value equals 1.  Implementers
  should override this method if there's a more efficient way to check for 1,
  rather than comparing with a temporary `one()`.

**Contributors**: @clarcharr, @cuviper, @vks

[32]: https://github.com/rust-num/num-traits/pull/32
[37]: https://github.com/rust-num/num-traits/pull/37
[39]: https://github.com/rust-num/num-traits/pull/39
[41]: https://github.com/rust-num/num-traits/pull/41


# Release 0.2.0

- **breaking change**: [There is now a `std` feature][30], enabled by default, along
  with the implication that building *without* this feature makes this a
  `#![no_std]` crate.
  - The `Float` and `Real` traits are only available when `std` is enabled.
  - Otherwise, the API is unchanged, and num-traits 0.1.43 now re-exports its
    items from num-traits 0.2 for compatibility (the [semver-trick]).

**Contributors**: @cuviper, @termoshtt, @vks

[semver-trick]: https://github.com/dtolnay/semver-trick
[30]: https://github.com/rust-num/num-traits/pull/30


# Release 0.1.43

- All items are now [re-exported from num-traits 0.2][31] for compatibility.

[31]: https://github.com/rust-num/num-traits/pull/31


# Release 0.1.42

- [num-traits now has its own source repository][num-356] at [rust-num/num-traits][home].
- [`ParseFloatError` now implements `Display`][22].
- [The new `AsPrimitive` trait][17] implements generic casting with the `as` operator.
- [The new `CheckedShl` and `CheckedShr` traits][21] implement generic
  support for the `checked_shl` and `checked_shr` methods on primitive integers.
- [The new `Real` trait][23] offers a subset of `Float` functionality that may be applicable to more
  types, with a blanket implementation for all existing `T: Float` types.

Thanks to @cuviper, @Enet4, @fabianschuiki, @svartalf, and @yoanlcq for their contributions!

[home]: https://github.com/rust-num/num-traits
[num-356]: https://github.com/rust-num/num/pull/356
[17]: https://github.com/rust-num/num-traits/pull/17
[21]: https://github.com/rust-num/num-traits/pull/21
[22]: https://github.com/rust-num/num-traits/pull/22
[23]: https://github.com/rust-num/num-traits/pull/23


# Prior releases

No prior release notes were kept.  Thanks all the same to the many
contributors that have made this crate what it is!
