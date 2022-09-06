#![no_std]
//! [`stylish`] helpers for discarding styles.

#![allow(uncommon_codepoints)]
#![doc(test(attr(deny(warnings))))]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

#[cfg(doc)]
extern crate self as stylish;

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "alloc")]
mod format;
#[cfg(feature = "std")]
pub mod io;
mod plain;
#[cfg(all(feature = "alloc", feature = "macros"))]
mod to_string;

#[cfg(feature = "alloc")]
pub use self::format::format;
pub use self::plain::Plain;
#[cfg(all(feature = "alloc", feature = "macros"))]
pub use self::to_string::ToPlainString;

#[cfg(feature = "macros")]
#[doc(hidden)]
pub mod 𓀄 {
    pub use stylish_core::format_args;
}
