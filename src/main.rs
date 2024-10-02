#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;

use panic_halt as _;
use rtt_target::rtt_init_print;


use microbit::{
    board::{Board},
    display::blocking::Display,
    hal::timer::Timer,
    hal::rng::Rng,
};

use embedded_hal::digital::{OutputPin, PinState};

fn random_tuple(rng: &mut Rng, max: usize) -> (usize, usize) {
    // Generate random numbers between 0 and 5 using the hardware RNG
    let first = rng.random_u8() as usize % max;
    let second = rng.random_u8() as usize % max;
    (first, second)
}

#[entry]
fn main() -> ! {
    rtt_init_print!();


    let mut board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    let mut rng = Rng::new(board.RNG);


    let mut leds = [[0; 5]; 5];


    loop {
        let (x, y) = random_tuple(&mut rng, 5_usize);
        if leds[x][y] == 0 {
            leds[x][y] = 1
        } else {
            leds[x][y] = 0
        }
        display.show(&mut timer, leds, 10);
    }
}