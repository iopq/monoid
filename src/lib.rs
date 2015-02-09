extern crate test;

use std::string::CowString;
use std::borrow::IntoCow;

pub trait Monoid {
    // don't have associated values yet, so use a nullary function
    fn id() -> Self;
    // an associative binary operation
    // this version consumes arguments
    // a non-consuming version might be possible
    fn op(self, other: Self) -> Self;
}

impl<'a> Monoid for CowString<'a> {
    fn id() -> CowString<'a> { "".into_cow() }
    fn op(self, other: CowString<'a>) -> CowString<'a> {
        let mut owned = self.into_owned();
        owned.push_str(&*other);
        owned.into_cow()
    }
}

// Strings are a Monoid over concatenation
impl Monoid for String {
    fn id() -> String { "".to_string() } // identity is empty string
    fn op(self, other: String) -> String {
        self + other.as_slice()
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
    use std::string::CowString;
    use std::borrow::IntoCow;
    
    #[test]
    fn string_append() {
        assert_eq!("abcdef".to_string(), "abc".to_string().op("def".to_string()));
    }
    
    fn cowstring_append() {
        assert_eq!("abcdef".into_cow(), "abc".into_cow().op("def".into_cow()));
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
    
    fn both() {
        assert_eq!(Some("oneone".to_string()), Some("one".to_string()).op(Some("one".to_string())));
    }
}

#[bench]
fn bench_cowstring(b: &mut test::Bencher) {
    b.iter(|| {
        let mut fizz = "Fizz".into_cow().into_owned();
        let mut buzz = "Buzz".into_cow();
        (fizz + &*buzz).into_cow()
    });
}

#[bench]
fn bench_cowstring_ms2ger(b: &mut test::Bencher) {
    b.iter( || {  
            let mut fizz = "Fizz".into_cow().into_owned();
            let mut buzz = "Buzz".into_cow();
            fizz.push_str(&*buzz);
            fizz.into_cow();
        }
    );
}