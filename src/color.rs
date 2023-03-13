#[derive(Clone, Copy)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
	Self {
	    r,
	    g,
	    b,
	    a,
	}
    }
    
    pub fn red(&self) -> u8 {
	self.r
    }
    
    pub fn green(&self) -> u8 {
	self.g
    }
    
    pub fn blue(&self) -> u8 {
	self.b
    }
    
    pub fn alpha(&self) -> u8 {
	self.a
    }
}
