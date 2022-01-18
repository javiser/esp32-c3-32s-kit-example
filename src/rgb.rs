pub struct RainbowRGB {
    r: u8,
    g: u8,
    b: u8,
}

impl RainbowRGB {
    pub fn new() -> Self {
        let r = 255;
        let g = 0;
        let b = 0;

        Self { r, g, b }
    }

    pub fn get_r(&mut self) -> u8 {
        self.r
    }

    pub fn get_g(&mut self) -> u8 {
        self.g
    }

    pub fn get_b(&mut self) -> u8 {
        self.b
    }

    pub fn next_color(&mut self) {
        if self.r > 0 && self.b == 0 {
            self.r -= 1;
            self.g += 1;
        } else if self.g > 0 && self.r == 0 {
            self.g -= 1;
            self.b += 1;
        } else {
            self.b -= 1;
            self.r += 1;
        }
    }
}
