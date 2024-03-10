use crate::*;

/// This is a type alias for RGB LED pins.
type RgbPins = [Output<'static, AnyPin>; 3];

/// Structure to represent the RGB LED.
pub struct Rgb {
    /// Pins for the RGB LED.
    rgb: RgbPins,
    // Shadow variables to minimize lock contention.
    // Current brightness level for each color - R G B
    levels: [u32; 3],
    /// Time interval for each frame tick.
    tick_time: u64,
}

impl Rgb {
    /// Calculate the time interval for each frame tick.
    ///
    /// The time interval is calculated based on the specified frame rate and the number of levels.
    ///
    /// # Arguments
    /// * `frame_rate` - This is the frame rates which is per second.
    ///
    /// # Returns
    ///
    /// The time interval for each frame tick in micro seconds.
    fn frame_tick_time(frame_rate: u64) -> u64 {
        1_000_000 / (3 * frame_rate * LEVELS as u64)
    }
    /// Create a new instance of a RGB LED.
    ///
    /// # Arguments
    /// * `rgb` - Array of pins which represents the RGB :ED.
    /// * `frame_rate` - This is the frame rate for RGB LED flickering.
    ///
    /// # Returns
    ///
    /// The new RGB instance.
    pub fn new(rgb: RgbPins, frame_rate: u64) -> Self {
        let tick_time = Self::frame_tick_time(frame_rate);
        Self {
            rgb,
            levels: [0; 3],
            tick_time,
        }
    }

    /// Executes the single step of the RGB LED flickering for specific color
    ///
    /// This function turns on the LED and then waits for a certain duration and LED
    /// levels are controlled by buttons and knobs.
    ///
    /// # Arguments
    /// * `led` - The index of the LED color that is 0 for red, 1 for green and 2 for blue.
    async fn step(&mut self, led: usize) {
        // Get the level of the led.
        let level = self.levels[led];
        // Turn on the LED.
        if level > 0 {
            self.rgb[led].set_high();
            let on_time = level as u64 * self.tick_time;
            Timer::after_micros(on_time).await;
            // TUrn off the LED.
            self.rgb[led].set_low();
        }
        // Calculate the remaining duration of the LED to be off.
        let level = LEVELS - level;
        if level > 0 {
            let off_time = level as u64 * self.tick_time;
            Timer::after_micros(off_time).await;
        }
    }

    /// Runs the RGB LED loop
    ///
    /// This function continuously updates the RGB LED animation which is controlled by knob and frames.
    ///
    /// # Returns
    ///
    /// Returns nothing.
    pub async fn run(mut self) -> ! {
        loop {
            // Get current RGB brightness level.
            self.levels = get_rgb_levels().await;
            // Get current frame rate.
            let frame_rate = get_frames().await as u64;
            // Calculate the time interval for each frame tick based on frame rate.
            self.tick_time = Self::frame_tick_time(frame_rate);
            // Execute a step if color change for each LED Color.
            for led in 0..3 {
                self.step(led).await;
            }
        }
    }
}
