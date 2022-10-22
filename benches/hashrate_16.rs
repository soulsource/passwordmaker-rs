mod mock_hashers;

use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use passwordmaker_rs::HashAlgorithm;
use mock_hashers::Pwm;

fn criterion_bench_16bytes_typical(c: &mut Criterion) {
    let pwm = Pwm::new(
        HashAlgorithm::Md5, 
        passwordmaker_rs::UseLeetWhenGenerating::NotAtAll,
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789`~!@#$%^&*()_-+={}|[]\\:\";'<>?,./",
        "",
        "",
        12,
        "",
        ""
    ).unwrap();
    c.bench_function("16 bytes typical", |b| b.iter(|| {
        pwm.generate(
            black_box("This is a long string. With many, many characters. For no particular reason.".to_owned()),
            black_box("And another relatively long string for no reason other than it being long.".to_owned())
        )
    }));
}

fn criterion_bench_16bytes_full_divide(c: &mut Criterion) {
    let pwm = Pwm::new(
        HashAlgorithm::Md5, 
        passwordmaker_rs::UseLeetWhenGenerating::NotAtAll,
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789`~!@#$%^&*()_-+={}|[]\\:\";'<>?,./",
        "",
        "",
        20,
        "",
        ""
    ).unwrap();
    c.bench_function("16 bytes full divide", |b| b.iter(|| {
        pwm.generate(
            black_box("This is a long string. With many, many characters. For no particular reason.".to_owned()),
            black_box("And another relatively long string for no reason other than it being long.".to_owned())
        )
    }));
}

fn criterion_bench_16bytes_worst_case(c: &mut Criterion) {
    let pwm = Pwm::new(
        HashAlgorithm::Md5, 
        passwordmaker_rs::UseLeetWhenGenerating::NotAtAll,
        "XY",
        "",
        "",
        128,
        "",
        ""
    ).unwrap();
    c.bench_function("16 bytes worst case", |b| b.iter(|| {
        pwm.generate(
            black_box("This is a long string. With many, many characters. For no particular reason.".to_owned()),
            black_box("And another relatively long string for no reason other than it being long.".to_owned())
        )
    }));
}

criterion_group!(name = benches;
    // This can be any expression that returns a `Criterion` object.
    config = Criterion::default().significance_level(0.02).sample_size(500).measurement_time(Duration::from_secs(10));
    targets = criterion_bench_16bytes_typical,
    criterion_bench_16bytes_full_divide,
    criterion_bench_16bytes_worst_case,
);
criterion_main!(benches);
