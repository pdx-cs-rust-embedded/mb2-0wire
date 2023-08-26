//! "0Wire" LED demo.

#![no_main]
#![no_std]

use panic_rtt_target as _;
use rtt_target::rtt_init_print;

use cortex_m_rt::entry;
use microbit::{
    board::Board,
    hal::{
        prelude::*,
        delay::Delay,
        gpio,
    },
};

#[entry]
fn main() -> ! {
    // Set up.
    rtt_init_print!();
    let board = Board::take().unwrap();
    let pin_0wire = board.pins.p0_02;
    let mut delay = Delay::new(board.SYST);

    let mut pin_0wire = pin_0wire.into_push_pull_output(
        gpio::Level::Low,
    );
    delay.delay_ms(1000u16);
    pin_0wire.set_high().unwrap();
    delay.delay_ms(1000u16);
    let _ = pin_0wire.into_push_pull_output_drive(
        gpio::Level::High,
        gpio::DriveConfig::HighDrive0HighDrive1,
    );
    loop {
        delay.delay_ms(1000u16);
    }
}
