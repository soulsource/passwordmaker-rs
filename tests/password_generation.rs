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

#[test]
fn test_each_algo_md4(){
    let pwm = Pwm::new(
        HashAlgorithm::Md4, 
        passwordmaker_rs::UseLeetWhenGenerating::Before { level: LeetLevel::Nine },
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789",
        "max_mustermann",
        "modification",
        64,
        "pre",
        "suf"
    ).unwrap();
    let result = pwm.generate(
        ".0123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789öä@€Whatever".to_owned(), 
        "0123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789".to_owned()).unwrap();
    assert_eq!(result, "preBhaY7RkI3FU2Dd1gGbkHGXHcaS9Rla5yOyBsxtRhYjUV4CoEKST1N73Ipmsuf");
}
#[test]
fn test_each_algo_hmac_md4(){
    let pwm = Pwm::new(
        HashAlgorithm::HmacMd4, 
        passwordmaker_rs::UseLeetWhenGenerating::Before { level: LeetLevel::Nine },
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789",
        "max_mustermann",
        "modification",
        64,
        "pre",
        "suf"
    ).unwrap();
    let result = pwm.generate(
        ".0123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789öä@€Whatever".to_owned(), 
        "0123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789".to_owned()).unwrap();
    assert_eq!(result, "preCWxYmOtw9SouPQCHRRxLWODlFQ5LFitOpYMLHdnELniLHtQpdH5U2eOAOHsuf");
}

#[test]
fn test_each_algo_md5(){
    let pwm = Pwm::new(
        HashAlgorithm::Md5, 
        passwordmaker_rs::UseLeetWhenGenerating::Before { level: LeetLevel::Nine },
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789",
        "max_mustermann",
        "modification",
        64,
        "pre",
        "suf"
    ).unwrap();
    let result = pwm.generate(
        ".0123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789öä@€Whatever".to_owned(), 
        "0123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789".to_owned()).unwrap();
    assert_eq!(result, "preDPeKYAEZMwmB99f7i48vWApmx8ZLbz46s2nyG6KNO00G4nEElILxWAtGLGsuf");
}

#[test]
fn test_each_algo_hmac_md5(){
    let pwm = Pwm::new(
        HashAlgorithm::HmacMd5, 
        passwordmaker_rs::UseLeetWhenGenerating::Before { level: LeetLevel::Nine },
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789",
        "max_mustermann",
        "modification",
        64,
        "pre",
        "suf"
    ).unwrap();
    let result = pwm.generate(
        ".0123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789öä@€Whatever".to_owned(), 
        "0123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789".to_owned()).unwrap();
    assert_eq!(result, "preGWR5UvFWn69uJQqedOi07JErUQfxJLLB3ZlLPjltwUI0HFDoN6p5xhGdd5suf");
}

#[test]
fn test_each_algo_md5_v06(){
    let pwm = Pwm::new(
        HashAlgorithm::Md5Version06, 
        passwordmaker_rs::UseLeetWhenGenerating::Before { level: LeetLevel::Nine },
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789",
        "max_mustermann",
        "modification",
        64,
        "pre",
        "suf"
    ).unwrap();
    let result = pwm.generate(
        ".0123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789öä@€Whatever".to_owned(), 
        "0123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789".to_owned()).unwrap();
    assert_eq!(result, "pred554290760c59fd928b7aae270c31fdbb8857442f34d92cdaca38fcfc0suf");
}

#[test]
fn test_each_algo_hmac_md5_v06(){
    let pwm = Pwm::new(
        HashAlgorithm::HmacMd5Version06, 
        passwordmaker_rs::UseLeetWhenGenerating::Before { level: LeetLevel::Nine },
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789",
        "max_mustermann",
        "modification",
        64,
        "pre",
        "suf"
    ).unwrap();
    let result = pwm.generate(
        ".0123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789öä@€Whatever".to_owned(), 
        "0123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789".to_owned()).unwrap();
    assert_eq!(result, "pread606e84133024f01831a2ce2f6728654bca7c4dd8098ce2e6f75693d2suf");
}

#[test]
fn test_each_algo_sha1(){
    let pwm = Pwm::new(
        HashAlgorithm::Sha1, 
        passwordmaker_rs::UseLeetWhenGenerating::Before { level: LeetLevel::Nine },
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789",
        "max_mustermann",
        "modification",
        64,
        "pre",
        "suf"
    ).unwrap();
    let result = pwm.generate(
        ".0123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789öä@€Whatever".to_owned(), 
        "0123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789".to_owned()).unwrap();
    assert_eq!(result, "preWiv8G0J7zvTurM7Mwjy7LeXKBfbJCqJtP6EOAR8dhgF8dFh6h3OCUybzwusuf");
}

