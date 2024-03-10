use crate::*;

/// Type alias for the ADC.
pub type Adc = saadc::Saadc<'static, 1>;

/// Structure to represent the Knob which is then connected to ADC.
pub struct Knob(Adc);

impl Knob {
    /// Constructs the new Knob instance.
    ///
    /// This function initializes the new Knob instance by calibrating the ADC
    ///
    /// # Arguments
    ///
    /// * `adc` - The instance of the ADC
    ///
    /// # Returns
    ///
    /// A new knob instance
    pub async fn new(adc: Adc) -> Self {
        // Calibrate an ADC to get the measurments.
        adc.calibrate().await;
        // Return the knob instance.
        Self(adc)
    }

    /// Function to measure the position of the Knob.
    ///
    /// This function measures the position of the knob by taking the samples from the ADC
    /// and it scales to the given range of levels.
    ///
    /// # Returns
    ///
    /// THe position of the knob which represnts a u32 value in the range of levels.
    pub async fn measure(&mut self) -> u32 {
        // This buffer stores the ADC sample.
        let mut buf = [0];
        // Take the sample from the ADC.
        self.0.sample(&mut buf).await;
        // Convert the meaured raw ADC value and then scale the value.
        let raw = buf[0].clamp(0, 0x7fff) as u16;
        let scaled = raw as f32 / 10_000.0;
        // Finally map the scaled value to the range of the given levels.
        let result = ((LEVELS + 2) as f32 * scaled - 2.0)
            .clamp(0.0, (LEVELS - 1) as f32)
            .floor();
        result as u32
    }
}
