#![allow(unused)]

use std::{iter, str};

pub fn char_boundaries<'a>(s: &'a str)
    -> impl DoubleEndedIterator<Item = usize> + 'a {
    s.char_indices().map(|p| p.0).chain(iter::once(s.len()))
}

pub struct Utf8Boundaries<'a> {
    bytes:        Option<&'a [u8]>,
    left_offset:  usize,
    right_offset: usize,
}

impl<'a> Utf8Boundaries<'a> {
    pub fn new(bytes: &'a [u8]) -> Utf8Boundaries<'a> {
        Utf8Boundaries {
            bytes: Some(bytes),
            left_offset: 0,
            right_offset: 0,
        }
    }
}

impl<'a> Iterator for Utf8Boundaries<'a> {
    type Item = Result<usize, str::Utf8Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let bytes = self.bytes.take()?;

        u8_find_first(bytes).map(|option|
            option.map(|index| {
                self.left_offset += index;
                self.bytes = Some(&bytes[index..]);
                self.left_offset
            }))
    }
}

impl<'a> DoubleEndedIterator for Utf8Boundaries<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let bytes = self.bytes.take()?;

        u8_find_last(bytes).map(|option|
            option.map(|index| {
                self.right_offset += index;
                self.bytes = Some(&bytes[.. bytes.len() - index]);
                self.right_offset
            }))
    }
}

fn u8_find_first(bytes: &[u8]) -> Option<Result<usize, str::Utf8Error>> {
    let attempt = u8_find_first_helper(bytes)?;

    if let Ok(i) = attempt {
        return Some(Ok(i));
    }

    for i in 1 ..= 4 {
        if i > bytes.len() {
            break;
        }

        if let Ok(i) = u8_find_first_helper(&bytes[..i])? {
            return Some(Ok(i));
        }
    }

    Some(attempt)
}

fn u8_find_last(bytes: &[u8]) -> Option<Result<usize, str::Utf8Error>> {
    let attempt = u8_find_last_helper(bytes)?;

    if let Ok(i) = attempt {
        return Some(Ok(i));
    }

    for i in 1 ..= 4 {
        if i > bytes.len() {
            break;
        }

        if let Ok(i) = u8_find_first_helper(&bytes[i..])? {
            return Some(Ok(i));
        }
    }

    Some(attempt)
}

fn u8_find_first_helper(bytes: &[u8]) -> Option<Result<usize, str::Utf8Error>> {
    transpose(str::from_utf8(bytes).map(|s| char_boundaries(s).nth(1)))
}

fn u8_find_last_helper(bytes: &[u8]) -> Option<Result<usize, str::Utf8Error>> {
    transpose(str::from_utf8(bytes).map(|s| char_boundaries(s).rev().nth(1)))
}

fn transpose<T, E>(result: Result<Option<T>, E>) -> Option<Result<T, E>> {
    match result {
        Ok(o) => o.map(Ok),
        Err(e) => Some(Err(e)),
    }
}
