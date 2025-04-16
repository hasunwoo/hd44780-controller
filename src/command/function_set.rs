use crate::device::*;

use super::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FunctionSet {
    pub data_length: DataLength,
    pub number_of_lines: NumberOfLines,
    pub character_font: CharacterFont,
}

impl Default for FunctionSet {
    fn default() -> Self {
        Self {
            data_length: DataLength::FourBit,
            number_of_lines: NumberOfLines::Two,
            character_font: CharacterFont::FiveByEight,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DataLength {
    FourBit,
    EightBit,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum NumberOfLines {
    One,
    Two,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CharacterFont {
    FiveByEight,
    FiveByTen,
}

impl FunctionSet {
    fn code(&self) -> u8 {
        let mut code = 0x20;

        if self.data_length == DataLength::EightBit {
            code |= 0x1 << 4;
        }
        if self.number_of_lines == NumberOfLines::Two {
            code |= 0x1 << 3;
        }
        if self.character_font == CharacterFont::FiveByTen {
            code |= 0x1 << 2;
        }

        code
    }
}

impl SyncCommand for FunctionSet {
    type Ret = ();

    type Err = super::Error;

    fn execute<D: SyncDevice + ?Sized>(&self, dev: &mut D) -> Result<Self::Ret, Self::Err> {
        dev.write_byte(RegisterSelectMode::Command, self.code())
            .map_err(|_| super::Error::DeviceError)?;
        dev.delay_us(50);

        Ok(())
    }
}

#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
impl AsyncCommand for FunctionSet {
    type Ret = ();

    type Err = super::Error;

    async fn execute_async<D: AsyncDevice + ?Sized>(
        &self,
        dev: &mut D,
    ) -> Result<Self::Ret, Self::Err> {
        dev.write_byte_async(RegisterSelectMode::Command, self.code())
            .await
            .map_err(|_| super::Error::DeviceError)?;
        dev.delay_us_async(50).await;

        Ok(())
    }
}
