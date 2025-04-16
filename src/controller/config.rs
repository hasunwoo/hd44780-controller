use crate::command::{
    entry_mode_set::{CursorMoveDirection, Shift},
    function_set::{CharacterFont, DataLength, NumberOfLines},
};

#[derive(Copy, Clone, Debug)]
pub struct InitialConfig {
    pub data_length: DataLength,
    pub lines: NumberOfLines,
    pub font: CharacterFont,
}

impl Default for InitialConfig {
    fn default() -> Self {
        Self {
            data_length: DataLength::FourBit,
            lines: NumberOfLines::Two,
            font: CharacterFont::FiveByEight,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct RuntimeConfig {
    pub display: bool,
    pub cursor: bool,
    pub blink: bool,
    pub backlight: bool,
    pub cursor_direction: CursorMoveDirection,
    pub shift: Shift,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            display: true,
            cursor: false,
            blink: false,
            backlight: true,
            cursor_direction: CursorMoveDirection::Increment,
            shift: Shift::CursorOnly,
        }
    }
}
