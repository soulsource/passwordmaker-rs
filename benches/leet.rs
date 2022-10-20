mod mock_hashers;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use passwordmaker_rs::{HashAlgorithm, LeetLevel};
use mock_hashers::Pwm;

fn criterion_bench_16bytes_post_leet(c: &mut Criterion) {
    let pwm = Pwm::new(
        HashAlgorithm::Md5, 
        passwordmaker_rs::UseLeetWhenGenerating::After { level: LeetLevel::Six },
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789`~!@#$%^&*()_-+={}|[]\\:\";'<>?,./",
        "",
        "",
        150,
        "",
        ""
    ).unwrap();
    c.bench_function("16 bytes with post_leet", |b| b.iter(|| {
        pwm.generate(
            black_box("This is a long string. With many, many characters. For no particular reason.".to_owned()),
            black_box("And another relatively long string for no reason other than it being long.".to_owned())
        )
    }));
}

fn criterion_bench_16bytes_pre_leet(c: &mut Criterion) {
    let pwm = Pwm::new(
        HashAlgorithm::Md5, 
        passwordmaker_rs::UseLeetWhenGenerating::Before { level: LeetLevel::Six },
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789`~!@#$%^&*()_-+={}|[]\\:\";'<>?,./",
        "",
        "",
        150,
        "",
        ""
    ).unwrap();
    c.bench_function("16 bytes with pre_leet", |b| b.iter(|| {
        pwm.generate(
            black_box("This is a long string. With many, many characters. For no particular reason.".to_owned()),
            black_box("And another relatively long string for no reason other than it being long.".to_owned())
        )
    }));
}

criterion_group!(benches,
    criterion_bench_16bytes_post_leet,
    criterion_bench_16bytes_pre_leet
);
criterion_main!(benches);
