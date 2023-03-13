use super::*;

pub struct Canvas {
    width: u64,
    height: u64,
    size: usize,
    pixels: Vec<Color>,
}

impl Canvas {
    pub fn new<U: Into<u64>>(w: U, h: U, p: Color) -> Self {
        let width: u64 = w.into();
        let height: u64 = h.into();
        let size = (width * height) as usize;

        Self {
            width,
            height,
            size,
            pixels: vec![p; size],
        }
    }

    fn get_linear(&self, i: usize) -> Option<Color> {
        match self.pixels.get(i) {
            Some(p) => Some(*p),
            None => None,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<Color> {
        self.get_linear(x * y)
    }

    pub fn dimensions(&self) -> (u64, u64) {
        (self.width, self.height)
    }
}
