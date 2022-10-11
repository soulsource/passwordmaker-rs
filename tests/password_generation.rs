use passwordmaker_rs::{PasswordMaker, Hasher, HasherList, HashAlgorithm, LeetLevel};
use digest::Digest;
use md4;
use md5;
use sha1;
use sha2;
use ripemd;

struct Md4;
struct Md5;
struct Sha1;
struct Sha256;
struct RipeMD160;
impl Hasher for Md4{
    type Output = [u8;16];
    fn hash(data : &[u8]) -> Self::Output {
        md4::Md4::digest(data).into()
    }
}
impl Hasher for Md5{
    type Output = [u8;16];
    fn hash(data : &[u8]) -> Self::Output {
        md5::Md5::digest(data).into()
    }
}
impl Hasher for Sha1{
    type Output = [u8;20];
    fn hash(data : &[u8]) -> Self::Output {
        sha1::Sha1::digest(data).into()
    }
}
impl Hasher for Sha256{
    type Output = [u8;32];
    fn hash(data : &[u8]) -> Self::Output {
        sha2::Sha256::digest(data).into()
    }
}
impl Hasher for RipeMD160{
    type Output = [u8;20];
    fn hash(data : &[u8]) -> Self::Output {
        ripemd::Ripemd160::digest(data).into()
    }
}

impl passwordmaker_rs::Md4 for Md4{}
impl passwordmaker_rs::Md5 for Md5{}
impl passwordmaker_rs::Sha1 for Sha1{}
impl passwordmaker_rs::Sha256 for Sha256{}
impl passwordmaker_rs::Ripemd160 for RipeMD160{}

struct Hashes{}
impl HasherList for Hashes {
    type MD4 = Md4;
    type MD5 = Md5;
    type SHA1 = Sha1;
    type SHA256 = Sha256;
    type RIPEMD160 = RipeMD160;
}

type Pwm<'a> = PasswordMaker<'a, Hashes>;

#[test]
fn default_settings() {
    let pwm = Pwm::new(
        HashAlgorithm::Md5, 
        passwordmaker_rs::UseLeetWhenGenerating::NotAtAll,
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789`~!@#$%^&*()_-+={}|[]\\:\";'<>?,./",
        "",
        "",
        8,
        "",
        ""
    ).unwrap();
    let result = pwm.generate(".abcdefghij".to_owned(), "1".to_owned()).unwrap();
    assert_eq!(result, "J3>'1F\"/");
}

#[test]
fn v06_compatibility_leading_zeros() {
    let pwm = Pwm::new(
        HashAlgorithm::Md5Version06, 
        passwordmaker_rs::UseLeetWhenGenerating::NotAtAll,
        "whatevr",
        "",
        "",
        8,
        "",
        ""
    ).unwrap();
    let result = pwm.generate("01".to_owned(), "a".to_owned()).unwrap();
    assert_eq!(result, "00d2a735");
}

#[test]
fn regular_md5_no_leading_zeros() {
    let pwm = Pwm::new(
        HashAlgorithm::Md5, 
        passwordmaker_rs::UseLeetWhenGenerating::NotAtAll,
        "0123456789abcdef",
        "",
        "",
        8,
        "",
        ""
    ).unwrap();
    let result = pwm.generate("01".to_owned(), "a".to_owned()).unwrap();
    assert_eq!(result, "d2a73551");
}

/// Tests compatibility with an edge case in the l33t generation of PasswordMaker Pro JavaScript Edition: Word-Final-Sigma.
#[test]
fn word_final_sigma_post_leet() {
    let pwm = Pwm::new(
        HashAlgorithm::Md4, 
        passwordmaker_rs::UseLeetWhenGenerating::After { level: LeetLevel::One },
        "ΣΔΠΖ",
        "",
        "",
        64,
        "",
        ""
    ).unwrap();
    let result = pwm.generate("123456".to_owned(), "password".to_owned()).unwrap();
    assert_eq!(result, "ζδζσσπσζδδσδπζδδδπσπζπζδδζζππσζσσζδπδσζπζππδσπσζζπσζσδπζσζπδσςπδ"); //mind the lunate sigma at character position 61.
}

#[test]
fn hmac_with_upper_bytes() {
    let pwm = Pwm::new(
        HashAlgorithm::HmacRipemd160, 
        passwordmaker_rs::UseLeetWhenGenerating::NotAtAll,
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789`~!@#$%^&*()_-+={}|[]\\:\";'<>?,./",
        "",
        "",
        41,
        "",
        ""
    ).unwrap();
    let result = pwm.generate("€äß".to_owned(), "password".to_owned()).unwrap();
    assert_eq!(result, "CX'!aI7J+\\.x?:ua'vtaj~c_PBbfATer1tstX_n<}");
}

#[test]
fn v06_yeet_bytes() {
    let pwm = Pwm::new(
        HashAlgorithm::Md5Version06, 
        passwordmaker_rs::UseLeetWhenGenerating::NotAtAll,
        "notused",
        "",
        "",
        47,
        "",
        ""
    ).unwrap();
    let result = pwm.generate("€äß".to_owned(), "password".to_owned()).unwrap();
    assert_eq!(result, "ea552be82dc75c12e6e9d9f30e643e63eeba34536077ce3");
}

#[test]
fn v06_yeet_bytes_hmac() {
    let pwm = Pwm::new(
        HashAlgorithm::HmacMd5Version06, 
        passwordmaker_rs::UseLeetWhenGenerating::NotAtAll,
        "notused",
        "",
        "",
        47,
        "",
        ""
    ).unwrap();
    let result = pwm.generate("€äß".to_owned(), "password".to_owned()).unwrap();
    assert_eq!(result, "28e1392052364d34c7e42e2711ccdd62c67a0a30dbf568a");
}