# mb2-0wire: "0Wire" RGB LED controller for MicroBit v2.
Bart Massey 2023

This crate demos control of a "0Wire" RGB LED controller for
the MicroBit v2. It relies on the `led-0wire` crate for its
core function.

This branch uses "high-drive" to be able to drive the LED
white directly, without using a MOSFET switch. This requires
a forked version of the upstream `microbit-v2` and
`nrf52833-hal` crates.

# License

This work is licensed under the "MIT License". Please see the file
`LICENSE.txt` in this distribution for license terms.
