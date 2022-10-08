use unicode_segmentation::UnicodeSegmentation;
#[derive(Clone)]
pub(super) struct Grapheme<'a>(&'a str);

impl<'a> Grapheme<'a> {
    pub(super) fn iter_from_str(string : &'a str) -> impl Iterator<Item=Grapheme<'a>> {
        string.graphemes(true).map(Self::extract_grapheme_unchecked)
    }
    pub(super) fn get<'b>(&'b self) -> &'a str { self.0 }
    fn extract_grapheme_unchecked(s : &str) -> Grapheme { Grapheme(s) }
}