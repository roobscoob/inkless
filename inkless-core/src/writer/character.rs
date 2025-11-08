pub trait CharacterWriter {
    type Error;

    fn write_str(&mut self, s: &str) -> Result<(), Self::Error>;
}

impl<W: CharacterWriter + ?Sized> CharacterWriter for &mut W {
    type Error = W::Error;

    fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
        (**self).write_str(s)
    }
}

#[cfg(feature = "alloc")]
impl CharacterWriter for alloc::string::String {
    type Error = core::fmt::Error;

    fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
        self.push_str(s);
        Ok(())
    }
}
