use crate::*;

/// Struct to store local values for RGB levels and frame rate
struct UiState {
    levels: [u32; 3],
    frame_rate: u64,
}

impl UiState {
    /// Print current UI state to console
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

/// Struct for managing UI elements and state
pub struct Ui {
    ///
    knob: Knob,
    _button_a: Button,
    _button_b: Button,
    state: UiState,
}

impl Ui {
    /// Create new UI
    pub fn new(knob: Knob, _button_a: Button, _button_b: Button) -> Self {
        Self {
            knob,
            _button_a,
            _button_b,
            state: UiState::default(),
        }
    }

    /// Run the UI loop
    pub async fn run(&mut self) -> ! {
        self.state.levels[2] = self.knob.measure().await;
        set_rgb_levels(|rgb| {
            *rgb = self.state.levels;
        })
        .await;
        self.state.show();
        loop {
            let level = self.knob.measure().await;

            rprintln!("Knob: {}", level);

            let a_down = self._button_a.is_low();
            let b_down = self._button_b.is_low();

            match (a_down, b_down) {
                (false, false) => {
                    // rprintln!("Both up");
                    let new_fps = (1 + level as u64) * 10;
                    if new_fps != self.state.frame_rate {
                        self.state.frame_rate = new_fps;
                        rprintln!("Set Frame Rate: {}", new_fps);
                        set_frame_rate(|rate| *rate = new_fps).await;
                    }
                }
                (true, false) => {
                    rprintln!("A down, B up");
                    self.set_rgb_level(2, &level).await;
                }
                (false, true) => {
                    rprintln!("A up, B down");
                    self.set_rgb_level(1, &level).await;
                }
                (true, true) => {
                    rprintln!("both down");
                    self.set_rgb_level(0, &level).await;
                }
            }

            Timer::after_millis(50).await;
        }
    }

    // Set RGB level
    async fn set_rgb_level(&mut self, level_to_set: usize, level: &u32) {
        if self.state.levels[level_to_set] == *level {
            return;
        }

        let color_string = match level_to_set {
            0 => "Red",
            1 => "Green",
            2 => "Blue",
            _ => "Undefined",
        };

        rprintln!("Setting {} to {}", color_string, level);

        self.state.levels[level_to_set] = *level;
        self.state.show();
        set_rgb_levels(|rgb| {
            *rgb = self.state.levels;
        })
        .await
    }
}
