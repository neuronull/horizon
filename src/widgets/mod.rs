mod current;
mod temperature;

trait Widget {}

#[derive(Default)]
pub struct Widgets {
    temperature: temperature::TemperatureWidget,
    current: current::CurrentWidget,
}
