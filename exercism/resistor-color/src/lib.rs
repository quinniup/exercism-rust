use enum_iterator::{all, Sequence};
use int_enum::{IntEnum, IntEnumError};

#[repr(u32)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Sequence, IntEnum)]
pub enum ResistorColor {
    Black = 0,
    Brown = 1,
    Red = 2,
    Orange = 3,
    Yellow = 4,
    Green = 5,
    Blue = 6,
    Violet = 7,
    Grey = 8,
    White = 9,
}

pub fn color_to_value(_color: ResistorColor) -> u32 {
    _color.int_value()
}

pub fn value_to_color_string(value: u32) -> String {
    let item: Result<ResistorColor, IntEnumError<ResistorColor>> = ResistorColor::from_int(value);
    match item {
        Ok(x) => format!("{:?}", x),
        Err(_) => String::from("value out of range"),
    }
}

pub fn colors() -> Vec<ResistorColor> {
    all::<ResistorColor>().collect::<Vec<_>>()
}
