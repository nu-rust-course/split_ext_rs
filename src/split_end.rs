use std::str;

use super::utf8::char_boundaries;

pub trait SplitEnd: Sized {
    type Item: Sized;

    fn split_first(self) -> Option<(Self::Item, Self)>;

    fn split_last(self) -> Option<(Self::Item, Self)>;

    fn try_split_first_n(self, n: usize) -> Option<(Self, Self)>;

    fn try_split_last_n(self, n: usize) -> Option<(Self, Self)>;

    fn split_first_n(self, n: usize) -> (Self, Self) {
        self.try_split_first_n(n)
            .expect("SplitEnd::split_first_n: out of bounds")
    }

    fn split_last_n(self, n: usize) -> (Self, Self) {
        self.try_split_last_n(n)
            .expect("SplitEnd::split_last_n: out of bounds")
    }
}

impl SplitEnd for &str {
    type Item = char;

    fn split_first(self) -> Option<(Self::Item, Self)> {
        self.chars().split_first().map(|(c, i)| (c, i.as_str()))
    }

    fn split_last(self) -> Option<(Self::Item, Self)> {
        self.chars().split_last().map(|(c, i)| (c, i.as_str()))
    }

    fn try_split_first_n(self, n: usize) -> Option<(Self, Self)> {
        char_boundaries(self)
            .nth(n)
            .map(|index| self.split_at(index))
    }

    fn try_split_last_n(self, n: usize) -> Option<(Self, Self)> {
        char_boundaries(self)
            .rev()
            .nth(n)
            .map(|index| flip(self.split_at(index)))
    }
}

impl<'a> SplitEnd for str::Chars<'a> {
    type Item = char;

    fn split_first(mut self) -> Option<(Self::Item, Self)> {
        self.next().map(|c| (c, self))
    }

    fn split_last(mut self) -> Option<(Self::Item, Self)> {
        self.next_back().map(|c| (c, self))
    }

    fn try_split_first_n(self, n: usize) -> Option<(Self, Self)> {
        self.as_str().try_split_first_n(n)
            .map(|(front, back)| (front.chars(), back.chars()))
    }

    fn try_split_last_n(self, n: usize) -> Option<(Self, Self)> {
        self.as_str().try_split_last_n(n)
            .map(|(front, back)| (front.chars(), back.chars()))
    }
}

impl<'a, T> SplitEnd for &'a [T] {
    type Item = &'a T;

    fn split_first(self) -> Option<(Self::Item, Self)> {
        <[T]>::split_first(self)
    }

    fn split_last(self) -> Option<(Self::Item, Self)> {
        <[T]>::split_last(self)
    }

    fn try_split_first_n(self, n: usize) -> Option<(Self, Self)> {
        if_opt!{ n <= self.len(), self.split_first_n(n) }
    }

    fn try_split_last_n(self, n: usize) -> Option<(Self, Self)> {
        if_opt!{ n <= self.len(), self.split_last_n(n) }
    }

    fn split_first_n(self, n: usize) -> (Self, Self) {
        self.split_at(n)
    }

    fn split_last_n(self, n: usize) -> (Self, Self) {
        flip(self.split_at(self.len() - n))
    }
}

impl<'a, T> SplitEnd for &'a mut [T] {
    type Item = &'a mut T;

    fn split_first(self) -> Option<(Self::Item, Self)> {
        self.split_first_mut()
    }

    fn split_last(self) -> Option<(Self::Item, Self)> {
        self.split_last_mut()
    }

    fn try_split_first_n(self, n: usize) -> Option<(Self, Self)> {
        if_opt!{ n <= self.len(), self.split_first_n(n) }
    }

    fn try_split_last_n(self, n: usize) -> Option<(Self, Self)> {
        if_opt!{ n <= self.len(), self.split_last_n(n) }
    }

    fn split_first_n(self, n: usize) -> (Self, Self) {
        self.split_at_mut(n)
    }

    fn split_last_n(self, n: usize) -> (Self, Self) {
        flip(self.split_at_mut(self.len() - n))
    }
}

//impl<'a> SplitEnd for &'a [u8] {
//    type Item = Result<(char, &'a [u8]), str::Utf8Error>;
//
//    fn split_first(self) -> Option<(Self::Item, Self)> {
//        unimplemented!() // <[T]>::split_first(self)
//    }
//
//    fn split_last(self) -> Option<(Self::Item, Self)> {
//        unimplemented!() // <[T]>::split_last(self)
//    }
//
//    fn try_split_first_n(self, n: usize) -> Option<(Self, Self)> {
//        unimplemented!() // if_opt!{ n <= self.len(), self.split_first_n(n) }
//    }
//
//    fn try_split_last_n(self, n: usize) -> Option<(Self, Self)> {
//        unimplemented!() // if_opt!{ n <= self.len(), self.split_last_n(n) }
//    }
//}

fn flip<T, U>((t, u): (T, U)) -> (U, T) {
    (u, t)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn str_split() {
        assert_eq!("hello".split_first(), Some(('h', "ello")));
        assert_eq!("h".split_first(), Some(('h', "")));
        assert_eq!("".split_first(), None);

        assert_eq!("hello".split_last(), Some(('o', "hell")));
        assert_eq!("h".split_last(), Some(('h', "")));
        assert_eq!("".split_last(), None);
    }

    #[test]
    fn str_split_multibyte() {
        assert_eq!("€6".split_first(), Some(('€', "6")));
        assert_eq!("6€".split_first(), Some(('6', "€")));
        assert_eq!("€".split_first(), Some(('€', "")));

        assert_eq!("€6".split_last(), Some(('6', "€")));
        assert_eq!("6€".split_last(), Some(('€', "6")));
        assert_eq!("€".split_last(), Some(('€', "")));
    }

    #[test]
    fn str_split_n() {
        assert_eq!( "bye".split_first_n(0), ("", "bye") );
        assert_eq!( "bye".split_first_n(1), ("b", "ye") );
        assert_eq!( "bye".split_first_n(2), ("by", "e") );
        assert_eq!( "bye".split_first_n(3), ("bye", "") );
        assert_eq!( "bye".try_split_first_n(4), None );

        assert_eq!( "bye".split_last_n(0), ("", "bye") );
        assert_eq!( "bye".split_last_n(1), ("e", "by") );
        assert_eq!( "bye".split_last_n(2), ("ye", "b") );
        assert_eq!( "bye".split_last_n(3), ("bye", "") );
        assert_eq!( "bye".try_split_last_n(4), None );
    
        assert_eq!( "".try_split_first_n(0), Some(("", "")) );
        assert_eq!( "".try_split_first_n(1), None );
    }

    #[test]
    fn str_split_n_all() {
        assert_all_splits("");
        assert_all_splits("hello");
        assert_all_splits("€ह 𐍈");
    }

    fn assert_all_splits(s: &str) {
        let len_c = s.chars().count();

        for (ic, ib) in char_boundaries(s).enumerate() {
            let front = &s[.. ib];
            let back = &s[ib ..];
            assert_eq!( s.try_split_first_n(ic), Some((front, back)) );
            assert_eq!( s.try_split_last_n(len_c - ic), Some((back, front)) );
        }

        assert_eq!( s.try_split_first_n(len_c + 1), None );
        assert_eq!( s.try_split_last_n(len_c + 1), None );
    }

    #[test]
    #[should_panic]
    fn split_str_n_oob() {
        "hello".split_first_n(12);
    }
}
