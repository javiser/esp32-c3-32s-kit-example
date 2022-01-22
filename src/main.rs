use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use std::thread;
use std::time::Duration;

use anyhow::*;
use log::*;

use embedded_hal::digital::blocking::OutputPin;
use embedded_hal::pwm::blocking::PwmPin;
use esp_idf_hal::peripherals::Peripherals;

use esp_idf_hal::ledc::{config::TimerConfig, Channel, Timer};
use esp_idf_hal::prelude::*;

use crate::button::{Button, ButtonEvent, WhenPressed};
use crate::rgb::RainbowRGB;
mod button;
mod rgb;

pub enum ShowMode {
    Off,
    AmberLight,
    WhiteLight,
    RGBRainbow,
}

fn main() -> Result<()> {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    // Declare all pins variables
    let peripherals = Peripherals::take().unwrap();
    let pins = peripherals.pins;

    let mut amber_led = pins.gpio18.into_output()?;
    let mut white_led = pins.gpio19.into_output()?;

    let red_led = pins.gpio3.into_output()?;
    let green_led = pins.gpio4.into_output()?;
    let blue_led = pins.gpio5.into_output()?;
    let config = TimerConfig::default().frequency(25.kHz().into());
    let timer = Timer::new(peripherals.ledc.timer0, &config)?;
    let mut red_pwm = Channel::new(peripherals.ledc.channel0, &timer, red_led)?;
    let mut green_pwm = Channel::new(peripherals.ledc.channel1, &timer, green_led)?;
    let mut blue_pwm = Channel::new(peripherals.ledc.channel2, &timer, blue_led)?;

    let mut button = Button::new(pins.gpio9.into_input()?, WhenPressed::Low);

    // Variables needed to control the lights show
    let mut show_mode = ShowMode::AmberLight;
    let mut rgb = RainbowRGB::new();

    info!("ESP32-C3-32S-Kit example started!");
    loop {
        match show_mode {
            // TODO This is still a very ugly implementation from a Rust-learner ;)
            ShowMode::Off => {
                amber_led.set_low()?;
                white_led.set_low()?;
                red_pwm.set_duty(0)?;
                green_pwm.set_duty(0)?;
                blue_pwm.set_duty(0)?;
            }

            ShowMode::AmberLight => {
                amber_led.set_high()?;
                red_pwm.set_duty(0)?;
                green_pwm.set_duty(0)?;
                blue_pwm.set_duty(0)?;
            }

            ShowMode::WhiteLight => {
                amber_led.set_low()?;
                white_led.set_high()?;
            }

            ShowMode::RGBRainbow => {
                white_led.set_low()?;
                rgb.next_color();
                red_pwm.set_duty(rgb.get_r() as u32)?;
                green_pwm.set_duty(rgb.get_g() as u32)?;
                blue_pwm.set_duty(rgb.get_b() as u32)?;
            }
        }
        match button.poll() {
            Some(ButtonEvent::ShortPress) => {
                info!("Button press: change show mode");
                show_mode = match show_mode {
                    ShowMode::Off => ShowMode::AmberLight,
                    ShowMode::AmberLight => ShowMode::WhiteLight,
                    ShowMode::WhiteLight => ShowMode::RGBRainbow,
                    ShowMode::RGBRainbow => ShowMode::AmberLight,
                }
            }

            Some(ButtonEvent::LongPress) => {
                info!("Long button press: turn off the LEDs");
                show_mode = ShowMode::Off;
            }

            _ => {}
        }

        thread::sleep(Duration::from_millis(10));
    }
}
