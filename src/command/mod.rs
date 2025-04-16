mod common;
pub use common::*;

pub mod enter_4bit;
pub use enter_4bit::Enter4Bit;

pub mod write_char;
pub use write_char::WriteChar;

pub mod set_backlight;
pub use set_backlight::SetBacklight;

pub mod clear_display;
pub use clear_display::ClearDisplay;

pub mod return_home;
pub use return_home::ReturnHome;

pub mod entry_mode_set;
pub use entry_mode_set::EntryModeSet;

pub mod display_on_off;
pub use display_on_off::DisplayOnOff;

pub mod cursor_or_display_shift;
pub use cursor_or_display_shift::CursorOrDisplayShift;

pub mod function_set;
pub use function_set::FunctionSet;

pub mod set_cgram_address;
pub use set_cgram_address::SetCGRamAddress;

pub mod set_ddram_address;
pub use set_ddram_address::SetDDRamAddress;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Error {
    DeviceError,
}

macro_rules! define_commands {
    (
        $( $variant:ident($ty:ty) ),* $(,)?
    ) => {
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub enum Commands {
            $( $variant($ty), )*
        }


        impl SyncCommand for Commands {
            type Ret = ();
            type Err = Error;

            fn execute<D: super::device::SyncDevice + ?Sized>(&self, dev: &mut D) -> Result<Self::Ret, Self::Err> {
                match self {
                    $( Commands::$variant(cmd) => cmd.execute(dev), )*
                }
            }
        }

        #[cfg(feature = "async")]
        #[cfg_attr(docsrs, doc(cfg(feature = "async")))]
        impl AsyncCommand for Commands {
            type Ret = ();
            type Err = Error;

            async fn execute_async<D: super::device::AsyncDevice + ?Sized>(
                &self,
                dev: &mut D,
            ) -> Result<Self::Ret, Self::Err> {
                match self {
                    $( Commands::$variant(cmd) => cmd.execute_async(dev).await, )*
                }
            }
        }

        $(
            impl From<$ty> for Commands {
                fn from(cmd: $ty) -> Self {
                    Commands::$variant(cmd)
                }
            }
        )*
    };
}

define_commands! {
    Enter4Bit(Enter4Bit),
    WriteChar(WriteChar),
    SetBacklight(SetBacklight),
    ClearDisplay(ClearDisplay),
    ReturnHome(ReturnHome),
    EntryModeSet(EntryModeSet),
    DisplayOnOnff(DisplayOnOff),
    CursorOrDisplayShift(CursorOrDisplayShift),
    FunctionSet(FunctionSet),
    SetCGRamAddress(SetCGRamAddress),
    SetDDRamAddress(SetDDRamAddress),
}
