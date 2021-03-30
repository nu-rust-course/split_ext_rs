#![doc(html_root_url = "https://docs.rs/split_ext/0.1.1")]
//! Extension traits for splitting.
//!
//! Right now the focus is on packaging together an owned string and
//! an iterator that borrows from the string to split it.
//!
//! This is an initial, work-in-progress release.

// Still needed on 2018 edition for rental crate:
#[cfg(feature = "into")]
#[macro_use]
extern crate rental;

#[macro_use]
mod internal_macros;

mod utf8;

mod split_end;
pub use split_end::*;

#[cfg(feature = "into")]
mod into_split;
#[cfg(feature = "into")]
pub use into_split::*;

#[cfg(feature = "regex")]
mod re;
#[cfg(feature = "regex")]
pub use re::IntoRegex;

// Dummy definitions for when `regex` is turned off.
#[cfg(not(feature = "regex"))]
mod re {
    pub type Regex = ();
    pub type Split<'r, 'b> = (&'r (), &'b ());
}

