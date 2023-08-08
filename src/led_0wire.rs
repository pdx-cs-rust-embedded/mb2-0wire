use crate::*;

pub struct Led0Wire {
    delay: Delay,
    pin: Pin<Output<PushPull>>,
}

impl Led0Wire {

    pub fn new(delay: Delay, pin: Pin<Output<PushPull>>) -> Self {
        Self { delay, pin }
    }

    pub fn send_cmd(&mut self, cmd: u8) {
        let mut pulser = |delay: &mut Delay| {
            self.pin.set_low().unwrap();
            delay.delay_us(200u16);
            self.pin.set_high().unwrap();
        };
        self.delay.delay_ms(50u16);
        pulser(&mut self.delay);
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
