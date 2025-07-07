mod temperature;

trait Widget {}

#[derive(Default)]
pub struct Widgets {
    temperature: temperature::TemperatureWidget,
}
