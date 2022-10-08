mod passwordmaker;
use passwordmaker::{PasswordPartParameters, PasswordAssemblyParameters};
use passwordmaker::leet::LeetReplacementTable;
use std::error::Error;
use std::fmt::Display;
use std::marker::PhantomData;

/// Trait you need to implement for the various hash functions you need to provide.
/// Currently only a single function, that computes the hash of a string slice, is needed. This may change in a later version.
pub trait Hasher {
    type Output;
    fn hash(input : &[u8]) -> Self::Output;
}

/// Trait your Md4 hash function needs to implement.
pub trait Md4 : Hasher<Output = [u8;16]> {}
/// Trait your Md5 hash function needs to implement.
pub trait Md5 : Hasher<Output = [u8;16]> {}
/// Trait your Sha1 hash function needs to implement.
pub trait Sha1 : Hasher<Output = [u8;20]> {}
/// Trait your Sha256 hash function needs to implement.
pub trait Sha256 : Hasher<Output = [u8;32]> {}
/// Trait your Ripemd160 hash function needs to implement.
pub trait Ripemd160 : Hasher<Output = [u8;20]> {}

/// List of hash functions to use. Trait may change in later versions to include constructors for actual hasher objects.
pub trait HasherList {
    type MD4 : Md4;
    type MD5 : Md5;
    type SHA1 : Sha1;
    type SHA256 : Sha256;
    type RIPEMD160 : Ripemd160;
}

/// A single-use instance of PasswordMaker, created after all inputs are verified to be usable.
/// Only has one method, which is to generate the password.
pub struct PasswordMaker<'a, T : HasherList>{
    data : &'a str, //aka url aka used text
    key : &'a str, //aka master password
    username : &'a str,
    modifier : &'a str,
    password_part_parameters : PasswordPartParameters<'a>, //contains pre_leet, as this is different for different algorithms
    post_leet : Option<LeetReplacementTable>, //same for all algorithms. applied before before password assembly.
    assembly_settings : PasswordAssemblyParameters<'a>,
    _hashers : PhantomData<T>,
}

impl<'a, T : HasherList> PasswordMaker<'a, T>{
    /// Validates user input and returns a PasswordMaker if the input is valid.
    /// `data` is the string to use, typically a URL or a part of it.
    /// `key` is the master password.
    /// `hash_algorithm` is a PasswordMaker Pro algorithm selection.
    /// `use_leet` details when to use leet, if at all.
    /// `characters` is the list of output password characters. Actually this is not true. It's the list of grapheme clusters.
    /// `username` is the "username" field of PasswordMaker Pro.
    /// `modifier` is the "modifier" field of PasswordMaker Pro.
    /// `password_length` is the desired password length to generate.
    /// `prefix` is the prefix to which the password gets appended. Counts towards `password_length`.
    /// `suffix` is the suffix appended to the password. Counts towards `password_length`.
    pub fn validate_input(
        data : &'a str,
        key: &'a str,
        hash_algorithm : HashAlgorithm,
        use_leet : UseLeetWhenGenerating,
        characters : &'a str,
        username : &'a str,
        modifier: &'a str,
        password_length : usize,
        prefix : &'a str,
        suffix : &'a str,
    ) -> Result<Self, GenerationError> {
        if data.len() == 0 {
            Err(GenerationError::MissingTextToUse)
        } else if key.len() == 0 {
            Err(GenerationError::MissingMasterPassword)
        } else if !Self::is_suitable_as_output_characters(characters) {
            Err(GenerationError::InsufficientCharset)
        } else {
            let post_leet = match &use_leet {
                UseLeetWhenGenerating::NotAtAll
                 | UseLeetWhenGenerating::Before { .. }
                 => None,
                UseLeetWhenGenerating::After { level }
                 | UseLeetWhenGenerating::BeforeAndAfter { level }
                 => Some(LeetReplacementTable::get(level)),
            };
            Ok(PasswordMaker {
                data,
                key,
                username,
                modifier,
                password_part_parameters: PasswordPartParameters::from_public_parameters(hash_algorithm, &use_leet, characters),
                post_leet,
                assembly_settings: PasswordAssemblyParameters::from_public_parameters(prefix, suffix, password_length),
                _hashers: PhantomData,
            })
        }
    }

    /// Consumes the PasswordMaker and returns the generated password.
    pub fn generate(self) -> String {
        self.generate_password_verified_input()
    }
}

/// The leet level to use. The higher the value, the more obfuscated the results.
#[cfg_attr(test, derive(strum_macros::EnumIter))]
#[derive(Debug,Clone, Copy)]
pub enum LeetLevel {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

/// The hash algorithm to use, as shown in the GUI of the JavaScript edition of PasswordMaker Pro.
/// Most algorithms work by computing the hash of the input values and doing a number system base conversion to indices into
/// the supplied character array.
/// Notable exceptions are the HMAC algorithms, which not only compute the HMAC for the input, but also, before that, encode the
/// input as UTF-16 and discard all upper bytes.
/// The `Md5Version06` variant is for compatibility with ancient versions of PasswordMaker Pro. Not only does it also do the conversion
/// to UTF-16 and the discarding of the upper bytes, in addition it disregards the user-supplied character set completely, and instead
/// just outputs the hash encoded as hexadecimal numbers.
/// The `HmacMd5Version06` is similarly ignoring the supplied characters and using hexadecimal numbers as output.
#[derive(Debug,Clone, Copy)]
pub enum HashAlgorithm {
    Md4,
    HmacMd4,
    Md5,
    Md5Version06,
    HmacMd5,
    HmacMd5Version06,
    Sha1,
    HmacSha1,
    Sha256,
    HmacSha256,
    Ripemd160,
    HmacRipemd160,
}

/// When the leet replacement shown in leet.rs is applied. It is always applied to each password part when the required password length
/// is longer than the length obtained by computing a single hash. This is important if the input data or output charset contains certain
/// characters where the lower case representation depends on context (e.g. 'Σ').
#[derive(Debug,Clone, Copy)]
pub enum UseLeetWhenGenerating {
    NotAtAll,
    Before {
        level : LeetLevel,
    },
    After {
        level : LeetLevel,
    },
    BeforeAndAfter {
        level : LeetLevel,
    },
}

/// Error returned if the supplied input did not meet expectations.
/// The two "missing" variants are self-explanatory, but the `InsufficientCharset` might need some explanation:
/// `InsufficientCharset` means that the output character set does not contain at least two grapheme clusters.
/// Since the output string is computed by doing a base system conversion from binary to number-of-grapheme-clusters,
/// any number of grapheme clusters lower than 2 forms a nonsensical input. There simply is no base-1 or base-0 number system.
#[derive(Debug, Clone, Copy)]
pub enum GenerationError {
    MissingMasterPassword,
    MissingTextToUse,
    InsufficientCharset
}

impl Display for GenerationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GenerationError::MissingMasterPassword => write!(f, "No master password given."),
            GenerationError::MissingTextToUse => write!(f, "No text to use. Would just hash the master password."),
            GenerationError::InsufficientCharset => write!(f, "Charset needs to have at least 2 characters."),
        }
    }
}
impl Error for GenerationError{}