use crate::device::*;

use super::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EntryModeSet {
    pub cursor_move_direction: CursorMoveDirection,
    pub shift: Shift,
}

impl Default for EntryModeSet {
    fn default() -> Self {
        Self {
            cursor_move_direction: CursorMoveDirection::Increment,
            shift: Shift::CursorOnly,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CursorMoveDirection {
    Increment,
    Decrement,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Shift {
    CursorOnly,
    CursorAndScreen,
}

impl EntryModeSet {
    fn code(&self) -> u8 {
        let mut code = 0x04;

        if self.cursor_move_direction == CursorMoveDirection::Increment {
            code |= 0x1 << 1;
        }
        if self.shift == Shift::CursorAndScreen {
            code |= 0x1 << 0;
        }

        code
    }
}

impl SyncCommand for EntryModeSet {
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
impl AsyncCommand for EntryModeSet {
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
