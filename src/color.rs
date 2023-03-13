#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub fn red(&self) -> f64 {
        self.r as f64 / 255.0 as f64
    }

    pub fn green(&self) -> f64 {
        self.g as f64 / 255.0 as f64
    }

    pub fn blue(&self) -> f64 {
        self.b as f64 / 255.0 as f64
    }

    pub fn alpha(&self) -> f64 {
        self.a as f64 / 255.0 as f64
    }

    pub fn red_u8(&self) -> u8 {
        self.r
    }

    pub fn green_u8(&self) -> u8 {
        self.g
    }

    pub fn blue_u8(&self) -> u8 {
        self.b
    }

    pub fn alpha_u8(&self) -> u8 {
        self.a
    }

    pub fn from_hex(x: u32) -> Color {
        let r: u8 = ((x & 0xFF000000) >> 8 * 3).try_into().unwrap();
        let g: u8 = ((x & 0x00FF0000) >> 8 * 2).try_into().unwrap();
        let b: u8 = ((x & 0x0000FF00) >> 8 * 1).try_into().unwrap();
        let a: u8 = ((x & 0x000000FF) >> 8 * 0).try_into().unwrap();

        Color { r, g, b, a }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_from_u32() {
        assert_eq!(Color::from_hex(0xFFFFFFFF), Color::new(255, 255, 255, 255));
    }
}
