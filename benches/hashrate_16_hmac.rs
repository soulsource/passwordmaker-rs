mod mock_hashers;

use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use passwordmaker_rs::HashAlgorithm;
use mock_hashers::Pwm;

fn criterion_bench_16bytes_hmac_long_key(c: &mut Criterion) {
    let pwm = Pwm::new(
        HashAlgorithm::HmacMd5,
        passwordmaker_rs::UseLeetWhenGenerating::NotAtAll,
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789`~!@#$%^&*()_-+={}|[]\\:\";'<>?,./",
        "",
        "",
        12,
        "",
        ""
    ).unwrap();
    c.bench_function("16 bytes HMAC long key", |b| b.iter(|| {
        pwm.generate(
            black_box("This is a long string. With many, many characters. For no particular reason.".to_owned()),
            black_box("And another relatively long string for no reason other than it being long.".to_owned())
        )
    }));
}

fn criterion_bench_16bytes_hmac_short_key(c: &mut Criterion) {
    let pwm = Pwm::new(
        HashAlgorithm::HmacMd5,
        passwordmaker_rs::UseLeetWhenGenerating::NotAtAll,
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789`~!@#$%^&*()_-+={}|[]\\:\";'<>?,./",
        "",
        "",
        12,
        "",
        ""
    ).unwrap();
    c.bench_function("16 bytes HMAC short key", |b| b.iter(|| {
        pwm.generate(
            black_box("This is a long string. With many, many characters. For no particular reason.".to_owned()),
            black_box("Short Key".to_owned())
        )
    }));
}

criterion_group!(name = benches;
    // This can be any expression that returns a `Criterion` object.
    config = Criterion::default().significance_level(0.02).sample_size(500).measurement_time(Duration::from_secs(10));
    targets = criterion_bench_16bytes_hmac_long_key,
    criterion_bench_16bytes_hmac_short_key,
);
criterion_main!(benches);
