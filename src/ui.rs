use crate::*;

struct UiState {
    levels: [u32; 3],
    frame_rate: u64,
}

impl UiState {
    fn show(&self) {
        let names = ["red", "green", "blue"];
        rprintln!();
        for (name, level) in names.iter().zip(self.levels.iter()) {
            rprintln!("{}: {}", name, level);
        }
        rprintln!("frame rate: {}", self.frame_rate);
    }
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            levels: [LEVELS - 1, LEVELS - 1, LEVELS - 1],
            frame_rate: 100,
        }
    }
}

pub struct Ui {
    knob: Knob,
    button_a: Button,
    button_b: Button,
    state: UiState,
}

impl Ui {
    pub fn new(knob: Knob, button_a: Button, button_b: Button) -> Self {
        Self {
            knob,
            button_a,
            button_b,
            state: UiState::default(),
        }
    }

    pub async fn run(&mut self) -> ! {
        let mut rgb_value = 1;
        self.state.levels[rgb_value] = self.knob.measure().await;
        set_rgb_levels(|rgb| {
            *rgb = self.state.levels;
        })
        .await;
        self.state.show();
        loop {
            let level = self.knob.measure().await;
            if self.button_a.is_high() && self.button_b.is_high() {
                if level != self.state.frame_rate as u32 {
                    self.state.frame_rate = ((level * 10) + 10) as u64;
                    self.state.show();
                    set_frames(|frame_rate| {
                        *frame_rate = self.state.frame_rate as u32;
                    })
                    .await;
                }
            } else if self.button_a.is_low() || self.button_b.is_low() {
                if self.button_a.is_low() && self.button_b.is_low() {
                    rgb_value = 0
                } else if self.button_a.is_low() {
                    rgb_value = 2
                } else if self.button_b.is_low() {
                    rgb_value = 1
                }
                if level != self.state.levels[rgb_value] {
                    self.state.levels[rgb_value] = level;
                    self.state.show();
                    set_rgb_levels(|rgb| {
                        *rgb = self.state.levels;
                    })
                    .await;
                }
            }
            Timer::after_millis(50).await;
        }
    }
}
