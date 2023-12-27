#![no_std]
#![no_main]

use cortex_m_rt::entry;
use microbit as _;
use panic_halt as _;

#[entry]
fn main() -> ! {
    loop {}
}
