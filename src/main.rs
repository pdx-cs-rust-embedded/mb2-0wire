//! "0Wire" LED demo.

#![no_main]
#![no_std]

use led_0wire::*;

use panic_rtt_target as _;
use rtt_target::{rtt_init_print, rprintln};

use cortex_m_rt::entry;
use microbit::{
    board::Board,
    hal::{
        prelude::*,
        delay::Delay,
        gpio,
        Timer,
    },
};

#[entry]
fn main() -> ! {
    // Set up.
    rtt_init_print!();
    let board = Board::take().unwrap();
    let pin_0wire = board.pins.p0_02.into_push_pull_output_drive(
        gpio::Level::Low,
        gpio::DriveConfig::HighDrive0HighDrive1,
    );
    let delay = Delay::new(board.SYST);
    let mut led_0wire = Led0Wire::new(delay, pin_0wire);
    let mut sleep = Timer::new(board.TIMER0);
    let button_a = board.buttons.button_a.into_floating_input().degrade();
    let button_b = board.buttons.button_b.into_floating_input().degrade();
    let buttons = [button_a, button_b];

    // Get a button state array.
    let get_buttons = || core::array::from_fn(
        |i| buttons[i].is_low().unwrap(),
    );

    led_0wire.send_cmd(Function::Static, Color::White).unwrap();

    // Run the demo.
    let mut fi = 0;
    let nfi = FUNCTIONS.len();
    loop {
        loop {
            match get_buttons() {
                [false, false] => sleep.delay_ms(100u16),
                [true, false] => {
                    fi = (fi + nfi - 1) % nfi;
                    break;
                }
                [false, true] => {
                    fi = (fi + 1) % nfi;
                    break;
                }
                _ => (),
            }
        }
        let f = FUNCTIONS[fi];
        rprintln!("{:?}", f);
        led_0wire.send_cmd(f, Color::Cyan).unwrap();
        sleep.delay_ms(500u16);
    }
}
