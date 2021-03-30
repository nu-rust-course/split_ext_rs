use std::fmt;
use stable_deref_trait::StableDeref;

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

/// Allows passing either a borrowed or an owned `Regex`
/// to a method that needs to own a stable pointer to it.
pub trait IntoRegex: Sized {
    type StableRegex: StableDeref<Target = regex::Regex>;

    fn into_stable_regex(self) -> Self::StableRegex;
}

impl IntoRegex for Regex {
    type StableRegex = Box<Regex>;

    fn into_stable_regex(self) -> Self::StableRegex {
        self.into()
    }
}

macro_rules! impl_into_regex {
    (impl<$a:lifetime> IntoRegex<$b:lifetime> for $t:ty) => {
        impl<$a> IntoRegex for $t {
            type StableRegex = Self;

            fn into_stable_regex(self) -> Self::StableRegex {
                self
            }
        }
    };

    (<$a:lifetime> $t:ty) => {
        impl_into_regex!(impl<$a> IntoRegex<$a> for $t);
    };

    ($t:ty) => {
        impl_into_regex!(impl<'a> IntoRegex<'static> for $t);
    };
}

impl_into_regex!(<'a> &'a Regex);
impl_into_regex!(Box<Regex>);
impl_into_regex!(std::rc::Rc<Regex>);
impl_into_regex!(std::sync::Arc<Regex>);

