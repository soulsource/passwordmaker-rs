use std::iter::repeat;
use unicode_segmentation::UnicodeSegmentation;
use leet::LeetReplacementTable;
use grapheme::Grapheme;

use base_conversion::BaseConversion;

use super::Hasher;

mod base_conversion;
mod grapheme;
mod hmac;
pub(crate) mod leet;

impl<'y, H : super::HasherList> super::PasswordMaker<'y, H>{
    pub(super) fn is_suitable_as_output_characters(characters : &str) -> bool {
        characters.graphemes(true).nth(1).is_some()
    }

    pub(super) fn generate_password_verified_input(&self, data : String, key : String) -> String {
        let modified_data = data + self.username + self.modifier;
        let get_modified_key = move |i : usize| { if i == 0 {key.clone()} else {key.clone() + "\n" + &i.to_string()}};
    
        //In Passwordmaker Pro, leet is applied on a per-password-part basis. This means that if a password part ends in an upper-case Sigma,
        //the results would differ if we moved leeting to after all password parts were joined, or worse, did it on a per-character level.
        //However, this makes the code a lot more complex, as it forces us to create an owned string for each password part before combining.
        //Therefore, we treat that case special.
        match &self.post_leet {
            None => Self::generate_password_verified_no_post_leet(&modified_data, get_modified_key, &self.assembly_settings, &self.password_part_parameters),
            Some(leet_level) => Self::generate_password_verified_with_post_leet(&modified_data, get_modified_key,&self.assembly_settings , &self.password_part_parameters, leet_level),
        }
    }

    fn generate_password_verified_no_post_leet<G : Fn(usize)->String>(modified_data : &str, get_modified_key : G, assembly_settings : &PasswordAssemblyParameters, password_part_parameters : &PasswordPartParameters) -> String {
        let password = (0..).flat_map(|i| Self::generate_password_part(modified_data, get_modified_key(i), password_part_parameters));
        combine_prefix_password_suffix(password, assembly_settings)
    }

    
    fn generate_password_verified_with_post_leet<G : Fn(usize)->String>(modified_data : &str, get_modified_key : G, assembly_settings : &PasswordAssemblyParameters, password_part_parameters : &PasswordPartParameters, post_leet : &LeetReplacementTable) -> String {
        let suffix_length = assembly_settings.suffix_length;
        let prefix_length = assembly_settings.prefix_length;
        let needed_password_length = assembly_settings.password_length.saturating_sub(suffix_length).saturating_sub(prefix_length);
    
        //Helper function that is used in try_fold below. Appends string part p to the input string, and counts graphemes.
        //Once grapheme count in total is >= needed_password_length, it returns a ControlFlow::Break.
        //Or, wait. Our target platform is limited to Rust 1.52 for now, so it's a Result::Err once the required length is reached.
        let append_strings_till_needed_length = |s: (String, usize),p : String| {
            let new_length = s.1 + p.graphemes(true).count();
            let st = s.0 + &p;
            if new_length >= needed_password_length  {
                Err(st)
            } else {
                Ok((st, new_length))
            }
       };
    
        //here we have to work on a string level... Because word-final sigma and leet's ToLower...
        let password = (0..)
            .map(|i| Self::generate_password_part(modified_data, get_modified_key(i), password_part_parameters))
            .map(|i| i.map(|g| g.get()).collect::<String>()) //make string from password part...
            .map(|non_leeted_password| post_leet.leetify(&non_leeted_password)) //leet it
            .try_fold((String::new(), 0), append_strings_till_needed_length).unwrap_err();
    
        combine_prefix_password_suffix(Grapheme::iter_from_str(&password), assembly_settings)
    }

    fn generate_password_part<'a>(data : &str, key : String, parameters : &'a PasswordPartParameters<'a>) -> GetGraphemesIterator<'a> {
        //Must follow PasswordMaker Pro closely here. For instance:
        // leet(key) + leet(data) != leet(key+data)
        //Soo, easiest way is to just make a _different_ function for each different combination of operations.
        //To make what happens explicit.
        
        match &parameters.hash_algorithm{
            AlgoSelection::V06(V06HmacOrNot::Hmac) => 
                Self::generate_password_part_v06_hmac(data, key, &parameters.pre_leet_level, &parameters.characters),
            AlgoSelection::V06(V06HmacOrNot::NonHmac) => 
                Self::generate_password_part_v06(data, key, &parameters.pre_leet_level, &parameters.characters),
            AlgoSelection::Modern(HmacOrNot::Hmac(a)) => 
                Self::generate_password_part_modern_hmac(data, key, a, &parameters.pre_leet_level, &parameters.characters),
            AlgoSelection::Modern(HmacOrNot::NonHmac(a)) => 
                Self::generate_password_part_modern(data, key, a, &parameters.pre_leet_level, &parameters.characters),
        }
    }

