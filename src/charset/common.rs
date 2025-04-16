pub trait Charset {
    fn char_to_code(&self, c: char) -> Option<u8>;
}

pub struct FallbackCharset<C: Charset, const FALLBACK_CODE: u8>(pub C);

pub type BlankFallback<C> = FallbackCharset<C, b' '>;
pub type QuestionFallback<C> = FallbackCharset<C, b'?'>;

impl<C: Charset, const FALLBACK_CODE: u8> FallbackCharset<C, FALLBACK_CODE> {
    pub fn char_to_code(&self, c: char) -> u8 {
        self.0.char_to_code(c).unwrap_or(FALLBACK_CODE)
    }
}
