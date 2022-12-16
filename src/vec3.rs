use std::ops::{Neg, Index, IndexMut, AddAssign, MulAssign, DivAssign};

/// A 3d vector struct
pub struct vec3 {
    pub e: [f64; 3]
}

impl vec3 {
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
}

/// Negation. Equivalent to -1 * vector
impl Neg for vec3 {
    type Output = [f64; 3];

    fn neg(self) -> Self::Output {
        return [-self.e[0], -self.e[1], -self.e[2]]
    }
}

/// Immutable element access operator
impl Index<usize> for vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        return &self.e[index]
    }
}

/// Mutable element access operator
impl IndexMut<usize> for vec3 {
    fn index_mut(&mut self, index: usize) -> &mut f64 {
        return &mut self.e[index]
    }
}

impl AddAssign for vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            e: [self.x() + other.x(), self.y() + other.y(), self.z() + other.z()]
        };
    }
}

impl MulAssign<f64> for vec3 {
    fn mul_assign(&mut self, other: f64) {
        *self = Self {
            e: [self.x() + other, self.y() + other, self.z() + other]
        };
    }
}

impl DivAssign<f64> for vec3 {
    fn div_assign(&mut self, other: f64) {
        *self *= 1.0 / other;
    }
}