    fn generate_password_part_v06<'a>(
        second_part : &str,
        message : String,
        pre_leet_level: &Option<LeetReplacementTable>,
        characters : &'a Vec<Grapheme<'a>>,
    ) -> GetGraphemesIterator<'a> {
        let message = message + second_part;
        let message = pre_leet_level.as_ref().map(|l| l.leetify(&message)).unwrap_or(message);
        let message = yeet_upper_bytes(&message).collect::<Vec<u8>>();
        let hash = H::MD5::hash(&message);
        let grapheme_indices = hash.convert_to_base(characters.len());
        let grapheme_indices = yoink_additional_graphemes_for_06_if_needed(grapheme_indices);
        GetGraphemesIterator { graphemes : characters, inner: GetGraphemesIteratorInner::V06(grapheme_indices)}
    }

    
    fn generate_password_part_v06_hmac<'a>(
        data : &str,
        key : String,
        pre_leet_level: &Option<LeetReplacementTable>,
        characters : &'a Vec<Grapheme<'a>>,
    ) -> GetGraphemesIterator<'a>  {
        let key = pre_leet_level.as_ref().map(|l| l.leetify(&key)).unwrap_or(key);
        let leetified_data = pre_leet_level.as_ref().map(|l| l.leetify(data));
        let data = leetified_data.as_deref().unwrap_or(data);
        let key = yeet_upper_bytes(&key);
        let data = yeet_upper_bytes(data);
        let hash = hmac::hmac::<H::MD5,_,_>(key, data);
        let grapheme_indices = hash.convert_to_base(characters.len());
        let grapheme_indices = yoink_additional_graphemes_for_06_if_needed(grapheme_indices);
        GetGraphemesIterator { graphemes : characters, inner: GetGraphemesIteratorInner::V06(grapheme_indices)}
    }
    
    fn generate_password_part_modern_hmac<'a>(
        data : &str,
        key : String,
        algo : &Algorithm,
        pre_leet_level: &Option<LeetReplacementTable>,
        characters : &'a Vec<Grapheme<'a>>,
    ) -> GetGraphemesIterator<'a>  {
        let key = pre_leet_level.as_ref().map(|l| l.leetify(&key)).unwrap_or(key);
        let leetified_data = pre_leet_level.as_ref().map(|l| l.leetify(data));
        let data = leetified_data.as_deref().unwrap_or(data);
        let grapheme_indices = match algo {
            Algorithm::Md4 => 
                modern_hmac_to_grapheme_indices::<H::MD4>(&key, data, characters.len()),
            Algorithm::Md5 => 
                modern_hmac_to_grapheme_indices::<H::MD5>(&key, data, characters.len()),
            Algorithm::Sha1 => 
                modern_hmac_to_grapheme_indices::<H::SHA1>(&key, data, characters.len()),
            Algorithm::Sha256 => 
                modern_hmac_to_grapheme_indices::<H::SHA256>(&key, data, characters.len()),
            Algorithm::Ripemd160 => 
                modern_hmac_to_grapheme_indices::<H::RIPEMD160>(&key, data, characters.len()),
        };
        GetGraphemesIterator { graphemes : characters, inner: GetGraphemesIteratorInner::Modern(grapheme_indices)}
    }
    
    fn generate_password_part_modern<'a>(
        second_part : &str,
        message : String,
        algo : &Algorithm,
        pre_leet_level: &Option<LeetReplacementTable>,
        characters : &'a Vec<Grapheme<'a>>,
    ) -> GetGraphemesIterator<'a>  {
        let message = message + second_part;
        let message = pre_leet_level.as_ref().map(|l| l.leetify(&message)).unwrap_or(message);
        let grapheme_indices = match algo {
            Algorithm::Md4 => 
                modern_message_to_grapheme_indices::<H::MD4>(&message, characters.len()),
            Algorithm::Md5 => 
                modern_message_to_grapheme_indices::<H::MD5>(&message,characters.len()),
            Algorithm::Sha1 => 
                modern_message_to_grapheme_indices::<H::SHA1>(&message,characters.len()),
            Algorithm::Sha256 => 
                modern_message_to_grapheme_indices::<H::SHA256>(&message,characters.len()),
            Algorithm::Ripemd160 => 
                modern_message_to_grapheme_indices::<H::RIPEMD160>(&message,characters.len()),
        };
        GetGraphemesIterator { graphemes : characters, inner: GetGraphemesIteratorInner::Modern(grapheme_indices)}
    }
}

