#![no_std]
#![no_main]

use cortex_m_rt::entry;
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::{Timer, Twim},
    pac::{twim0::frequency::FREQUENCY_A, TWIM0}
};
use rtt_target::{rtt_init_print, rprintln};
use panic_rtt_target as _;


const RTC_ADDRESS: u8 = 0x52;
const RTC_MINUTES_REGISTER: u8 = 0x01;
const RTC_HOURS_REGISTER: u8 = 0x02;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    let mut i2c = Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100);

    let mut led_pattern: [[u8; 5]; 5] = [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];


    loop {
        rprintln!("tick");
        let (hours, minutes) = get_time(&mut i2c);
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

fn get_time(i2c: &mut Twim<TWIM0>) -> (u32, u32) {
    let hours = get_hours(i2c);
    let mins = get_minutes(i2c);
    return (hours, mins);
}
 
fn get_minutes(i2c: &mut Twim<TWIM0>) -> u32 {
    let mut minutes_reg = [0];
    i2c.write_then_read(RTC_ADDRESS, 
        &[RTC_MINUTES_REGISTER], 
        &mut minutes_reg).unwrap();
        
    //              Bit 7 Bit 6 Bit 5 Bit 4 Bit 3 Bit 2 Bit 1 Bit 0 
    // Minutes R/WP   -     40    20    10     8     4     2     1
    let minutes_tens: u8 = (minutes_reg[0] & 0x70) >> 4;
    let minutes_units: u8 = minutes_reg[0] & 0x0f;
    return ((minutes_tens * 10) + minutes_units).into();
}

fn get_hours(i2c: &mut Twim<TWIM0>) -> u32 {
    let mut hours_reg = [0];
    i2c.write_then_read(RTC_ADDRESS, 
        &[RTC_HOURS_REGISTER], 
        &mut hours_reg).unwrap();
    //              Bit 7 Bit 6 Bit 5 Bit 4 Bit 3 Bit 2 Bit 1 Bit 0 
    // Hours R/WP     -     -     20    10     8     4     2     1
    let hours_tens: u8 = (hours_reg[0] & 0x30) >> 4;
    let hours_units: u8 = hours_reg[0] & 0x0f;
    return ((hours_tens * 10) + hours_units).into();
}
