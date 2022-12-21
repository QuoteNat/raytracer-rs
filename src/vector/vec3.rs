use std::ops::{Neg, Index, IndexMut, AddAssign, MulAssign, DivAssign, Mul, Add, Div, Sub};

/// A 3d vector struct
#[derive(Copy, Clone)]
pub struct Vec3 {
    pub e: [f64; 3]
}

impl Vec3 {
    /// Returns x value of the vector
    pub fn x(&self) -> f64 {
        return self.e[0]
    }

    /// Returns y value of the vector
    pub fn y(&self) -> f64 {
        return self.e[1]
    }

    /// Returns z value of the vector
    pub fn z(&self) -> f64 {
        return self.e[2]
    }

    /// magnitude/length squared
    pub fn length_squared(&self) -> f64 {
        self.e[0]*self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    /// magnitude/length
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    /// Returns true if all vector dimensions are less than 1e-8
    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.e[0].abs() < s && self.e[1].abs() < s && self.e[2].abs() < s
    }

    pub fn err_write(&self) {
        eprintln!("({}, {}, {})", self.e[0], self.e[1], self.e[2]);
    }

}

/// Negation. Equivalent to -1 * vector
impl Neg for Vec3 {
    type Output = [f64; 3];

    fn neg(self) -> Self::Output {
        return [-self.e[0], -self.e[1], -self.e[2]]
    }
}

/// Immutable element access operator
impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        return &self.e[index]
    }
}

/// Mutable element access operator
impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut f64 {
        return &mut self.e[index]
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            e: [self.x() + other.x(), self.y() + other.y(), self.z() + other.z()]
        };
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        *self = Self {
            e: [self.x() + other, self.y() + other, self.z() + other]
        };
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, other: f64) {
        *self *= 1.0 / other;
    }
}

// Type aliases
pub type Point3 = Vec3;
pub type Color = Vec3;

// Utlity functions!
impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            e: [self.e[0] + other.e[0], self.e[1] + other.e[1], self.e[2] + other.e[2]]
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            e: [self.e[0] - other.e[0], self.e[1] - other.e[1], self.e[2] - other.e[2]]
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            e: [self.e[0] * other.e[0], self.e[1] * other.e[1], self.e[2] * other.e[2]]
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self {
            e: [self.e[0] * other, self.e[1] * other, self.e[2] * other]
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul (self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [other.e[0] * self, other.e[1] * self, other.e[2] * self]
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div (self, other: f64) -> Self::Output {
        (1.0/other) * self
    }
}

/// Dot product of two vectors
pub fn dot(u: &Vec3, v: &Vec3) -> f64{
    u.e[0] * v.e[0]
    + u.e[1] * v.e[1]
    + u.e[2] * v.e[2]
}

/// Cross product of two vectors
#[allow(dead_code)]
pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3 {
        e: [u.e[1] * v.e[2] - u.e[2] * v.e[1],
        u.e[2] * v.e[0] - u.e[0] * v.e[2],
        u.e[0] * v.e[1] - u.e[1] * v.e[0]]
    }
}

/// Unit vector
pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}