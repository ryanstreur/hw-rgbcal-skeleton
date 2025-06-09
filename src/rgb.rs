use crate::*;

type RgbPins = [Output<'static, AnyPin>; 3];

/// Representation of RGB Pins and the data necessary to display specific colors
pub struct Rgb {
    rgb: RgbPins,
    // Shadow variables to minimize lock contention.
    levels: [u32; 3],
    tick_time: u64,
}

impl Rgb {
    /// Calculate the number of milliseconds to keep each LED lit per tick based
    /// on frame rate
    fn frame_tick_time(frame_rate: u64) -> u64 {
        1_000_000 / (3 * frame_rate * LEVELS as u64)
    }

    /// Create new RBG representation
    pub fn new(rgb: RgbPins, frame_rate: u64) -> Self {
        let tick_time = Self::frame_tick_time(frame_rate);
        Self {
            rgb,
            levels: [0; 3],
            tick_time,
        }
    }

    /// Run a single step for one single LED pin
    ///
    /// 1. Access level for that pin
    /// 2. Get the tick time from internal state
    /// 3. Set that level to high for that amount of time, then set it to low
    ///    for the rest of the tick
    async fn step(&mut self, led: usize) {
        let level = self.levels[led];

        if level > 0 {
            self.rgb[led].set_high();
            let on_time = level as u64 * self.tick_time;
            Timer::after_micros(on_time).await;
            self.rgb[led].set_low();
        }

        let level = LEVELS - level;
        if level > 0 {
            let off_time = level as u64 * self.tick_time;
            Timer::after_micros(off_time).await;
        }
    }

    /// Run loop for RGBs.
    ///
    /// 1. Get RGB levels and frame rate from global mutexes
    /// 2. Set local values for levels and frame rate
    /// 3. Run step for each LED pin
    pub async fn run(mut self) -> ! {
        loop {
            self.levels = get_rgb_levels().await;
            let frame_rate = get_frame_rate().await;
            self.tick_time = Self::frame_tick_time(frame_rate);

            for led in 0..3 {
                self.step(led).await;
            }
        }
    }
}
