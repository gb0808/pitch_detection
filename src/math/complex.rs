pub enum Sign {
    Positive,
    Negative,
}

#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct ComplexNumber {
    pub r: f32,
    pub i: f32
}

impl ComplexNumber {
    pub fn new(r: f32, i: f32) -> Self {
        Self {
            r,
            i,
        }
    }

    pub fn from(x: f32) -> Self {
        Self {
            r: x,
            i: 0.0,
        }
    }

    pub fn default() -> Self {
        Self::new(0.0, 0.0)
    }

    pub fn get_magnitude(&self) -> f32 {
        ((self.r * self.r) + (self.i * self.i)).sqrt()
    }

    pub fn get_phase(&self) -> f32 {
        (self.i / self.r).atan()
    }

    pub fn add(&self, other: &Self) -> ComplexNumber {
        Self {
            r: self.r + other.r,
            i: self.i + other.i,
        }
    }

    pub fn sub(&self, other: &Self) -> ComplexNumber {
        Self {
            r: self.r - other.r,
            i: self.i - other.i,
        }
    }

    pub fn mul(&self, other: &Self) -> ComplexNumber {
        Self {
            r: self.r * other.r - self.i * other.i,
            i: self.r * other.i + self.i * other.r,
        }
    }
}

impl PartialEq for ComplexNumber {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.i == other.i
    }
}

pub fn _e(i: Sign, x: f32) -> ComplexNumber {
    if let Sign::Negative = i {
        return ComplexNumber::new(x.cos(), x.sin() * -1.0)
    }
    ComplexNumber::new(x.cos(), x.sin())
}