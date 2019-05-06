#![doc(html_root_url = "https://docs.rs/split_ext/0.1.0")]
//! Extension traits for splitting.
//!
//! Right now the focus is on packaging together an owned string and
//! an iterator that borrows from the string to split it.
//!
//! This is an initial, work-in-progress release.

// Still needed on 2018 edition for rental crate:
#[macro_use]
extern crate rental;

use std::ops;

use rental::rental;
use stable_deref_trait::StableDeref;

/// Trait for owning-splitter methods. These methods work on `String`,
/// `Box<[str]>`, `Rc<[str]>`, etc.
pub trait IntoSplitExt: ops::Deref<Target = str> + StableDeref + Sized {
    fn into_split_whitespace(
        self)
        -> IntoSplitWhitespace<Self> {

        IntoSplitWhitespace {
            inner: inner::Whitespace::new(self, str::split_whitespace),
        }
    }

    fn into_split_whitespace_map<F, R>(
        self,
        fun: F)
        -> IntoSplitWhitespaceMap<Self, F>
    where
        F: FnMut(&str) -> R {

        IntoSplitWhitespaceMap {
            inner: self.into_split_whitespace().inner,
            fun
        }
    }

    fn into_split_whitespace_and_then<F, I>(
        self,
        fun: F)
        -> IntoSplitWhitespaceAndThen<Self, F, I>
    where
        F: FnMut(&str) -> I,
        I: IntoIterator, {

        IntoSplitWhitespaceAndThen {
            inner: self.into_split_whitespace().inner,
            fun,
            rest: None,
        }
    }

    #[cfg(feature = "regex")]
    fn into_split_regex(self, regex: regex::Regex) -> IntoSplitRegex<Self> {
        IntoSplitRegex {
            inner: inner::Regex::new(
                self,
                move |_| Box::new(regex),
                |r, s| regex::Split(r.split(s)))
        }
    }

    #[cfg(feature = "regex")]
    fn into_split_regex_map<F, R>(self, regex: regex::Regex, fun: F)
                                  -> IntoSplitRegexMap<Self, F>
    where
        F: FnMut(&str) -> R {

        IntoSplitRegexMap {
            inner: self.into_split_regex(regex).inner,
            fun
        }
    }

    #[cfg(feature = "regex")]
    fn into_split_regex_and_then<F, I>(self, regex: regex::Regex, fun: F)
                                       -> IntoSplitRegexAndThen<Self, F, I>
    where
        F: FnMut(&str) -> I,
        I: IntoIterator {

        IntoSplitRegexAndThen {
            inner: self.into_split_regex(regex).inner,
            fun,
            rest:  None,
        }
    }

    #[cfg(feature = "regex")]
    fn into_split_regex_ref(self, regex: &regex::Regex) -> IntoSplitRegexRef<Self> {
        IntoSplitRegexRef {
            inner: inner::RegexRef::new(
                self,
                move |s| regex::Split(regex.split(s)))
        }
    }

    #[cfg(feature = "regex")]
    fn into_split_regex_ref_map<F, R>(self, regex: &regex::Regex, fun: F)
                                      -> IntoSplitRegexRefMap<Self, F>
    where
        F: FnMut(&str) -> R {

        IntoSplitRegexRefMap {
            inner: self.into_split_regex_ref(regex).inner,
            fun,
        }
    }

    #[cfg(feature = "regex")]
    fn into_split_regex_ref_and_then<F, I>(self, regex: &regex::Regex, fun: F)
                                           -> IntoSplitRegexRefAndThen<Self, F, I>
    where
        F: FnMut(&str) -> I,
        I: IntoIterator, {

        IntoSplitRegexRefAndThen {
            inner: self.into_split_regex_ref(regex).inner,
            fun,
            rest:  None,
        }
    }
}

impl<T: ops::Deref<Target = str> + StableDeref + Sized> IntoSplitExt for T { }

#[cfg(feature = "regex")]
mod regex {
    use std::fmt;

    pub type Regex = regex::Regex;

    // Wrap `regex::Split` in order to impl `Debug` for it.
    pub struct Split<'r, 'b>(pub regex::Split<'r, 'b>);

    impl<'r, 'b> Iterator for Split<'r, 'b> {
        type Item = &'b str;

        fn next(&mut self) -> Option<Self::Item> {
            self.0.next()
        }
    }

    impl<'r, 'b> fmt::Debug for Split<'r, 'b> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "regex::Split(...)")
        }
    }
}