#[test]
fn test_each_algo_hmac_sha1(){
    let pwm = Pwm::new(
        HashAlgorithm::HmacSha1, 
        passwordmaker_rs::UseLeetWhenGenerating::Before { level: LeetLevel::Nine },
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789",
        "max_mustermann",
        "modification",
        64,
        "pre",
        "suf"
    ).unwrap();
    let result = pwm.generate(
        ".0123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789öä@€Whatever".to_owned(), 
        "0123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789".to_owned()).unwrap();
    assert_eq!(result, "prekLwEUc8ccgo6cK6nct1E58HZu3x5q0yCN8HHLVMx0QzjKMAfHCMyGeZxFEsuf");
}

#[test]
fn test_each_algo_sha256(){
    let pwm = Pwm::new(
        HashAlgorithm::Sha256, 
        passwordmaker_rs::UseLeetWhenGenerating::Before { level: LeetLevel::Nine },
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789",
        "max_mustermann",
        "modification",
        64,
        "pre",
        "suf"
    ).unwrap();
    let result = pwm.generate(
        ".0123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789öä@€Whatever".to_owned(), 
        "0123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789".to_owned()).unwrap();
    assert_eq!(result, "pregtXH0tXq1nKXH6adaYb9dtXgCAFl2cqCPMQW3E7EeDggB5Oft4HaNdq5uRsuf");
}

#[test]
fn test_each_algo_hmac_sha256(){
    let pwm = Pwm::new(
        HashAlgorithm::HmacSha256, 
        passwordmaker_rs::UseLeetWhenGenerating::Before { level: LeetLevel::Nine },
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789",
        "max_mustermann",
        "modification",
        64,
        "pre",
        "suf"
    ).unwrap();
    let result = pwm.generate(
        ".0123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789öä@€Whatever".to_owned(), 
        "0123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789".to_owned()).unwrap();
    assert_eq!(result, "pre5oyv5RXFzY0NiZF4b5JWQj5RUtotkI5dbJOeRJmSjpiYllu5ZZ8FXZqyY4suf");
}

#[test]
fn test_each_algo_ripemd_160(){
    let pwm = Pwm::new(
        HashAlgorithm::Ripemd160, 
        passwordmaker_rs::UseLeetWhenGenerating::Before { level: LeetLevel::Nine },
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789",
        "max_mustermann",
        "modification",
        64,
        "pre",
        "suf"
    ).unwrap();
    let result = pwm.generate(
        ".0123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789öä@€Whatever".to_owned(), 
        "0123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789".to_owned()).unwrap();
    assert_eq!(result, "preFJeYiiAXx8Aa1Fhvyy0ffw7D9AMU2qKLg5BBjwZxyT6rsbHctS1Yv1PhGjsuf");
}

#[test]
fn test_each_algo_hmac_ripemd_160(){
    let pwm = Pwm::new(
        HashAlgorithm::HmacRipemd160, 
        passwordmaker_rs::UseLeetWhenGenerating::Before { level: LeetLevel::Nine },
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789",
        "max_mustermann",
        "modification",
        64,
        "pre",
        "suf"
    ).unwrap();
    let result = pwm.generate(
        ".0123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789öä@€Whatever".to_owned(), 
        "0123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789".to_owned()).unwrap();
    assert_eq!(result, "preZ1zVB4UtRfvu6PhBvMPTkmAbX9WZ6Xzqb20OKFmKrFMfyF2eB4ImF2fhmWsuf");
}

#[test]
fn test_suffix_with_insufficient_length(){
    let pwm = Pwm::new(
        HashAlgorithm::HmacRipemd160, 
        passwordmaker_rs::UseLeetWhenGenerating::Before { level: LeetLevel::Nine },
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789",
        "max_mustermann",
        "modification",
        5,
        "pre",
        "suffix"
    ).unwrap();
    let result = pwm.generate(
        ".0123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789öä@€Whatever".to_owned(), 
        "0123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789".to_owned()).unwrap();
    assert_eq!(result, "suffi");
}

#[test]
fn test_suffix_with_insufficient_length_with_post_leet(){
    let pwm = Pwm::new(
        HashAlgorithm::HmacRipemd160, 
        passwordmaker_rs::UseLeetWhenGenerating::BeforeAndAfter{ level: LeetLevel::Nine },
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789",
        "max_mustermann",
        "modification",
        5,
        "pre",
        "suffix"
    ).unwrap();
    let result = pwm.generate(
        ".0123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789öä@€Whatever".to_owned(), 
        "0123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789".to_owned()).unwrap();
    assert_eq!(result, "suffi");
}