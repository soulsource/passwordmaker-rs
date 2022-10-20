mod mock_hashers;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use passwordmaker_rs::HashAlgorithm;
use mock_hashers::Pwm;

fn criterion_bench_32bytes_typical(c: &mut Criterion) {
    let pwm = Pwm::new(
        HashAlgorithm::Sha256, 
        passwordmaker_rs::UseLeetWhenGenerating::NotAtAll,
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789`~!@#$%^&*()_-+={}|[]\\:\";'<>?,./",
        "",
        "",
        12,
        "",
        ""
    ).unwrap();
    c.bench_function("32 bytes typical", |b| b.iter(|| {
        pwm.generate(
            black_box("This is a long string. With many, many characters. For no particular reason.".to_owned()),
            black_box("And another relatively long string for no reason other than it being long.".to_owned())
        )
    }));
}

fn criterion_bench_32bytes_full_divide(c: &mut Criterion) {
    let pwm = Pwm::new(
        HashAlgorithm::Sha256, 
        passwordmaker_rs::UseLeetWhenGenerating::NotAtAll,
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789`~!@#$%^&*()_-+={}|[]\\:\";'<>?,./",
        "",
        "",
        40,
        "",
        ""
    ).unwrap();
    c.bench_function("32 bytes full divide", |b| b.iter(|| {
        pwm.generate(
            black_box("This is a long string. With many, many characters. For no particular reason.".to_owned()),
            black_box("And another relatively long string for no reason other than it being long.".to_owned())
        )
    }));
}

fn criterion_bench_32bytes_worst_case(c: &mut Criterion) {
    let pwm = Pwm::new(
        HashAlgorithm::Sha256, 
        passwordmaker_rs::UseLeetWhenGenerating::NotAtAll,
        "XY",
        "",
        "",
        256,
        "",
        ""
    ).unwrap();
    c.bench_function("32 bytes worst case", |b| b.iter(|| {
        pwm.generate(
            black_box("This is a long string. With many, many characters. For no particular reason.".to_owned()),
            black_box("And another relatively long string for no reason other than it being long.".to_owned())
        )
    }));
}

criterion_group!(benches,
    criterion_bench_32bytes_typical,
    criterion_bench_32bytes_full_divide,
    criterion_bench_32bytes_worst_case,
);
criterion_main!(benches);
