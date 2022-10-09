#![warn(missing_docs)]
//! Library that should allow quick implementation of tools that are compatible with PasswordMaker Pro.
//! 
//! It forms the core of an upcoming PasswordMaker Pro compatible Sailfish OS App (as of yet unnamed).
//! This library intentionally does not depend on any specific implementation of the cryptographic hashes
//! it relies on. To see an example of how to integrate with the [Rust Crypto Hashes](https://github.com/RustCrypto/hashes),
//! see the integration tests.
//! 
//! There are two main functions in this library: [`generate_password`][PasswordMaker::generate_password] and
//! [`parse()`][UrlParsing::parse].


mod passwordmaker;
mod url_parsing;
use passwordmaker::{PasswordPartParameters, PasswordAssemblyParameters};
use passwordmaker::leet::LeetReplacementTable;
use std::error::Error;
use std::fmt::Display;
use std::marker::PhantomData;

/// Trait you need to implement for the various hash functions you need to provide.
/// Currently only a single function, that computes the hash of a string slice, is needed. This may change in a later version.
/// 
/// Beware: There is currently no way to put constraints on associated constants in Rust, so Block Size is not exposed.
/// It's anyhow the same (currently hardcoded) value for all supported algorithms.
pub trait Hasher {
    /// The output type of the respective hash function. Typically some form of byte array.
    type Output;
    /// Function that takes a byte array as input, and generates the cryptographic hash of it as output.
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
    /// The type that offers MD4 hashing. See the [`Md4`] trait.
    type MD4 : Md4;
    /// The type that offers MD5 hashing. See the [`Md5`] trait.
    type MD5 : Md5;
    /// The type that offers SHA1 hashing. See the [`Sha1`] trait.
    type SHA1 : Sha1;
    /// The type that offers SHA256 hashing. See the [`Sha256`] trait.
    type SHA256 : Sha256;
    /// The type that offers Ripemd160 hashing. See the [`Ripemd160`] trait.
    type RIPEMD160 : Ripemd160;
}

/// A single-use instance of PasswordMaker, created after all inputs are verified to be usable.
/// Only has one method, which is to generate the password.
pub struct PasswordMaker<'a, T : HasherList>{
    data : String, //aka url aka used text
    key : String, //aka master password
    username : &'a str,
    modifier : &'a str,
    password_part_parameters : PasswordPartParameters<'a>, //contains pre_leet, as this is different for different algorithms
    post_leet : Option<LeetReplacementTable>, //same for all algorithms. applied before before password assembly.
    assembly_settings : PasswordAssemblyParameters<'a>,
    _hashers : PhantomData<T>,
}

impl<'a, T : HasherList> PasswordMaker<'a, T>{
    /// Generates a password in a way that's (hopefully) compatible to PasswordMaker Pro. Returns an error for unusable input.
    /// 
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
    pub fn generate_password(
        data : String,
        key: String,
        hash_algorithm : HashAlgorithm,
        use_leet : UseLeetWhenGenerating,
        characters : &'a str,
        username : &'a str,
        modifier: &'a str,
        password_length : usize,
        prefix : &'a str,
        suffix : &'a str,
    ) -> Result<String, GenerationError>{
        Ok(
            Self::validate_input(data, key, hash_algorithm, use_leet, characters, username, modifier, password_length, prefix, suffix)?
            .generate()
        )
    }


    /// Validates user input and returns a `PasswordMaker` object if the input is valid.
    /// Use this if you want to split input validation from actual password computation.
    /// Otherwise, consider using the `generate_password` function for shorter code.
    /// 
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
        data : String,
        key: String,
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
    /// First Leet level: ["4", "b", "c", "d", "3", "f", "g", "h", "i", "j", "k", "1", "m", "n", "0", "p", "9", "r", "s", "7", "u", "v", "w", "x", "y", "z"]
    One,
    /// Second Leet level: ["4", "b", "c", "d", "3", "f", "g", "h", "1", "j", "k", "1", "m", "n", "0", "p", "9", "r", "5", "7", "u", "v", "w", "x", "y", "2"]
    Two,
    /// Third Leet level: ["4", "8", "c", "d", "3", "f", "6", "h", "'", "j", "k", "1", "m", "n", "0", "p", "9", "r", "5", "7", "u", "v", "w", "x", "'/", "2"]
    Three,
    /// Fourth Leet level: ["@", "8", "c", "d", "3", "f", "6", "h", "'", "j", "k", "1", "m", "n", "0", "p", "9", "r", "5", "7", "u", "v", "w", "x", "'/", "2"]
    Four,
    /// Fifth Leet level: ["@", "|3", "c", "d", "3", "f", "6", "#", "!", "7", "|<", "1", "m", "n", "0", "|>", "9", "|2", "$", "7", "u", "\\/", "w", "x", "'/", "2"]
    Five,
    /// Sixth Leet level: ["@", "|3", "c", "|)", "&", "|=", "6", "#", "!", ",|", "|<", "1", "m", "n", "0", "|>", "9", "|2", "$", "7", "u", "\\/", "w", "x", "'/", "2"]
    Six,
    /// Seventh Leet level: ["@", "|3", "[", "|)", "&", "|=", "6", "#", "!", ",|", "|<", "1", "^^", "^/", "0", "|*", "9", "|2", "5", "7", "(_)", "\\/", "\\/\\/", "><", "'/", "2"]
    Seven,
    /// Eigth Leet level: ["@", "8", "(", "|)", "&", "|=", "6", "|-|", "!", "_|", "|(", "1", "|\\/|", "|\\|", "()", "|>", "(,)", "|2", "$", "|", "|_|", "\\/", "\\^/", ")(", "'/", "\"/_"]
    Eight,
    /// Ninth Leet level: ["@", "8", "(", "|)", "&", "|=", "6", "|-|", "!", "_|", "|{", "|_", "/\\/\\", "|\\|", "()", "|>", "(,)", "|2", "$", "|", "|_|", "\\/", "\\^/", ")(", "'/", "\"/_"]
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
    /// Regular Md4 PasswordMaker Pro setting.
    Md4,
    /// HAMC Md4 PasswordMaker Pro setting. Encodes input as UTF-16 and discards upper byte (just as PasswordMaker Pro does for HMAC).
    HmacMd4,
    /// Regular Md5 PasswordMaker Pro setting.
    Md5,
    /// Md5 as computed by PasswordMaker Pro version 0.6. Encodes input as UTF-16 and discards upper byte and outputs MD5 as hex number.
    Md5Version06,
    /// HMAC Md5 PasswordMaker Pro setting. Encodes input as UTF-16 and discards upper byte (just as PasswordMaker Pro does for HMAC).
    HmacMd5,
    /// HMAC Md5 as computed by PasswordMaker Pro version 0.6. Encodes input as UTF-16 and discards upper byte and outputs MD5 as hex number.
    HmacMd5Version06,
    /// Regular Sha1 PasswordMaker Pro setting.
    Sha1,
    /// HAMC Sha1 PasswordMaker Pro setting. Encodes input as UTF-16 and discards upper byte (just as PasswordMaker Pro does for HMAC).
    HmacSha1,
    /// Regular Sha256 PasswordMaker Pro setting.
    Sha256,
    /// HAMC Sha256 PasswordMaker Pro setting. Encodes input as UTF-16 and discards upper byte (just as PasswordMaker Pro does for HMAC).
    HmacSha256,
    /// Regular Ripemd160 PasswordMaker Pro setting.
    Ripemd160,
    /// HAMC Ripemd160 PasswordMaker Pro setting. Encodes input as UTF-16 and discards upper byte (just as PasswordMaker Pro does for HMAC).
    HmacRipemd160,
}

