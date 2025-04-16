use crate::device::*;

use super::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CursorOrDisplayShift {
    pub mode: Mode,
    pub direction: Direction,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Mode {
    Cursor,
    Display,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Direction {
    Left,
    Right,
}

impl CursorOrDisplayShift {
    fn code(&self) -> u8 {
        let mut code = 0x10;

        if self.mode == Mode::Display {
            code |= 0x1 << 3;
        }
        if self.direction == Direction::Right {
            code |= 0x1 << 2;
        }

        code
    }
}

impl SyncCommand for CursorOrDisplayShift {
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
impl AsyncCommand for CursorOrDisplayShift {
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
