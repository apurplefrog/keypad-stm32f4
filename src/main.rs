#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use rtt_target::{rprint, rprintln, rtt_init_print};
use stm32f4xx_hal::{self, gpio::GpioExt, pac::Peripherals};

#[entry]
fn main() -> ! {
    // Setup debug printing
    rtt_init_print!();

    // Setup GPIO peripherals
    let p = Peripherals::take().unwrap();
    let gpioa = p.GPIOA.split();
    let gpiob = p.GPIOB.split();
    let gpioc = p.GPIOC.split();

    // Rows are set to high to check which buttons thereon are pulled high
    let mut row1 = gpioa.pa7.into_push_pull_output();
    let mut row2 = gpiob.pb6.into_push_pull_output();
    let mut row3 = gpioc.pc7.into_push_pull_output();
    let mut row4 = gpioa.pa9.into_push_pull_output();

    // Columns are what is sent back to determine which key is pressed
    let col1 = gpioa.pa8.into_pull_down_input();
    let col2 = gpiob.pb10.into_pull_down_input();
    let col3 = gpiob.pb4.into_pull_down_input();
    let col4 = gpiob.pb5.into_pull_down_input();

    // Keys on the keypad
    const KEYS: [char; 16] = [
        '1', '2', '3', 'A', // first row of keys
        '4', '5', '6', 'B', // second
        '7', '8', '9', 'C', // third
        '*', '0', '#', 'D', // forth
    ];

    // Track previous keys pressed to stop repeated printing of the same state
    let mut previous_keys_pressed = [false; KEYS.len()];

    loop {
        // We don't know what keys are pressed at the moment, so we set keys_pressed to an array of
        // booleans set to false
        let mut keys_pressed = [false; KEYS.len()];

        // Check what columns of row1 are pressed at the moment
        row1.set_high();
        keys_pressed[0] = col1.is_high();
        keys_pressed[1] = col2.is_high();
        keys_pressed[2] = col3.is_high();
        keys_pressed[3] = col4.is_high();
        row1.set_low();

        // Check what columns of row2 are pressed at the moment
        row2.set_high();
        keys_pressed[4] = col1.is_high();
        keys_pressed[5] = col2.is_high();
        keys_pressed[6] = col3.is_high();
        keys_pressed[7] = col4.is_high();
        row2.set_low();

        // Check what columns of row3 are pressed at the moment
        row3.set_high();
        keys_pressed[8] = col1.is_high();
        keys_pressed[9] = col2.is_high();
        keys_pressed[10] = col3.is_high();
        keys_pressed[11] = col4.is_high();
        row3.set_low();

        // Check what columns of row4 are pressed at the moment
        row4.set_high();
        keys_pressed[12] = col1.is_high();
        keys_pressed[13] = col2.is_high();
        keys_pressed[14] = col3.is_high();
        keys_pressed[15] = col4.is_high();
        row4.set_low();

        // Check two conditions:
        // - Is there only one key being pressed?
        // - Have there been any changes to what keys have been pressed?
        // If yes to both conditions, print what key(s) the program has registered
        if keys_pressed.iter().filter(|b| **b).count() == 1 && keys_pressed != previous_keys_pressed
        {
            rprint!("Key pressed: ");
            for i in 0..keys_pressed.len() {
                if keys_pressed[i] {
                    rprint!("{}", KEYS[i]);
                    break;
                }
            }
            rprintln!();
        }

        // The previous_keys_pressed are now equivalent to keys_pressed, as the keys pressed are
        // now previous
        previous_keys_pressed = keys_pressed;
    }
}
