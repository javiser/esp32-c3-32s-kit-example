# Rust on ESP-C3-32S-Kit board
A simple Rust example for the [ESP-C3-32S-Kit ESP32 WiFi+Bluetooth Development Board](https://www.waveshare.com/esp-c3-32s-kit.htm) with ESP32C3 chip from Waveshare. This example uses the `std` implementation.

**Highlights**:
- No external periphery needed, just the board
- Small but not just an empty "hello world" example
- Available custom bootloader and partition tables for this board
- Preconfigured runner, just call `cargo run`

**This example is still work in progress!** Current open tasks:
- [ ] Example builds and turns on both the amber and white built-in leds and the RGB led. It also has some basic polling of the built-in button
- [ ] Usage of built-in button to change the mode
- [ ] Control of built-in RGB LED using PWM

## Motivation and goals
This is my simple example to learn Rust beginning with the board ESP-C3-32S-Kit I purchased from Waveshare. It uses the built-in periphery (LEDs and button) to demonstrate basic concepts. 

## Preparation
- Install the nightly toolchain of Rust: `rustup toolchain install nightly`
- Make sure the toolchains are up to date: `rustup update`
- Switch to nightly: `rustup override set nightly`
- Install [ldproxy](https://crates.io/crates/embuild/ldproxy): `cargo install ldproxy`
- Install `espflash` and `espmonitor` crates: `cargo install espflash espmonitor`

## Build & run
To build the project (this step is optional)
```
cargo build
```
To build, flash and monitor the output (all in one step thanks to preconfigured runner, be aware that it assumes that you are using `/dev/ttyUSB0`):
```
cargo run
```
If you need more control or other parameters, you can use the `espflash` and `espmonitor` commands instead (see the [Credits and acknowledgment](#credits-and-acknowledgment) for more details on how to do this).

## Credits and acknowledgment
For this example I used the template in https://github.com/esp-rs/esp-idf-template to create the initial files. Furthermore this exmaple is heavily inspired by:
- https://github.com/fkohlgrueber/esp32c3-idf-led-example
- https://github.com/ivmarkov/rust-esp32-std-demo

You can check those for better instructions on how to use `espflash` and `espmonitor` and other details I don't cover here.

## Disclaimers
- I am a beginner at Rust and ESP32 boards. Don't take my implementation as best practice! I don't really understand every single line of code of my example, which I have partly copied from the template and other examples.
- Still, I am very grateful if you have comments or improvement proposals. Don't hesitate to open issues!
- I have tested this example using my own board. This might not work well with other ESP32C3 boards. Use this at your own risk.
- I believe that you don't need to install ESP_IDF as well (see https://docs.espressif.com/projects/esp-idf/en/latest/esp32c3/get-started/index.html), but I am not very sure about this yet.

## License
Copyright (c) 2022 javiser
`esp32-c3-32s-kit-example` is distributed under the terms of the MIT License.

See the [LICENSE](LICENSE) for license details.