// Dummy definitions for when `regex` is turned off.
#[cfg(not(feature = "regex"))]
mod regex {
    pub type Regex = ();
    pub type Split<'r, 'b> = (&'r (), &'b ());
}

rental! {
    #[allow(clippy::useless_transmute)]
    mod inner {
        use std::str;
        use super::{IntoSplitExt, regex};

        #[rental(debug)]
        pub struct Whitespace<S: IntoSplitExt> {
            base:     S,
            splitter: str::SplitWhitespace<'base>,
        }

        #[rental(debug)]
        pub struct Regex<S: IntoSplitExt> {
            base:     S,
            regex:    Box<regex::Regex>,
            splitter: regex::Split<'regex, 'base>,
        }

        #[rental(debug)]
        pub struct RegexRef<'r, S: IntoSplitExt> {
            base:     S,
            splitter: regex::Split<'r, 'base>,
        }
    }
}

impl<S: IntoSplitExt> inner::Whitespace<S> {
    fn next_map<R, F: FnOnce(&str) -> R>(&mut self, fun: F) -> Option<R> {
        self.rent_mut(|iter| iter.next().map(fun))
    }
}

impl<S: IntoSplitExt> inner::Regex<S> {
    fn next_map<R, F: FnOnce(&str) -> R>(&mut self, fun: F) -> Option<R> {
        self.rent_mut(|iter| iter.next().map(fun))
    }
}

impl<'a, S: IntoSplitExt> inner::RegexRef<'a, S> {
    fn next_map<R: 'a, F: FnOnce(&str) -> R>(&mut self, fun: F) -> Option<R> {
        self.rent_mut(|iter| iter.next().map(fun))
    }
}

#[derive(Debug)]
pub struct IntoSplitWhitespace<S: IntoSplitExt> {
    inner: inner::Whitespace<S>,
}

impl<S: IntoSplitExt> Iterator for IntoSplitWhitespace<S> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next_map(str::to_owned)
    }
}

#[derive(Debug)]
pub struct IntoSplitWhitespaceMap<S: IntoSplitExt, F> {
    inner: inner::Whitespace<S>,
    fun:   F,
}

impl<S, F, R> Iterator for IntoSplitWhitespaceMap<S, F>
where
    S: IntoSplitExt,
    F: FnMut(&str) -> R,
{
    type Item = R;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next_map(&mut self.fun)
    }
}

#[derive(Debug)]
pub struct IntoSplitWhitespaceAndThen<S, F, I>
where
    S: IntoSplitExt,
    F: FnMut(&str) -> I,
    I: IntoIterator, {

    inner: inner::Whitespace<S>,
    fun:   F,
    rest:  Option<I::IntoIter>,
}

impl<S, F, I> Iterator for IntoSplitWhitespaceAndThen<S, F, I>
where
    S: IntoSplitExt,
    F: FnMut(&str) -> I,
    I: IntoIterator, {

    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        while let Some(rest) = self.rest.as_mut() {
            if let Some(result) = rest.next() {
                return Some(result);
            }

            self.rest = self.inner
                .next_map(&mut self.fun)
                .map(I::into_iter);
        }

        None
    }
}

#[cfg(feature = "regex")]
pub use regex_only::*;

#[cfg(feature = "regex")]
mod regex_only {
    use super::*;

    #[derive(Debug)]
    pub struct IntoSplitRegex<S>
    where
        S: IntoSplitExt {

        pub (super) inner: inner::Regex<S>,
    }

    #[derive(Debug)]
    pub struct IntoSplitRegexRef<'a, S>
    where
        S: IntoSplitExt {

        pub (super) inner: inner::RegexRef<'a, S>,
    }

    #[derive(Debug)]
    pub struct IntoSplitRegexMap<S, F>
    where
        S: IntoSplitExt {

        pub (super) inner: inner::Regex<S>,
        pub (super) fun:   F,
    }

    #[derive(Debug)]
    pub struct IntoSplitRegexRefMap<'a, S, F>
    where
        S: IntoSplitExt {

        pub (super) inner: inner::RegexRef<'a, S>,
        pub (super) fun:   F,
    }

    #[derive(Debug)]
    pub struct IntoSplitRegexAndThen<S, F, I>
    where
        S: IntoSplitExt,
        I: IntoIterator, {

        pub (super) inner: inner::Regex<S>,
        pub (super) fun:   F,
        pub (super) rest:  Option<I::IntoIter>,
    }

