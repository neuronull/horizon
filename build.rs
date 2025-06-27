fn main() {
    if let Ok(val) = std::env::var("PIRATEWEATHER_API_KEY") {
        println!("cargo:rustc-env=PIRATEWEATHER_API_KEY={}", val);
    }
}
