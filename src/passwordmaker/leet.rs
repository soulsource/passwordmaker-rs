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
        "K??mi n?? ??xi h??r, ykist ??j??fum n?? b????i v??l og ??drepa." //yes, I know, the upper case ?? is wrong, but it's there to test a property.
    }
    fn get_icelandic_test_result(level : LeetLevel) -> &'static str {
        match level {
            LeetLevel::One => "k??mi n?? ??xi h??r, ykis7 ??j??fum n?? b????i v??1 0g ??dr3p4.",
            LeetLevel::Two => "k??m1 n?? ??x1 h??r, yk157 ??j??fum n?? b????1 v??1 0g ??dr3p4.",
            LeetLevel::Three => "k??m' n?? ??x' h??r, '/k'57 ??j??fum n?? 8????' v??1 06 ??dr3p4.",
            LeetLevel::Four => "k??m' n?? ??x' h??r, '/k'57 ??j??fum n?? 8????' v??1 06 ??dr3p@.",
            LeetLevel::Five => "|<??m! n?? ??x! #??|2, '/|<!$7 ??7??fum n?? |3????! \\/??1 06 ??d|23|>@.",
            LeetLevel::Six => "|<??m! n?? ??x! #??|2, '/|<!$7 ??,|??|=um n?? |3????! \\/??1 06 ??|)|2&|>@.",
            LeetLevel::Seven => "|<??^^! ^/?? ??><! #??|2, '/|<!57 ??,|??|=(_)^^ ^/?? |3????! \\/??1 06 ??|)|2&|*@.",
            LeetLevel::Eight => "|(??|\\/|! |\\|?? ??)(! |-|??|2, '/|(!$| ??_|??|=|_||\\/| |\\|?? 8????! \\/??1 ()6 ??|)|2&|>@.",
            LeetLevel::Nine => "|{??/\\/\\! |\\|?? ??)(! |-|??|2, '/|{!$| ??_|??|=|_|/\\/\\ |\\|?? 8????! \\/??|_ ()6 ??|)|2&|>@.",
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
        "?????????????????????? ???????????? ?????? ?????????????????? ??????????????"
    }
    fn get_greek_test_result(_level : LeetLevel) -> &'static str {
        "?????????????????????? ???????????? ?????? ?????????????????? ??????????????"
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