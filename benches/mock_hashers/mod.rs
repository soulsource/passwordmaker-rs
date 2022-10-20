//We want to bench the surrounding string manipulation, not the hashers.
//For this reason, we fake them with a black_box.

use passwordmaker_rs::{PasswordMaker, Hasher, HasherList, };
use criterion::{black_box};


pub(crate) struct MockMd4;
pub(crate) struct MockMd5;
pub(crate) struct MockSha1;
pub(crate) struct MockSha256;
pub(crate) struct MockRipeMD160;
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

pub(crate) struct MockHashes{}
impl HasherList for MockHashes {
    type MD4 = MockMd4;
    type MD5 = MockMd5;
    type SHA1 = MockSha1;
    type SHA256 = MockSha256;
    type RIPEMD160 = MockRipeMD160;
}

pub(crate) type Pwm<'a> = PasswordMaker<'a, MockHashes>;