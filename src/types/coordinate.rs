use std::ops::*;

#[derive(Copy, Clone)]
pub struct Coordinate {
    pub x: f64,
    pub y: f64
}

impl Add for Coordinate {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {x: self.x + other.x, y: self.y + other.y}
    }
}

impl Add<f64> for Coordinate {
    type Output = Self;

    fn add(self, other: f64) -> Self {
        Self {x: self.x + other, y: self.y + other}
    }
}

impl AddAssign for Coordinate {
    fn add_assign(&mut self, other: Self) {
        *self = Self {x: self.x + other.x, y: self.y + other.y}
    }
}

impl Sub for Coordinate {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {x: self.x - other.x, y: self.y - other.y}
    }
}

impl Sub<f64> for Coordinate {
    type Output = Self;

    fn sub(self, other: f64) -> Self {
        Self {x: self.x - other, y: self.y - other}
    }
}

impl SubAssign for Coordinate {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {x: self.x - other.x, y: self.y - other.y}
    }
}

impl Mul for Coordinate {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {x: self.x * other.x, y: self.y * other.y}
    }
}

impl Mul<f64> for Coordinate {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self {x: self.x * other, y: self.y * other}
    }
}

impl MulAssign for Coordinate {
    fn mul_assign(&mut self, other: Self) {
        *self = Self {x: self.x * other.x, y: self.y * other.y}
    }
}

impl Div for Coordinate {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {x: self.x / other.x, y: self.y / other.y}
    }
}

impl Div<f64> for Coordinate {
    type Output = Self;

    fn div(self, other: f64) -> Self {
        Self {x: self.x / other, y: self.y / other}
    }
}

impl DivAssign for Coordinate {
    fn div_assign(&mut self, other: Self) {
        *self = Self {x: self.x / other.x, y: self.y / other.y}
    }
}

impl Rem for Coordinate {
    type Output = Self;

    fn rem(self, other: Self) -> Self {
        Self {x: self.x % other.x, y: self.y % other.y}
    }
}

impl Rem<f64> for Coordinate {
    type Output = Self;

    fn rem(self, other: f64) -> Self {
        Self {x: self.x % other, y: self.y % other}
    }
}

impl RemAssign for Coordinate {
    fn rem_assign(&mut self, other: Self) {
        *self = Self {x: self.x % other.x, y: self.y % other.y}
    }
}

impl Neg for Coordinate {
    type Output = Self;

    fn neg(self) -> Self {
        Self {x: -self.x, y: -self.y}
    }
}