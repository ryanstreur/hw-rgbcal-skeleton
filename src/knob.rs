use crate::*;

/// Implementation type for Saadc:
/// Successive Approximation Analog to Digital Converter
pub type Adc = saadc::Saadc<'static, 1>;

/// Struct wrapper for ADC
pub struct Knob(Adc);

impl Knob {
    /// Create new Knob
    pub async fn new(adc: Adc) -> Self {
        adc.calibrate().await;
        Self(adc)
    }

    /// Read value from Knob
    pub async fn measure(&mut self) -> u32 {
        let mut buf = [0];
        self.0.sample(&mut buf).await;
        let raw = buf[0].clamp(0, 0x7fff) as u16;
        let scaled = raw as f32 / 10_000.0;
        let result = ((LEVELS + 2) as f32 * scaled - 2.0)
            .clamp(0.0, (LEVELS - 1) as f32)
            .floor();
        result as u32
    }
}