pub(super) struct PasswordAssemblyParameters<'a> {
    suffix : &'a str,
    prefix : &'a str,
    password_length : usize,
    suffix_length : usize,
    prefix_length : usize,
}
impl<'a> PasswordAssemblyParameters<'a> {
    pub(super) fn from_public_parameters(prefix : &'a str, suffix : &'a str, password_length : usize) -> Self{
        PasswordAssemblyParameters {
            suffix,
            prefix,
            password_length,
            suffix_length: Grapheme::iter_from_str(suffix).count(),
            prefix_length: Grapheme::iter_from_str(prefix).count(),
        }
    }
}

fn combine_prefix_password_suffix<'a, T : Iterator<Item=Grapheme<'a>>>(password: T, assembly_settings : &PasswordAssemblyParameters<'a>) -> String {
    //Rust's collect only uses the lower hint for pre-allocation. UnicodeSegmentation is giving correct hints,
    //meaning that the lower bound is 1 (or 0 for empty strings).
    //We know however, that assembly_settings.password_length is a much better lower bound. Still too low for
    //passwords that contain characters that take more than 1 byte though. Still, this value should reduce the number of needed re-allocations drastically.
    let mut result = String::with_capacity(assembly_settings.password_length);
    result.extend(Grapheme::iter_from_str(assembly_settings.prefix)
        .chain(password)
        .take(assembly_settings.password_length.saturating_sub(assembly_settings.suffix_length))
        .chain(Grapheme::iter_from_str(assembly_settings.suffix))
        .take(assembly_settings.password_length)//cut end if suffix_length is larger than password_length...
        .map(|g| g.get()));
    result
}

enum GetGraphemesIteratorInner {
    Modern(std::iter::Rev<std::vec::IntoIter<usize>>),
    V06(std::iter::Chain<std::iter::Take<std::iter::Repeat<usize>>, std::iter::Rev<std::vec::IntoIter<usize>>>)
}
struct GetGraphemesIterator<'a> {
    graphemes : &'a Vec<Grapheme<'a>>,
    inner : GetGraphemesIteratorInner
    //There really should be a better solution than storing those values. If we had arbitrary-length multiplication and subtraction maybe?
    //like, finding the highest potence of divisor that still is smaller than the dividend, and dividing by that one to get the left-most digit,
    //dividing the remainder of this operation by the next-lower potence of divisor to get the second digit, and so on?
}

impl<'a> Iterator for GetGraphemesIterator<'a> {
    type Item = Grapheme<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let idx = match &mut self.inner {
            GetGraphemesIteratorInner::Modern(i) => i.next(),
            GetGraphemesIteratorInner::V06(i) => i.next(),
        };
        idx.and_then(|idx| self.graphemes.get(idx).cloned())
    }
}

fn modern_hmac_to_grapheme_indices<T>(key : &str, data: &str, divisor : usize) -> std::iter::Rev<std::vec::IntoIter<usize>>
    where T:Hasher,
    <T as Hasher>::Output: BaseConversion + AsRef<[u8]>
{
    hmac::hmac::<T,_,_>(key.bytes(), data.bytes()).convert_to_base(divisor)
}

fn modern_message_to_grapheme_indices<T>(data: &str, divisor : usize) -> std::iter::Rev<std::vec::IntoIter<usize>>
    where T:Hasher,
    <T as Hasher>::Output: BaseConversion
{
    T::hash(data.as_bytes()).convert_to_base(divisor)
}

