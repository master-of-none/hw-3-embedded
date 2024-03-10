use crate::*;

/// This is a structure to represent a state of the UI.
struct UiState {
    /// Brightness levels for each color.
    levels: [u32; 3],
    /// Current frame rate for LED display.
    frame_rate: u64,
}

impl UiState {
    /// THis displays the current state of the UI.
    fn show(&self) {
        // Array of RGB clearly defined.
        let names = ["red", "green", "blue"];
        // Print each color and frame rate.
        rprintln!();
        for (name, level) in names.iter().zip(self.levels.iter()) {
            rprintln!("{}: {}", name, level);
        }
        rprintln!("frame rate: {}", self.frame_rate);
    }
}

impl Default for UiState {
    /// Creates the default state of UI.
    ///
    /// The default state initializes RGB levels to the maximum and it sets the frame rate to 100.
    fn default() -> Self {
        Self {
            levels: [LEVELS - 1, LEVELS - 1, LEVELS - 1],
            frame_rate: 100,
        }
    }
}

/// This is a structure to represent the UI.
pub struct Ui {
    /// Knob to adjust the RGB level.
    knob: Knob,
    /// Button A for Blue level.
    button_a: Button,
    /// Button B for Green color.
    button_b: Button,
    /// Current state of the UI.
    state: UiState,
}

impl Ui {
    /// Constructs a new UI instance.
    ///
    /// # Arguments
    ///
    /// * `knob` - Knob to adjust the RGB level from ADC.
    /// * `button_a` - Button a to control Blue color.
    /// * `button_b` - Button b to control Green color.
    ///
    /// # Returns
    ///
    /// A new instance of UI.
    pub fn new(knob: Knob, button_a: Button, button_b: Button) -> Self {
        Self {
            knob,
            button_a,
            button_b,
            state: UiState::default(),
        }
    }

    /// RUns the UI loop
    ///
    /// This function is continuosly used to monitor the knob buttons to update the RGB levels and the Frame Rate.
    ///
    /// # Returns
    ///
    /// Function returns nothing.
    pub async fn run(&mut self) -> ! {
        // Check for button state to set the initial LED configuration.
        if self.button_a.is_low() && self.button_b.is_low() {
            self.initial_state(0).await
        } else if self.button_a.is_low() {
            self.initial_state(2).await
        } else if self.button_b.is_low() {
            self.initial_state(1).await
        }
        // Call the setter function to set RGB
        set_rgb_levels(|rgb| {
            *rgb = self.state.levels;
        })
        .await;
        // Display the initial state of UI.
        self.state.show();
        // Core UI Loop
        loop {
            let level = self.knob.measure().await;
            // Check button to adjust color levels and frame rate.
            if self.button_a.is_high() && self.button_b.is_high() {
                // If both buttons are not pressed, change the frame rate.
                self.change_frame_rate(level).await;
            } else if self.button_a.is_low() || self.button_b.is_low() {
                // If one of the buttons is pressed, then adjust the RGB levels.
                if self.button_a.is_low() && self.button_b.is_low() {
                    self.change_led(level, 0).await;
                } else if self.button_a.is_low() {
                    self.change_led(level, 2).await;
                } else if self.button_b.is_low() {
                    self.change_led(level, 1).await;
                }
                // Update RGB level for selected color.
            }
            // Set a delay.
            Timer::after_millis(50).await;
        }
    }

    /// Function to change the LED color.
    ///
    /// # Arguments
    ///
    /// `level` - The level of LED color
    /// * `rgb_value` - RGB value to be changed 0 for R, 1 for G and 2 for B.
    ///
    /// # Returns
    ///
    /// Returns nothing
    async fn change_led(&mut self, level: u32, rgb_value: usize) {
        if level != self.state.levels[rgb_value] {
            self.state.levels[rgb_value] = level;
            self.state.show();
            // Call the setter function to set the RGB levels.
            set_rgb_levels(|rgb| {
                *rgb = self.state.levels;
            })
            .await;
        }
    }

    /// Function to change the Frame Rate
    ///
    /// # Arguments
    ///
    /// * `level` - The level such that frame rate is changed
    ///
    /// # Returns
    ///
    /// Function returns nothing.
    async fn change_frame_rate(&mut self, level: u32) {
        if level != self.state.frame_rate as u32 {
            self.state.frame_rate = ((level * 10) + 10) as u64;
            self.state.show();
            // Call the setter function to set the frames.
            set_frames(|frame_rate| {
                *frame_rate = self.state.frame_rate as u32;
            })
            .await;
        }
    }

    /// Function to set the initial state of the LED
    ///
    /// # Arguments
    ///
    /// * `rgb_value` - RGB value to be changed 0 for R, 1 for G and 2 for B.
    ///
    /// # Returns
    ///
    /// Function returns nothing.
    async fn initial_state(&mut self, rgb_value: usize) {
        // Initialize the default RGB value to adjust.
        // Measure the knob position and set initial RGB levels.
        self.state.levels[rgb_value] = self.knob.measure().await;
    }
}
