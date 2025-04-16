use super::{BlankFallback, Charset, FallbackCharset, QuestionFallback};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct MinimalCharset;

impl MinimalCharset {
    pub const BLANK_FALLBACK: BlankFallback<Self> = FallbackCharset(Self);
    pub const QUESTION_FALLBACK: QuestionFallback<Self> = FallbackCharset(Self);
}

impl Charset for MinimalCharset {
    fn char_to_code(&self, c: char) -> Option<u8> {
        match c {
            '\\' | '\x10'..='\x1f' => None,
            '\x00'..='\x7d' => Some(c as u8),
            _ => None,
        }
    }
}
