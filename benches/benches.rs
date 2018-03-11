#[macro_use]
extern crate bencher;

use bencher::Bencher;
use std::borrow::Cow;

fn bench_cowstring(b: &mut Bencher) {
    b.iter(|| {
    let fizz = Into::<Cow<_>>::into("Fizz").into_owned();
    let buzz = Into::<Cow<_>>::into("Buzz");
    Into::<Cow<_>>::into(fizz + &*buzz)
    });
}

fn bench_cowstring_ms2ger(b: &mut Bencher) {
    b.iter( || {  
        let mut fizz = Into::<Cow<_>>::into("Fizz").into_owned();
        let buzz = Into::<Cow<_>>::into("Buzz");
        fizz.push_str(&*buzz);
        Into::<Cow<_>>::into(fizz);
    }
    );
}

benchmark_group!(benches, bench_cowstring, bench_cowstring_ms2ger);
benchmark_main!(benches);
