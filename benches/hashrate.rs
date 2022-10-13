use criterion::{black_box, criterion_group, criterion_main, Criterion};
use passwordmaker_rs::{PasswordMaker, Hasher, HasherList, HashAlgorithm, LeetLevel};

//We want to bench the surrounding string manipulation, not the hashers.
//For this reason, we fake them with a black_box.

struct MockMd4;
struct MockMd5;
struct MockSha1;
struct MockSha256;
struct MockRipeMD160;
impl Hasher for MockMd4{
    type Output = [u8;16];
    fn hash(_data : &[u8]) -> Self::Output {
        black_box([156u8,4u8,123u8,54u8,91u8,85u8,34u8,159u8,243u8,210u8,35u8,41u8,31u8,34u8,75u8,94u8])
    }
}
impl Hasher for MockMd5{
    type Output = [u8;16];
    fn hash(_data : &[u8]) -> Self::Output {
        black_box([156u8,4u8,123u8,54u8,91u8,85u8,34u8,159u8,243u8,210u8,35u8,41u8,31u8,34u8,75u8,94u8])
    }
}
impl Hasher for MockSha1{
    type Output = [u8;20];
    fn hash(_data : &[u8]) -> Self::Output {
        black_box([156u8,4u8,123u8,54u8,91u8,85u8,34u8,159u8,243u8,210u8,35u8,41u8,31u8,34u8,75u8,94u8,46,49,13,24])
    }
}
impl Hasher for MockSha256{
    type Output = [u8;32];
    fn hash(_data : &[u8]) -> Self::Output {
        black_box([156u8,4u8,123u8,54u8,91u8,85u8,34u8,159u8,243u8,210u8,35u8,41u8,31u8,34u8,75u8,94u8,156u8,4u8,123u8,54u8,91u8,85u8,34u8,159u8,243u8,210u8,35u8,41u8,31u8,34u8,75u8,94u8])
    }
}
impl Hasher for MockRipeMD160{
    type Output = [u8;20];
    fn hash(_data : &[u8]) -> Self::Output {
        black_box([156u8,4u8,123u8,54u8,91u8,85u8,34u8,159u8,243u8,210u8,35u8,41u8,31u8,34u8,75u8,94u8,46,49,13,24])
    }
}

impl passwordmaker_rs::Md4 for MockMd4{}
impl passwordmaker_rs::Md5 for MockMd5{}
impl passwordmaker_rs::Sha1 for MockSha1{}
impl passwordmaker_rs::Sha256 for MockSha256{}
impl passwordmaker_rs::Ripemd160 for MockRipeMD160{}

struct MockHashes{}
impl HasherList for MockHashes {
    type MD4 = MockMd4;
    type MD5 = MockMd5;
    type SHA1 = MockSha1;
    type SHA256 = MockSha256;
    type RIPEMD160 = MockRipeMD160;
}

type Pwm<'a> = PasswordMaker<'a, MockHashes>;

fn criterion_bench_32bit(c: &mut Criterion) {
    let pwm = Pwm::new(
        HashAlgorithm::Sha256, 
        passwordmaker_rs::UseLeetWhenGenerating::NotAtAll,
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789`~!@#$%^&*()_-+={}|[]\\:\";'<>?,./",
        "",
        "",
        150,
        "",
        ""
    ).unwrap();
    c.bench_function("32 bytes", |b| b.iter(|| {
        pwm.generate(
            black_box("This is a long string. With many, many characters. For no particular reason.".to_owned()),
            black_box("And another relatively long string for no reason other than it being long.".to_owned())
        )
    }));
}

fn criterion_bench_16bit(c: &mut Criterion) {
    let pwm = Pwm::new(
        HashAlgorithm::Md5, 
        passwordmaker_rs::UseLeetWhenGenerating::NotAtAll,
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789`~!@#$%^&*()_-+={}|[]\\:\";'<>?,./",
        "",
        "",
        150,
        "",
        ""
    ).unwrap();
    c.bench_function("16 bytes", |b| b.iter(|| {
        pwm.generate(
            black_box("This is a long string. With many, many characters. For no particular reason.".to_owned()),
            black_box("And another relatively long string for no reason other than it being long.".to_owned())
        )
    }));
}

fn criterion_bench_16bit_post_leet(c: &mut Criterion) {
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

fn criterion_bench_16bit_pre_leet(c: &mut Criterion) {
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

criterion_group!(benches, criterion_bench_32bit, criterion_bench_16bit, criterion_bench_16bit_post_leet, criterion_bench_16bit_pre_leet);
criterion_main!(benches);
