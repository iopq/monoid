use std::borrow::Cow;
use std::convert::{AsRef, Into};

pub trait Monoid {
    // don't have associated values yet, so use a nullary function
    fn id() -> Self;
    // an associative binary operation
    // this version consumes arguments
    // a non-consuming version might be possible
    fn op(self, other: Self) -> Self;
}

impl<'a> Monoid for Cow<'a, str> {
    fn id() -> Cow<'a, str> { "".into() }
    fn op(self, other: Cow<'a, str>) -> Cow<'a, str> {
        let mut owned = self.into_owned();
        owned.push_str(&*other);
        owned.into()
    }
}

// Strings are a Monoid over concatenation
impl Monoid for String {
    fn id() -> String { "".to_string() } // identity is empty string
    fn op(self, other: String) -> String {
        self + other.as_ref()
    }
}

// Options are Monoids if they contain Monoids
impl<A: Monoid> Monoid for Option<A> {
    fn id() -> Option<A> { None }
    fn op(self, other: Option<A>) -> Option<A> {
        match (self, other) {
             (None, b) => b,
             (a, None) => a,
             (Some(a), Some(b)) => Some(a.op(b)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Monoid;
    use std::borrow::Cow;
    use std::convert::Into;

    #[test]
    fn string_append() {
        assert_eq!("abcdef".to_string(), "abc".to_string().op("def".to_string()));
    }
    
    #[test]
    fn cowstring_append() {
        assert_eq!(Into::<Cow<_>>::into("abcdef"), Into::<Cow<_>>::into("abc").op(Into::<Cow<_>>::into("def")));
    }
    
    #[test]
    fn none() {
        assert_eq!(None::<String>, None::<String>.op(None::<String>));
    }
    
    #[test]
    fn left() {
        assert_eq!(Some("one".to_string()), Some("one".to_string()).op(None));
    }
    
    #[test]
    fn right() {
        assert_eq!(Some("one".to_string()), None.op(Some("one".to_string())));
    }
    
    #[test]
    fn both() {
        assert_eq!(Some("oneone".to_string()), Some("one".to_string()).op(Some("one".to_string())));
    }
}
