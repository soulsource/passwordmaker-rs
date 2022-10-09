use crate::LeetLevel;

pub(crate) struct LeetReplacementTable{
    lookup_table : &'static [&'static str; 26],
}

enum CharOrSlice{
    Char(char),
    Slice(&'static str)
}

impl LeetReplacementTable {
    /// Gets the appropriate leet replacement table for a given leet level.
    pub(crate) fn get(leet_level : LeetLevel) -> LeetReplacementTable {
        let lookup_table = match leet_level {
            LeetLevel::One => &["4", "b", "c", "d", "3", "f", "g", "h", "i", "j", "k", "1", "m", "n", "0", "p", "9", "r", "s", "7", "u", "v", "w", "x", "y", "z"],
            LeetLevel::Two => &["4", "b", "c", "d", "3", "f", "g", "h", "1", "j", "k", "1", "m", "n", "0", "p", "9", "r", "5", "7", "u", "v", "w", "x", "y", "2"],
            LeetLevel::Three => &["4", "8", "c", "d", "3", "f", "6", "h", "'", "j", "k", "1", "m", "n", "0", "p", "9", "r", "5", "7", "u", "v", "w", "x", "'/", "2"],
            LeetLevel::Four => &["@", "8", "c", "d", "3", "f", "6", "h", "'", "j", "k", "1", "m", "n", "0", "p", "9", "r", "5", "7", "u", "v", "w", "x", "'/", "2"],
            LeetLevel::Five => &["@", "|3", "c", "d", "3", "f", "6", "#", "!", "7", "|<", "1", "m", "n", "0", "|>", "9", "|2", "$", "7", "u", "\\/", "w", "x", "'/", "2"],
            LeetLevel::Six => &["@", "|3", "c", "|)", "&", "|=", "6", "#", "!", ",|", "|<", "1", "m", "n", "0", "|>", "9", "|2", "$", "7", "u", "\\/", "w", "x", "'/", "2"],
            LeetLevel::Seven => &["@", "|3", "[", "|)", "&", "|=", "6", "#", "!", ",|", "|<", "1", "^^", "^/", "0", "|*", "9", "|2", "5", "7", "(_)", "\\/", "\\/\\/", "><", "'/", "2"],
            LeetLevel::Eight => &["@", "8", "(", "|)", "&", "|=", "6", "|-|", "!", "_|", "|(", "1", "|\\/|", "|\\|", "()", "|>", "(,)", "|2", "$", "|", "|_|", "\\/", "\\^/", ")(", "'/", "\"/_"],
            LeetLevel::Nine => &["@", "8", "(", "|)", "&", "|=", "6", "|-|", "!", "_|", "|{", "|_", "/\\/\\", "|\\|", "()", "|>", "(,)", "|2", "$", "|", "|_|", "\\/", "\\^/", ")(", "'/", "\"/_"],
        };
        LeetReplacementTable { lookup_table }
    }

    /// Applies this replacement table to an input string slice.
    /// Needs an intermediate allocation.
    pub(super) fn leetify(&self, input: &str) -> String{
        //PasswordMaker Pro is converting input to lower-case before leet is applied.
        //We must apply to_lowercase on the whole input. PasswordMaker Pro is properly treating Final_Sigma, what we cannot do if we just
        //iterate on a per-char basis.
        input.to_lowercase().chars()
            .map(|c| self.conditionally_replace(c))
            .fold(String::with_capacity(input.len()), |mut result, c| {
                match c {
                    CharOrSlice::Char(c) => result.push(c),
                    CharOrSlice::Slice(s) => result.push_str(s),
                };
                result
            })
    }

    fn conditionally_replace(&self, character : char) -> CharOrSlice {
        match (character as usize).checked_sub(0x61).and_then(|index| self.lookup_table.get(index)) {
            Some(s) => CharOrSlice::Slice(s),
            None => CharOrSlice::Char(character),
        }
    }
}

#[cfg(test)]
mod leet_tests{
    use super::*;
    use strum::IntoEnumIterator;
    fn get_icelandic_test_string() -> &'static str {
        "Kæmi ný Öxi hér, ykist þjófum nú bæði víl og ádrepa." //yes, I know, the upper case Ö is wrong, but it's there to test a property.
    }
    fn get_icelandic_test_result(level : LeetLevel) -> &'static str {
        match level {
            LeetLevel::One => "kæmi ný öxi hér, ykis7 þjófum nú bæði ví1 0g ádr3p4.",
            LeetLevel::Two => "kæm1 ný öx1 hér, yk157 þjófum nú bæð1 ví1 0g ádr3p4.",
            LeetLevel::Three => "kæm' ný öx' hér, '/k'57 þjófum nú 8æð' ví1 06 ádr3p4.",
            LeetLevel::Four => "kæm' ný öx' hér, '/k'57 þjófum nú 8æð' ví1 06 ádr3p@.",
            LeetLevel::Five => "|<æm! ný öx! #é|2, '/|<!$7 þ7ófum nú |3æð! \\/í1 06 ád|23|>@.",
            LeetLevel::Six => "|<æm! ný öx! #é|2, '/|<!$7 þ,|ó|=um nú |3æð! \\/í1 06 á|)|2&|>@.",
            LeetLevel::Seven => "|<æ^^! ^/ý ö><! #é|2, '/|<!57 þ,|ó|=(_)^^ ^/ú |3æð! \\/í1 06 á|)|2&|*@.",
            LeetLevel::Eight => "|(æ|\\/|! |\\|ý ö)(! |-|é|2, '/|(!$| þ_|ó|=|_||\\/| |\\|ú 8æð! \\/í1 ()6 á|)|2&|>@.",
            LeetLevel::Nine => "|{æ/\\/\\! |\\|ý ö)(! |-|é|2, '/|{!$| þ_|ó|=|_|/\\/\\ |\\|ú 8æð! \\/í|_ ()6 á|)|2&|>@.",
        }
    }

    /// Runs a simple icelandic test sentence as found on the web through the leetifier for all levels.
    #[test]
    fn leet_test_icelandic(){
        for leet_level in LeetLevel::iter(){
            let result = LeetReplacementTable::get(leet_level).leetify(get_icelandic_test_string());
            let expected = get_icelandic_test_result(leet_level);
            assert_eq!(result, expected);
        }
    }

    fn get_greek_test_string() -> &'static str {
        "ΕΤΥΜΟΛΟΓΙΚΌ ΛΕΞΙΚΌ ΤΗΣ ΕΛΛΗΝΙΚΉΣ ΓΛΏΣΣΑΣ"
    }
    fn get_greek_test_result(_level : LeetLevel) -> &'static str {
        "ετυμολογικό λεξικό της ελληνικής γλώσσας"
    }

    #[test]
    fn leet_test_greek(){
        for leet_level in LeetLevel::iter(){
            let result = LeetReplacementTable::get(leet_level).leetify(get_greek_test_string());
            let expected = get_greek_test_result(leet_level);
            assert_eq!(result, expected);
        }
    }
}