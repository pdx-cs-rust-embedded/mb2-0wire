use crate::*;

use thiserror_no_std::Error;

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Function {
    Static = 0,
    LigherDarker = 1,
    SlowFlashWhite = 2,
    Wave = 3,
    SlowFlash = 4,
    FlashWhite = 5,
    SlowFlashBgOff = 6,
    FastFlash = 7,
}

#[derive(Debug, Error)]
pub struct TryFromFunctionError;

impl TryFrom<u8> for Function {
    type Error = TryFromFunctionError;

    fn try_from(d: u8) -> Result<Self, TryFromFunctionError> {
        use Function::*;
        let fs = [
            Static,
            LigherDarker,
            SlowFlashWhite,
            Wave,
            SlowFlash,
            FlashWhite,
            SlowFlashBgOff,
            FastFlash,
        ];
        fs.get(d as usize).copied().ok_or(TryFromFunctionError)
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Color {
    Off = 0,
    Red = 1,
    Green = 2,
    Yellow = 3,
    Blue = 4,
    Purple = 5,
    Cyan = 6,
    White = 7,
    FullColor = 8,
    Rgw = 9,
    Rbw = 0xa,
    SixColor = 0xb,
}

#[derive(Debug, Error)]
pub struct TryFromColorError;

impl TryFrom<u8> for Color {
    type Error = TryFromColorError;

    fn try_from(d: u8) -> Result<Self, TryFromColorError> {
        use Color::*;
        let cs = [
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
        cs.get(d as usize).copied().ok_or(TryFromColorError)
    }
}

pub struct Led0Wire {
    delay: Delay,
    pin: Pin<Output<PushPull>>,
}

impl Led0Wire {

    pub fn new(delay: Delay, pin: Pin<Output<PushPull>>) -> Self {
        Self { delay, pin }
    }

    pub fn send_cmd(&mut self, f: Function, c: Color) {
        let mut pulser = |delay: &mut Delay| {
            self.pin.set_low().unwrap();
            delay.delay_us(200u16);
            self.pin.set_high().unwrap();
        };
        self.delay.delay_ms(50u16);
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
