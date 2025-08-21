use std::fmt;

/// Config for a stack light endpoint.
#[derive(Debug, Clone)]
pub struct PatliteConfig {
    pub ip: String,
    pub port: u16,
}

/// Color codes per device spec.
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Color {
    Off = 0x00,
    Red = 0x01,
    Amber = 0x02,
    Lemon = 0x03,
    Green = 0x04,
    SkyBlue = 0x05,
    Blue = 0x06,
    Purple = 0x07,
    Pink = 0x08,
    White = 0x09,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Flash (blinking) state.
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Flash {
    Off = 0x00,
    On = 0x01,
}

/// Buzzer pattern
/// 0x00 = stop, 0x01..0x0B = patterns
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Buzzer {
    Off = 0x00,
    A = 0x01,
    B = 0x02,
    C = 0x03,
    D = 0x04,
    E = 0x05,
    F = 0x06,
    G = 0x07,
    H = 0x08,
    I = 0x09,
    J = 0x0A,
    K = 0x0B,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct PatliteState {
    pub tier1: Color,
    pub tier2: Color,
    pub tier3: Color,
    pub tier4: Color,
    pub tier5: Color,
    pub flash: Flash,
    pub buzzer: Buzzer,
}
