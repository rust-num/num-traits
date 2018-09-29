//! Traits for safe casting in a generic context.
//!
//! Each of the traits herein has a varying degree of safety. Raw casting is
//! implicitly dangerous because it can perform four possible actions:
//! * Convert from integer to float or vice versa.
//! * Convert from signed to unsigned or vice versa.
//! * Increases the size (or precision) of a number without loss.
//! * Decreases the size (or precision) of a number with loss.
//!
//! By using the raw cast operation you may get unintentional side effects.
//! These will often compile without warning and appear only as bugs later.
//!
//! The purpose of this module is to provide zero-cost abstraction traits that
//! encapsulate each discrete action of a cast. When using the methods on these
//! traits, you will only ever get the intended explicit behavior.
//!
//! For example, here is a generic implementation of a trivial function which
//! takes any integer type that can hold at least 12 bits and returns the 12
//! least significant bits. Note that attempting to pass an 8-bit integer to
//! this function would result in a compiler error (since `TrimInto<u16>` is
//! not implemented for `u8`).
//!
//! ```rust
//! use num_traits::cast::safe::{SignCast, TrimInto};
//!
//! const MASK: u16 = 0b0000111111111111;
//!
//! fn low12<T: TrimInto<u16>, C: SignCast<Unsigned=T>>(x: C) -> u16 {
//!     x.unsigned().trim() & MASK
//! }
//!
//! assert_eq!(low12(u64::max_value()), u64::max_value() as u16 & MASK);
//! assert_eq!(low12(-1i32), -1i32 as u16 & MASK);
//! ```
//!
//! It is recommended that you use the most specific type of cast appropriate
//! for your own use. Using a more generalized cast results in reduced compile
//! time validation of your code.
//!
//! Recommended cast types are:
//!
//!   * `GrowFrom<T>` / `GrowInto<T>`
//!   * `TrimFrom<T>` / `TrimInto<T>`
//!   * `SignCast`

mod cast;
mod grow;
mod sign;
mod trim;
mod size;

pub use self::cast::{CastFrom, CastInto};
pub use self::size::{SizeFrom, SizeInto};
pub use self::grow::{GrowFrom, GrowInto};
pub use self::trim::{TrimFrom, TrimInto};
pub use self::sign::SignCast;
