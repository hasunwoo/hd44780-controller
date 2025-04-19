pub trait Charset {
    fn char_to_code(&self, c: char) -> Option<u8>;
}

pub trait InfallibleCharset {
    fn char_to_code(&self, c: char) -> u8;
}

pub struct BlankFallback<C: Charset>(pub C);
pub struct QuestionFallback<C: Charset>(pub C);

impl<C: Charset> InfallibleCharset for BlankFallback<C> {
    fn char_to_code(&self, c: char) -> u8 {
        self.0.char_to_code(c).unwrap_or(b' ')
    }
}

impl<C: Charset> InfallibleCharset for QuestionFallback<C> {
    fn char_to_code(&self, c: char) -> u8 {
        self.0.char_to_code(c).unwrap_or(b'?')
    }
}
