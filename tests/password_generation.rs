use passwordmaker_rs::{PasswordMaker, Hasher, HasherList, HashAlgorithm};
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