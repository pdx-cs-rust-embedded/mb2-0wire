//! Driver for "0Wire" LED using CZineLight [control
//! protocol](https://cdn.sparkfun.com/assets/9/f/1/c/6/CZineLight_0-Wire_Communication_Protocol.pdf).

use crate::*;

use thiserror_no_std::Error;

/// Operating mode for LED.
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Function {
    /// On at constant full brightness for color.
    Static = 0,
    /// Fades from dark to light over 4s, fades from
    /// light to dark over 4s, stays dark.
    LighterDarker = 1,
    /// Static color except 0.5s white flash every 4s.
    SlowFlashWhite = 2,
    /// Cycles smoothly between color and dark over 2s period.
    Wave = 3,
    /// Off except 0.5s color flash every 4s.
    SlowFlash = 4,
    /// Static color except 0.125s white flash every 1s.
    FastFlashWhite = 5,
    /// Static color except 0.5s off flash every 4s.
    SlowFlashOff = 6,
    /// Off except 0.125s color flash every 1s.
    FastFlash = 7,
}
use Function::*;

/// Invalid discriminant was passed to [TryFrom] for
/// [Function].
#[derive(Debug, Error)]
pub struct TryFromFunctionError;

/// List of all functions.
pub const FUNCTIONS: [Function; 8] = [
    Static,
    LighterDarker,
    SlowFlashWhite,
    Wave,
    SlowFlash,
    FastFlashWhite,
    SlowFlashOff,
    FastFlash,
];

impl TryFrom<u8> for Function {
    type Error = TryFromFunctionError;

    fn try_from(d: u8) -> Result<Self, TryFromFunctionError> {
        FUNCTIONS.get(d as usize).copied().ok_or(TryFromFunctionError)
    }
}

/// LED principal color.
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Color {
    Off = 0,
    Red = 1,
    Green = 2,
    Yellow = 3,
    Blue = 4,
    /// Really magenta.
    Purple = 5,
    Cyan = 6,
    White = 7,
    /// This color seems to correspond to some sort of
    /// special function. Listed in docs as RGYBPCW.
    FullColor = 8,
    /// This color seems to correspond to some sort of
    /// special function.
    Rgw = 9,
    /// This color seems to correspond to some sort of
    /// special function.
    Rbw = 0xa,
    /// This color seems to correspond to some sort of
    /// special function. Listed in docs as RGYBPW.
    SixColor = 0xb,
}
use Color::*;

/// Invalid discriminant was passed to [TryFrom] for
/// [Color].
#[derive(Debug, Error)]
pub struct TryFromColorError;

/// List of all colors.
pub const COLORS: [Color; 12] = [
    Off,
    Red,
    Green,
    Yellow,
    Blue,
    Purple,
    Cyan,
    White,
    FullColor,
    Rgw,
    Rbw,
    SixColor,
];

impl TryFrom<u8> for Color {
    type Error = TryFromColorError;

    fn try_from(d: u8) -> Result<Self, TryFromColorError> {
        COLORS.get(d as usize).copied().ok_or(TryFromColorError)
    }
}

/// Contains endpoints necessary for LED control.
pub struct Led0Wire {
    delay: Delay,
    pin: Pin<Output<PushPull>>,
}

impl Led0Wire {

    /// Create a new controller.
    pub fn new(delay: Delay, pin: Pin<Output<PushPull>>) -> Self {
        Self { delay, pin }
    }

    /// Send the given command to this controller.
    pub fn send_cmd(&mut self, f: Function, c: Color) {
        // Emits a 200µs low pulse.
        let mut pulser = |delay: &mut Delay| {
            self.pin.set_low().unwrap();
            delay.delay_us(200u16);
            self.pin.set_high().unwrap();
        };

        // Put a 50ms guard delay between commands, per the
        // spec.
        self.delay.delay_ms(50u16);

        // Transmit the pulse-encoded 7-bit command.
        pulser(&mut self.delay);
        let cmd = ((f as u8) << 4) | c as u8;
        for bit in (0..7).rev() {
            if (cmd >> bit) & 1 == 1 {
                self.delay.delay_ms(15u16);
            } else {
                self.delay.delay_ms(5u16);
            }
            pulser(&mut self.delay);
        }
    }
}