/// When the Leet replacement shown in leet.rs is applied.
/// If Leet is enabled, the input will be converted to lower case.
/// It is always applied to each password part when the required password length
/// is longer than the length obtained by computing a single hash. This is important if the input data or output charset contains certain
/// characters where the lower case representation depends on context (e.g. 'Σ').
#[derive(Debug,Clone, Copy)]
pub enum UseLeetWhenGenerating {
    /// Do not apply Leet on input or output.
    NotAtAll,
    /// Apply Leet on the input before computing a password part.
    Before {
        /// The Leet level to apply to the input.
        level : LeetLevel,
    },
    /// Apply Leet on the generated password-part. Beware that this will force the password to lower-case characters.
    After {
        /// The Leet level to apply to the generated password parts.
        level : LeetLevel,
    },
    /// Apply Leet both, to the input for the hasher, and the generated password parts. Beware that this will force the password to lower-case characters.
    BeforeAndAfter {
        /// The Leet level to apply to both, input and generated password parts.
        level : LeetLevel,
    },
}

/// Settings for the parsing of the user's input URL.
/// This is used to generate the `data` parameter for [`PasswordMaker`].
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone)]
pub struct UrlParsing {
    use_protocol : ProtocolUsageMode,
    use_userinfo : bool,
    use_subdomains : bool,
    use_domain : bool,
    use_port_path : bool,
}

impl UrlParsing {
    /// Creates a new `UrlParsing` instance with the given settings.
    pub fn new(
        use_protocol : ProtocolUsageMode,
        use_userinfo : bool,
        use_subdomains : bool,
        use_domain : bool,
        use_port_path : bool,
    ) -> Self{
        UrlParsing{ use_protocol, use_userinfo, use_subdomains, use_domain, use_port_path, }
    }

    /// Parses an input string, applying the settings in `self`, and generates a string suitable for
    /// the `data` parameter of [`PasswordMaker`]
    pub fn parse(&self, input : &str) -> String{
        self.make_used_text_from_url(input)
    }
}

/// The "Use Protocol" checkbox in PasswordMaker Pro Javascript Edition has some weird behaviour, that's probably a bug.
/// This enum lets you select how to hande the case that the user wants to use the Protocol, but the input string doesn't contain one.
#[derive(Debug, Clone, Copy)]
pub enum ProtocolUsageMode{
    /// The protocol part of the URI is not used in the output.
    Ignored,
    /// The protocol part of the URI is used in the output, if it's non-empty in the input. Otherwise it isn't.
    Used,
    /// The protocol part of the URI is used in the output, if it's non-empty in the input. Otherwise the string "undefined" is used in the output.
    /// This mirrors behaviour of the PasswordMaker Pro Javascript Edition.
    UsedWithUndefinedIfEmpty,
}

/// Error returned if the supplied input did not meet expectations.
/// The two "missing" variants are self-explanatory, but the `InsufficientCharset` might need some explanation:
/// `InsufficientCharset` means that the output character set does not contain at least two grapheme clusters.
/// Since the output string is computed by doing a base system conversion from binary to number-of-grapheme-clusters,
/// any number of grapheme clusters lower than 2 forms a nonsensical input. There simply is no base-1 or base-0 number system.
#[derive(Debug, Clone, Copy)]
pub enum GenerationError {
    /// Password generation failed, because the user did not supply a master password.
    MissingMasterPassword,
    /// Password generation failed, because the user did not supply a text-to-use.
    MissingTextToUse,
    /// Password generation failed, because the character set supplied by the user did not contain at least 2 grapheme clusters.
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