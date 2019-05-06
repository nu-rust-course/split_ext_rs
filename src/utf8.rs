#![allow(unused)]

use std::{iter, str};

pub fn char_boundaries<'a>(s: &'a str)
    -> impl DoubleEndedIterator<Item = usize> + 'a {
    s.char_indices().map(|p| p.0).chain(iter::once(s.len()))
}

pub struct Utf8Boundaries<'a> {
    slice:        Option<&'a [u8]>,
    left_offset:  usize,
    right_offset: usize,
}

impl<'a> Utf8Boundaries<'a> {
    pub fn new(slice: &'a [u8]) -> Utf8Boundaries<'a> {
        Utf8Boundaries {
            slice: Some(slice),
            left_offset: 0,
            right_offset: 0,
        }
    }
}

impl<'a> Iterator for Utf8Boundaries<'a> {
    type Item = Result<usize, str::Utf8Error>;

    fn next(&mut self) -> Option<Self::Item> {
        let slice = self.slice.take()?;

        u8_find_first(slice).map(|option|
            option.map(|index| {
                self.left_offset += index;
                self.slice = Some(&slice[index..]);
                self.left_offset
            }))
    }
}

impl<'a> DoubleEndedIterator for Utf8Boundaries<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let slice = self.slice.take()?;

        u8_find_last(slice).map(|option|
            option.map(|index| {
                self.right_offset += index;
                self.slice = Some(&slice[.. slice.len() - index]);
                self.right_offset
            }))
    }
}

fn u8_find_first(slice: &[u8]) -> Option<Result<usize, str::Utf8Error>> {
    let attempt = u8_find_first_helper(slice)?;

    if let Ok(i) = attempt {
        return Some(Ok(i));
    }

    for i in 1 ..= 4 {
        if i > slice.len() {
            break;
        }

        if let Ok(i) = u8_find_first_helper(&slice[..i])? {
            return Some(Ok(i));
        }
    }

    Some(attempt)
}

fn u8_find_last(slice: &[u8]) -> Option<Result<usize, str::Utf8Error>> {
    let attempt = u8_find_last_helper(slice)?;

    if let Ok(i) = attempt {
        return Some(Ok(i));
    }

    for i in 1 ..= 4 {
        if i > slice.len() {
            break;
        }

        if let Ok(i) = u8_find_first_helper(&slice[i..])? {
            return Some(Ok(i));
        }
    }

    Some(attempt)
}

fn u8_find_first_helper(slice: &[u8]) -> Option<Result<usize, str::Utf8Error>> {
    str::from_utf8(slice).map(|s| char_boundaries(s).nth(1)).transpose()
}

fn u8_find_last_helper(slice: &[u8]) -> Option<Result<usize, str::Utf8Error>> {
    str::from_utf8(slice).map(|s| char_boundaries(s).rev().nth(1)).transpose()
}

