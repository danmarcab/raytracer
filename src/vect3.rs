use std::ops::{Add, Sub, Mul, Neg};

#[derive(Copy, Clone, Debug)]
pub struct Vect3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}


impl Vect3 {
    pub fn zero() -> Vect3 {
        Vect3::all(0.0)
    }

    pub fn all(v: f64) -> Vect3 {
        Vect3 { x: v, y: v, z: v }
    }

    pub fn length_sq(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        self.length_sq().sqrt()
    }

    pub fn normalize(&self) -> Vect3 {
        let inv_length = self.length().recip();

        *self * inv_length
    }

    pub fn dot(&self, other: &Vect3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl Add for Vect3 {
    type Output = Vect3;

    fn add(self, other: Vect3) -> Vect3 {
        Vect3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vect3 {
    type Output = Vect3;

    fn sub(self, other: Vect3) -> Vect3 {
        Vect3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Neg for Vect3 {
    type Output = Vect3;

    fn neg(self) -> Vect3 {
        Vect3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Mul for Vect3 {
    type Output = Vect3;

    fn mul(self, other: Vect3) -> Vect3 {
        Vect3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Mul<f64> for Vect3 {
    type Output = Vect3;

    fn mul(self, k: f64) -> Vect3 {
        Vect3 {
            x: k * self.x,
            y: k * self.y,
            z: k * self.z,
        }
    }
}

impl Mul<Vect3> for f64 {
    type Output = Vect3;

    fn mul(self, v: Vect3) -> Vect3 {
        Vect3 {
            x: self * v.x,
            y: self * v.y,
            z: self * v.z,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero() {
        let vec = Vect3::zero();
        assert_eq!(vec.x, 0.0);
        assert_eq!(vec.y, 0.0);
        assert_eq!(vec.z, 0.0);
    }
}