pub(super) struct PasswordPartParameters<'a>{
    hash_algorithm : AlgoSelection,
    pre_leet_level : Option<LeetReplacementTable>,
    characters : Vec<Grapheme<'a>>,
}

impl<'a> PasswordPartParameters<'a>{
    pub(super) fn from_public_parameters(hash_algorithm : super::HashAlgorithm, leet : super::UseLeetWhenGenerating, characters : &'a str) -> Self {
        use super::UseLeetWhenGenerating;
        let hash_algorithm = AlgoSelection::from_public_parameters(hash_algorithm);
        PasswordPartParameters{
            characters: match &hash_algorithm {
                AlgoSelection::V06(_) => Grapheme::iter_from_str("0123456789abcdef").collect(),
                AlgoSelection::Modern(_) => Grapheme::iter_from_str(characters).collect(),
            },
            pre_leet_level: match leet {
                UseLeetWhenGenerating::NotAtAll
                 | UseLeetWhenGenerating::After{..} => None,
                UseLeetWhenGenerating::Before { level }
                 | UseLeetWhenGenerating::BeforeAndAfter { level } => Some(LeetReplacementTable::get(level)),
            },
            hash_algorithm,
        }
    }
}

enum Algorithm {
    Md4,
    Md5,
    Sha1,
    Sha256,
    Ripemd160,
}

enum HmacOrNot{
    Hmac(Algorithm),
    NonHmac(Algorithm),
}

enum V06HmacOrNot{
    Hmac,
    NonHmac,
}

enum AlgoSelection{
    V06(V06HmacOrNot),
    Modern(HmacOrNot),
}

impl AlgoSelection {
    fn from_public_parameters(settings_algorithm : super::HashAlgorithm) -> Self {
        use super::HashAlgorithm;
        match settings_algorithm {
            HashAlgorithm::Md5Version06 => AlgoSelection::V06(V06HmacOrNot::NonHmac),
            HashAlgorithm::HmacMd5Version06 => AlgoSelection::V06(V06HmacOrNot::Hmac),
            HashAlgorithm::Md4 => AlgoSelection::Modern(HmacOrNot::NonHmac(Algorithm::Md4)),
            HashAlgorithm::HmacMd4 => AlgoSelection::Modern(HmacOrNot::Hmac(Algorithm::Md4)),
            HashAlgorithm::Md5 => AlgoSelection::Modern(HmacOrNot::NonHmac(Algorithm::Md5)),
            HashAlgorithm::HmacMd5 => AlgoSelection::Modern(HmacOrNot::Hmac(Algorithm::Md5)),
            HashAlgorithm::Sha1 => AlgoSelection::Modern(HmacOrNot::NonHmac(Algorithm::Sha1)),
            HashAlgorithm::HmacSha1 => AlgoSelection::Modern(HmacOrNot::Hmac(Algorithm::Sha1)),
            HashAlgorithm::Sha256 => AlgoSelection::Modern(HmacOrNot::NonHmac(Algorithm::Sha256)),
            HashAlgorithm::HmacSha256 => AlgoSelection::Modern(HmacOrNot::Hmac(Algorithm::Sha256)),
            HashAlgorithm::Ripemd160 => AlgoSelection::Modern(HmacOrNot::NonHmac(Algorithm::Ripemd160)),
            HashAlgorithm::HmacRipemd160 => AlgoSelection::Modern(HmacOrNot::Hmac(Algorithm::Ripemd160)),
        }
    }
}

// Yeets the upper bytes of each UTF-16 char representation. Needed, because PasswordMaker Pro did that for MD5 in version 0.6
// Returns bytes, because there's no way that this transform doesn't break the string.
#[allow(clippy::cast_possible_truncation)] //clippy, stop complaining. Truncating is the very purpose of this function...
fn yeet_upper_bytes(input : &str) -> impl Iterator<Item=u8> + Clone + '_ {
    input.encode_utf16().map(|wide_char| wide_char as u8)
}

//signature subject to change, but need named types...
fn yoink_additional_graphemes_for_06_if_needed(input : std::iter::Rev<std::vec::IntoIter<usize>>)
     -> std::iter::Chain<std::iter::Take<std::iter::Repeat<usize>>, std::iter::Rev<std::vec::IntoIter<usize>>>
{
    repeat(0_usize).take(32-input.len()).chain(input)
}