#![no_main]
#![no_std]

pub mod led_0wire;
use led_0wire::*;

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

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();
    let pin_0wire = board.pins.p0_02.into_push_pull_output(Level::High).degrade();
    let delay = Delay::new(board.SYST);
    let mut led_0wire = Led0Wire::new(delay, pin_0wire);

    led_0wire.send_cmd(Function::Static, Color::Yellow);

    loop {
        asm::wfi();
    }
}