    #[derive(Debug)]
    pub struct IntoSplitRegexRefAndThen<'a, S, F, I>
    where
        S: IntoSplitExt,
        I: IntoIterator, {

        pub (super) inner: inner::RegexRef<'a, S>,
        pub (super) fun:   F,
        pub (super) rest:  Option<I::IntoIter>,
    }


    impl<S: IntoSplitExt> Iterator for IntoSplitRegex<S> {
        type Item = String;

        fn next(&mut self) -> Option<Self::Item> {
            self.inner.next_map(str::to_owned)
        }
    }

    impl<'r, S: IntoSplitExt> Iterator for IntoSplitRegexRef<'r, S> {
        type Item = String;

        fn next(&mut self) -> Option<Self::Item> {
            self.inner.next_map(str::to_owned)
        }
    }

    impl<S, F, R> Iterator for IntoSplitRegexMap<S, F>
    where
        S: IntoSplitExt,
        F: FnMut(&str) -> R {

        type Item = R;

        fn next(&mut self) -> Option<Self::Item> {
            self.inner.next_map(&mut self.fun)
        }
    }

    impl<'r, S, F, R: 'r> Iterator for IntoSplitRegexRefMap<'r, S, F>
    where
        S: IntoSplitExt,
        F: FnMut(&str) -> R {

        type Item = R;

        fn next(&mut self) -> Option<Self::Item> {
            self.inner.next_map(&mut self.fun)
        }
    }

    impl<S, F, I> Iterator for IntoSplitRegexAndThen<S, F, I>
    where
        S: IntoSplitExt,
        F: FnMut(&str) -> I,
        I: IntoIterator {

        type Item = I::Item;

        fn next(&mut self) -> Option<Self::Item> {
            while let Some(rest) = self.rest.as_mut() {
                if let Some(result) = rest.next() {
                    return Some(result);
                }

                self.rest = self.inner
                    .next_map(&mut self.fun)
                    .map(I::into_iter);
            }

            None
        }
    }

    impl<'r, S, F, I: 'r> Iterator for IntoSplitRegexRefAndThen<'r, S, F, I>
    where
        S: IntoSplitExt,
        F: FnMut(&str) -> I,
        I: IntoIterator, {

        type Item = I::Item;

        fn next(&mut self) -> Option<Self::Item> {
            while let Some(rest) = self.rest.as_mut() {
                if let Some(result) = rest.next() {
                    return Some(result);
                }

                self.rest = self.inner
                                .next_map(&mut self.fun)
                                .map(I::into_iter);
            }

            None
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::io::{Read, BufRead, BufReader};
        use lazy_static::lazy_static;

        #[test]
        fn three_words() {
            assert_words(
                "one two three",
                &["one", "two", "three"]
            );
        }

        #[test]
        fn with_punctuation() {
            assert_words(
                "one--two-two /three",
                &["one", "two-two", "three"]
            );
        }

        #[test]
        fn multiple_lines() {
            assert_words(
                concat![
                "first line\n",
                "\n",
                "above line was blank!\n",
            ],
                &["first", "line", "above", "line", "was", "blank!"]
            );
        }

        fn assert_words(input: &str, expected: &[&str]) {
            assert_eq!( words(input.as_bytes()).collect::<Vec<_>>(),
                        ownv(expected) );
        }


        fn words(reader: impl Read) -> impl Iterator<Item = String> {
            lazy_static! {
            static ref RE: regex::Regex =
                regex::Regex::new("(?:--|/|[[:space:]])+").unwrap();
        }

            BufReader::new(reader).lines()
                                  .flat_map(|s| s.unwrap()
                                                 .into_split_regex_ref_map(&RE, trim_and_lowercase))
        }

        fn trim_and_lowercase(word: &str) -> String {
            word.trim().to_lowercase()
        }
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use super::*;

    #[test]
    fn split_boxed_str() {
        let s: Box<str> = "hello this is my string".into();

        assert_eq!(
            s.into_split_whitespace().collect::<Vec<_>>(),
            ownv(&["hello", "this", "is", "my", "string"])
        )
    }

    #[test]
    fn split_rc_str() {
        let s: Rc<str> = "hello this is my string".into();

        assert_eq!(
            s.into_split_whitespace().collect::<Vec<_>>(),
            ownv(&["hello", "this", "is", "my", "string"])
        )
    }
}

#[cfg(test)]
fn ownv<'a, T, I>(seq: I) -> Vec<T::Owned>
where T: ToOwned + ?Sized + 'a,
      I: IntoIterator<Item = &'a T>, {
    
    seq.into_iter().map(T::to_owned).collect()
}


