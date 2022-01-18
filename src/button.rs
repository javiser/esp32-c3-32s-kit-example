use embedded_hal::digital::blocking::InputPin;
use std::time::Instant;

pub struct Button<InputPin> {
    pin: InputPin,
    button_state: ButtonState,
    press_time: Instant,
    when_pressed: WhenPressed,
    long_press_thd_ms: u128,
}

pub enum ButtonEvent {
    ShortPress,
    LongPress,
}

enum ButtonState {
    Released,
    Pressed,
}

pub enum WhenPressed {
    High,
    Low,
}

impl<T: InputPin> Button<T> {
    pub fn new(pin: T, when_pressed: WhenPressed) -> Self {
        let button_state = ButtonState::Released;
        let press_time = Instant::now();
        let long_press_thd_ms = 500;

        Self {
            pin,
            button_state,
            press_time,
            when_pressed,
            long_press_thd_ms,
        }
    }

    fn is_pressed(&mut self) -> bool {
        match self.when_pressed {
            WhenPressed::High => self.pin.is_high().unwrap(),
            WhenPressed::Low => self.pin.is_low().unwrap(),
        }
    }

    pub fn poll(&mut self) -> Option<ButtonEvent> {
        match self.button_state {
            ButtonState::Released => {
                if self.is_pressed() {
                    self.button_state = ButtonState::Pressed;
                    self.press_time = Instant::now();
                }
            }
            ButtonState::Pressed => {
                if !self.is_pressed() {
                    self.button_state = ButtonState::Released;
                    let press_duration = Instant::now().duration_since(self.press_time).as_millis();

                    if press_duration < self.long_press_thd_ms {
                        return Some(ButtonEvent::ShortPress);
                    } else {
                        return Some(ButtonEvent::LongPress);
                    }
                }
            }
        }

        None
    }

    pub fn set_long_press_thd_ms(&mut self, new_thd_ms:u128) {
        self.long_press_thd_ms = new_thd_ms
    }
}
