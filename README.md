# horizon

A modular weather visualization app. What's on your horizon?

Access the web app here: https://neuronull.github.io/horizon/

![Screenshot](./doc-assets/horizon-screenshot.png)

:pushpin: See the [project roadmap](ROADMAP.md) for planned features and milestones.

:art: Built with [egui](https://github.com/emilk/egui)

:zap: powered by [Pirate Weather](https://pirateweather.net/en/latest/)

:rocket: deployed via [GitHub Pages](https://pages.github.com/).

## Desktop Build

The app supports both web and native desktop builds. Currently native desktop requires building from source.

### Dependencies

- Rust
- Pirate Weather API key

#### Installing Rust

https://www.rust-lang.org/tools/install

#### Obtaining Pirate Weather API key

https://pirate-weather.apiable.io/products/weather-data/Plans


#### Build and run
1. Clone or download the repo
2. Execute the following command from the repository checkout directory
   ```
   PIRATEWEATHER_API_KEY=<YOUR_API_KEY> cargo run
   ```

## Testing

Run the tests locally with `cargo test` on a checkout of the source following the instructions in [Build](#build)

## License

This project uses AGPL-3.0 to ensure that it remains open and non-commercial.
