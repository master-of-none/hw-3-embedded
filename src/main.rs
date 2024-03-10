//! This is a rust program for controlling the RGB LED and UI components on the microbit v2.
//! This program has various components including an RGB LED, knob or potentiometer and buttons on the microbit board.
//! This program allows the setting and getting the RGB values in an asynchronous way and it also has the setting of the frame rates.
//!
#![no_std]
#![no_main]

mod knob;
mod rgb;
mod ui;
pub use knob::*;
pub use rgb::*;
pub use ui::*;

// Import the crates necessary for the code
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

// Import the embassy and microbit creates
use embassy_executor::Spawner;
use embassy_futures::join;
use embassy_sync::{blocking_mutex::raw::ThreadModeRawMutex, mutex::Mutex};
use embassy_time::Timer;
use microbit_bsp::{
    embassy_nrf::{
        bind_interrupts,
        gpio::{AnyPin, Level, Output, OutputDrive},
        saadc,
    },
    Button, Microbit,
};
use num_traits::float::FloatCore;

/// Global static variable for storing the RGB LED level.
pub static RGB_LEVELS: Mutex<ThreadModeRawMutex, [u32; 3]> = Mutex::new([0; 3]);
/// Global static variable for storing the Frame Rate.
pub static FRAME_RATE: Mutex<ThreadModeRawMutex, u32> = Mutex::new(10);
/// Constant to store the number of levels for the RGB LED.
pub const LEVELS: u32 = 16;

/// Function to get the current level of an RGB.
async fn get_rgb_levels() -> [u32; 3] {
    let rgb_levels = RGB_LEVELS.lock().await;
    *rgb_levels
}

/// Function to set the level of an RGB.
async fn set_rgb_levels<F>(setter: F)
where
    F: FnOnce(&mut [u32; 3]),
{
    let mut rgb_levels = RGB_LEVELS.lock().await;
    setter(&mut rgb_levels);
}

/// Function to get the current frame rate.
async fn get_frames() -> u32 {
    let frame_rate = FRAME_RATE.lock().await;
    *frame_rate
}

/// Function to set the frame rate.
async fn set_frames<F>(setter: F)
where
    F: FnOnce(&mut u32),
{
    let mut frame_rate = FRAME_RATE.lock().await;
    setter(&mut frame_rate)
}

/// Point of execution of the program.
#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    // Initialize an RTT to print and debug the messages.
    rtt_init_print!();
    // Initialize the microbit v2 board.
    let board = Microbit::default();

    // Bind the interrupts for Successive Approximation Analog-to-Digital Converter (SAADC)
    bind_interrupts!(struct Irqs {
        SAADC => saadc::InterruptHandler;
    });

    // Defined the LED Pins for Red, Green and Blue Colors.
    let led_pin = |p| Output::new(p, Level::Low, OutputDrive::Standard);
    let red = led_pin(AnyPin::from(board.p9));
    let green = led_pin(AnyPin::from(board.p8));
    let blue = led_pin(AnyPin::from(board.p16));
    let rgb: Rgb = Rgb::new([red, green, blue], 100);

    // Part to configure SAADC.
    let mut saadc_config = saadc::Config::default();
    saadc_config.resolution = saadc::Resolution::_14BIT;
    let saadc = saadc::Saadc::new(
        board.saadc,
        Irqs,
        saadc_config,
        [saadc::ChannelConfig::single_ended(board.p2)],
    );

    // Initialize the Knob Component.
    let knob = Knob::new(saadc).await;
    // Initialize the UI Component.
    let mut ui = Ui::new(knob, board.btn_a, board.btn_b);

    // Run the RGB and the UI tasks. This must be run concurrently.
    join::join(rgb.run(), ui.run()).await;

    // This is an error condition when the program control reaches here.
    panic!("fell off end of main loop");
}
