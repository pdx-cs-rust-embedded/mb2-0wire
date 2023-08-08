#![no_main]
#![no_std]

use panic_rtt_target as _;
use rtt_target::rtt_init_print;

use cortex_m::asm;
use cortex_m_rt::entry;
use microbit::{
    board::Board,
    hal::{
        prelude::*,
        delay::Delay,
        gpio::{Pin, Level, Output, PushPull},
    },
};

fn send_cmd(cmd: u8, delay: &mut Delay, pin: &mut Pin<Output<PushPull>>) {
    let mut pulser = |delay: &mut Delay| {
        pin.set_low().unwrap();
        delay.delay_us(200u16);
        pin.set_high().unwrap();
    };
    delay.delay_ms(50u16);
    pulser(delay);
    for bit in (0..7).rev() {
        if (cmd >> bit) & 1 == 1 {
            delay.delay_ms(15u16);
        } else {
            delay.delay_ms(5u16);
        }
        pulser(delay);
    }
}

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();
    let mut pin_0wire = board.pins.p0_02.into_push_pull_output(Level::High).degrade();
    let mut delay = Delay::new(board.SYST);

    send_cmd(0x21, &mut delay, &mut pin_0wire);
    loop {
        asm::wfi();
    }
}
