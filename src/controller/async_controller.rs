use core::marker::PhantomData;

use crate::{
    charset::{Charset, FallbackCharset, MinimalCharset},
    command::{Commands, *},
    device::{AsyncDevice, CommandExtAsync},
};

use super::{config::*, state::*, Controller};

impl<D: AsyncDevice> Controller<D, Uninit> {
    pub async fn init(mut self) -> Result<Controller<D, Init>, super::Error> {
        self.device
            .execute_commands_async::<Commands, _>(&[
                Enter4Bit.into(),
                FunctionSet {
                    data_length: self.initial_config.data_length,
                    number_of_lines: self.initial_config.lines,
                    character_font: self.initial_config.font,
                }
                .into(),
            ])
            .await?;

        let mut controller: Controller<_, Init> = Controller {
            device: self.device,
            initial_config: self.initial_config,
            runtime_config: self.runtime_config,
            _state: PhantomData,
        };

        controller.set_runtime_config(self.runtime_config).await?;
        controller.clear().await?;

        Ok(controller)
    }

    pub fn release(self) -> D {
        self.device
    }
}

impl<D: AsyncDevice> Controller<D, Init> {
    pub async fn clear(&mut self) -> Result<(), super::Error> {
        Ok(self.device.execute_command_async(&ClearDisplay).await?)
    }

    pub async fn reset(&mut self) -> Result<(), super::Error> {
        self.clear().await?;
        self.set_runtime_config(RuntimeConfig::default()).await?;
        self.set_cursor_position(0, 0).await
    }

    pub async fn return_home(&mut self) -> Result<(), super::Error> {
        Ok(self.device.execute_command_async(&ReturnHome).await?)
    }

    pub fn initial_config(&self) -> InitialConfig {
        self.initial_config
    }

    pub fn runtime_config(&self) -> RuntimeConfig {
        self.runtime_config
    }

    pub async fn set_runtime_config(&mut self, cfg: RuntimeConfig) -> Result<(), super::Error> {
        self.device
            .execute_commands_async::<Commands, _>(&[
                DisplayOnOff {
                    display: cfg.display,
                    cursor: cfg.cursor,
                    blinking: cfg.blink,
                }
                .into(),
                EntryModeSet {
                    cursor_move_direction: cfg.cursor_direction,
                    shift: cfg.shift,
                }
                .into(),
                SetBacklight(cfg.backlight).into(),
            ])
            .await?;

        self.runtime_config = cfg;

        Ok(())
    }

    pub fn backlight(&self) -> bool {
        self.runtime_config.backlight
    }

    pub async fn set_backlight(&mut self, state: bool) -> Result<(), super::Error> {
        let new_config = RuntimeConfig {
            backlight: state,
            ..self.runtime_config()
        };
        self.set_runtime_config(new_config).await
    }

    pub fn cursor_blinking(&self) -> bool {
        self.runtime_config.blink
    }

    pub async fn set_cursor_blinking(&mut self, state: bool) -> Result<(), super::Error> {
        let new_config = RuntimeConfig {
            blink: state,
            ..self.runtime_config()
        };
        self.set_runtime_config(new_config).await
    }

    pub fn cursor_visible(&self) -> bool {
        self.runtime_config.cursor
    }

    pub async fn set_cursor_visible(&mut self, state: bool) -> Result<(), super::Error> {
        let new_config = RuntimeConfig {
            cursor: state,
            ..self.runtime_config()
        };
        self.set_runtime_config(new_config).await
    }

    pub async fn set_cursor_position(&mut self, row: u8, col: u8) -> Result<(), super::Error> {
        if row >= 4 || col >= 0x40 {
            return Err(super::Error::OutOfBounds);
        }

        let addr = match (row, col) {
            (0, c) => c,
            (1, c) => 0x40 + c,
            (2, c) => 0x14 + c,
            (3, c) => 0x54 + c,
            _ => unreachable!(),
        };

        self.device
            .execute_command_async(&SetDDRamAddress(addr))
            .await?;

        Ok(())
    }

    pub async fn write_raw_char(&mut self, code: u8) -> Result<(), super::Error> {
        Ok(self.device.execute_command_async(&WriteChar(code)).await?)
    }

    pub async fn write_char_with_charset<C: Charset, const FALLBACK_CODE: u8>(
        &mut self,
        c: char,
        charset: &FallbackCharset<C, FALLBACK_CODE>,
    ) -> Result<(), super::Error> {
        self.write_raw_char(charset.char_to_code(c)).await
    }

    pub async fn write_char(&mut self, c: char) -> Result<(), super::Error> {
        self.write_char_with_charset(c, &MinimalCharset::BLANK_FALLBACK)
            .await
    }

    pub async fn write_raw_str(&mut self, raw_code: &[u8]) -> Result<(), super::Error> {
        for c in raw_code.iter().copied() {
            self.write_raw_char(c).await?;
        }
        Ok(())
    }

    pub async fn write_str_with_charset<C: Charset, const FALLBACK_CODE: u8>(
        &mut self,
        s: &str,
        charset: &FallbackCharset<C, FALLBACK_CODE>,
    ) -> Result<(), super::Error> {
        for c in s.chars() {
            self.write_char_with_charset(c, charset).await?;
        }
        Ok(())
    }

    pub async fn write_str(&mut self, s: &str) -> Result<(), super::Error> {
        self.write_str_with_charset(s, &MinimalCharset::BLANK_FALLBACK)
            .await
    }

    #[cfg(feature = "fmt")]
    #[cfg_attr(docsrs, doc(cfg(feature = "fmt")))]
    pub async fn write_fmt_with_charset<
        C: Charset,
        const FALLBACK_CODE: u8,
        const BUFFER_SIZE: usize,
    >(
        &mut self,
        args: core::fmt::Arguments<'_>,
        charset: &FallbackCharset<C, FALLBACK_CODE>,
    ) -> Result<(), super::Error> {
        let mut buffer = BufferedWrite(heapless::String::<BUFFER_SIZE>::new());
        // string formatting should be infallible
        let _ = core::fmt::write(&mut buffer, args);
        self.write_str_with_charset(&buffer.0, charset).await
    }

    #[cfg(feature = "fmt")]
    #[cfg_attr(docsrs, doc(cfg(feature = "fmt")))]
    pub async fn write_fmt<const BUFFER_SIZE: usize>(
        &mut self,
        args: core::fmt::Arguments<'_>,
    ) -> Result<(), super::Error> {
        // placeholders for const generic is not yet supported.
        self.write_fmt_with_charset::<_, b' ', BUFFER_SIZE>(args, &MinimalCharset::BLANK_FALLBACK)
            .await
    }

    pub fn device(&mut self) -> &mut D {
        &mut self.device
    }

    pub fn release(self) -> D {
        self.device
    }
}

#[cfg(feature = "fmt")]
#[cfg_attr(docsrs, doc(cfg(feature = "fmt")))]
struct BufferedWrite<const N: usize>(heapless::String<N>);

#[cfg(feature = "fmt")]
#[cfg_attr(docsrs, doc(cfg(feature = "fmt")))]
impl<const N: usize> core::fmt::Write for BufferedWrite<N> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let _ = self.0.push_str(s);
        // string formatting should be infallible
        Ok(())
    }
}
