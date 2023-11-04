use context_attribute::context;
use serde::{Deserialize, Serialize};
use types::sensor_data::SensorData;
use hardware::SpeakerInterface;
use types::audio::{Sound, SpeakerRequest};

#[derive(Deserialize, Serialize)]
pub struct BatteryWarning {
}

#[context]
pub struct CreationContext {}

#[context]
pub struct CycleContext {
    sensor_data : Input<SensorData, "sensor_data">,
    hardware_interface : HardwareInterface,
}

#[context]
#[derive(Default)]
pub struct MainOutputs {} // TODO this module has no output. How to handle? Will this get scheduled correctly?

impl BatteryWarning {
    pub fn new(_context: CreationContext) {}

    pub fn cycle(
        &mut self,
        context: CycleContext<impl SpeakerInterface>,
    ) {
        // TODO: Only execute every 30s or so
        let battery_level = context.sensor_data.battery_state.charge;
        if battery_level < 42f32 { // TODO some config value probably
            // TODO: Replace audio file
            context.
                hardware_interface.
                write_to_speakers(SpeakerRequest::PlaySound { sound: Sound::FalsePositiveDetected});
        }
    }
}