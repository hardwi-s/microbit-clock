#![no_std]
#![no_main]

use cortex_m_rt::entry;
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::Timer};
use panic_halt as _;

#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    let mut led_pattern: [[u8; 5]; 5] = [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];

    loop {
        let (hours, minutes) = get_time();
        set_time(hours, minutes, &mut led_pattern);
        set_colon(true, &mut led_pattern);
        display.show(&mut timer, led_pattern, 500);
        set_colon(false, &mut led_pattern);
        display.show(&mut timer, led_pattern, 500);
    }
}

fn set_colon(state: bool, leds: &mut [[u8; 5]; 5]) {
    let mut i = 0;
    if state {
        i = 1;
    }
    leds[3][2] = i;
    leds[4][2] = i;
}

fn set_time(hours: u32, minutes: u32, leds: &mut [[u8; 5]; 5]) {
    let first_digit = hours / 10;
    let second_digit = hours % 10;
    let third_digit = minutes / 10;
    let fourth_digit = minutes % 10;
    set_column(0, first_digit, leds);
    set_column(1, second_digit, leds);
    set_column(3, third_digit, leds);
    set_column(4, fourth_digit, leds);
}

fn set_column(col: usize, digit: u32, leds: &mut [[u8; 5]; 5]) {
    let bit_0 = get_bit(digit, 0);
    let bit_1 = get_bit(digit, 1);
    let bit_2 = get_bit(digit, 2);
    let bit_3 = get_bit(digit, 3);
    let bit_4 = get_bit(digit, 4);
    leds[4][col] = bit_0;
    leds[3][col] = bit_1;
    leds[2][col] = bit_2;
    leds[1][col] = bit_3;
    leds[0][col] = bit_4;
}

fn get_bit(digit: u32, bit: i32) -> u8 {
    let mut bit_value = (digit & (0x01 << bit)).try_into().unwrap();
    if bit_value > 0 {
        bit_value = 1;
    }
    return bit_value;
}

fn get_time() -> (u32, u32) {
    let hours = 13;
    let mins = 47;
    return (hours, mins);
}
