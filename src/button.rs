use embedded_hal::digital::blocking::InputPin;
use std::time::Instant;

pub struct Button<InputPin> {
    pin: InputPin,
    button_state: ButtonState,
    click_time: Instant,
}

// TODO Make normal / long click times configurable or a constant outside the code
pub enum ButtonEvent {
    NormalClick,
    LongClick,
}

enum ButtonState {
    Released,
    Pressed,
}

impl<T: InputPin> Button<T> {
    pub fn new(pin: T) -> Self {
        let button_state = ButtonState::Released;
        let click_time = Instant::now();

        Self {
            pin,
            button_state,
            click_time,
        }
    }

    pub fn poll(&mut self) -> Option<ButtonEvent> {
        match self.button_state {
            ButtonState::Released => {
                if self.pin.is_low().unwrap() {
                    self.button_state = ButtonState::Pressed;
                    self.click_time = Instant::now();
                }
            }
            ButtonState::Pressed => {
                if !self.pin.is_low().unwrap() {
                    self.button_state = ButtonState::Released;
                    let press_duration = Instant::now().duration_since(self.click_time).as_millis();

                    if press_duration < 500 {
                        return Some(ButtonEvent::NormalClick);
                    } else {
                        return Some(ButtonEvent::LongClick);
                    }
                }
            }
        }

        None
    }
}
