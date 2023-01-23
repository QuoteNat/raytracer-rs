use crate::utility::{clamp, random_float, random_float_1};
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub};
/// A 3d vector struct
#[derive(Copy, Clone)]
pub struct Vec3 {
    pub e: [f64; 3],
}

impl Vec3 {
    /// Creates a new Vec3 with values (x, y, z)
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { e: [x, y, z] }
    }
    /// Returns x value of the vector
    pub fn x(&self) -> f64 {
        return self.e[0];
    }

    /// Returns y value of the vector
    pub fn y(&self) -> f64 {
        return self.e[1];
    }

    /// Returns z value of the vector
    pub fn z(&self) -> f64 {
        return self.e[2];
    }

    /// magnitude/length squared
    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
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

    pub fn to_string(&self) -> String {
        format!("({}, {}, {})", self.e[0], self.e[1], self.e[2])
    }
}

/// Negation. Equivalent to -1 * vector
impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        return Vec3 {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        };
    }
}

/// Immutable element access operator
impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        return &self.e[index];
    }
}

/// Mutable element access operator
impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut f64 {
        return &mut self.e[index];
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            e: [
                self.x() + other.x(),
                self.y() + other.y(),
                self.z() + other.z(),
            ],
        };
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        *self = Self {
            e: [self.x() * other, self.y() * other, self.z() * other],
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
            e: [
                self.e[0] + other.e[0],
                self.e[1] + other.e[1],
                self.e[2] + other.e[2],
            ],
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            e: [
                self.e[0] - other.e[0],
                self.e[1] - other.e[1],
                self.e[2] - other.e[2],
            ],
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            e: [
                self.e[0] * other.e[0],
                self.e[1] * other.e[1],
                self.e[2] * other.e[2],
            ],
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self {
            e: [self.e[0] * other, self.e[1] * other, self.e[2] * other],
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [other.e[0] * self, other.e[1] * self, other.e[2] * self],
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, other: f64) -> Self::Output {
        (1.0 / other) * self
    }
}

/// Dot product of two vectors
pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
    u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
}

/// Cross product of two vectors
#[allow(dead_code)]
pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3 {
        e: [
            u.e[1] * v.e[2] - u.e[2] * v.e[1],
            u.e[2] * v.e[0] - u.e[0] * v.e[2],
            u.e[0] * v.e[1] - u.e[1] * v.e[0],
        ],
    }
}

/// Unit vector
pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}

pub fn random_vec_1() -> Vec3 {
    Vec3 {
        e: [random_float_1(), random_float_1(), random_float_1()],
    }
}

pub fn random_vec(min: f64, max: f64) -> Vec3 {
    Vec3 {
        e: [
            random_float(min, max),
            random_float(min, max),
            random_float(min, max),
        ],
    }
}

/// Return a random point in the unit sphere
pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = random_vec(-1.0, 1.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

/// Random unit vector
pub fn random_unit_vector() -> Vec3 {
    unit_vector(random_in_unit_sphere())
}

/// Reflect a vector around an normal
pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - *n * dot(v, n) * 2.0
}

pub fn zero_vec() -> Vec3 {
    Vec3 { e: [0.0, 0.0, 0.0] }
}

pub fn quick_vec(x: f64, y: f64, z: f64) -> Vec3 {
    Vec3 { e: [x, y, z] }
}

/// Ray refraction using Snell's law
pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
    // dereference uv and n for math operations
    let uv = *uv;
    let n = *n;
    // Get the angle between -uv and the normal
    let cos_theta = f64::min(dot(&(-uv), &n), 1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -f64::sqrt(f64::abs(1.0 - r_out_perp.length_squared())) * n;

    r_out_perp + r_out_parallel
}

pub fn write_color(pixel_color: Color, samples_per_pixel: i32) {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    // Divide the color by the number of samples
    let scale = 1.0 / samples_per_pixel as f64;
    let r = (r * scale).sqrt();
    let g = (g * scale).sqrt();
    let b = (b * scale).sqrt();

    print!(
        "{} {} {}\n",
        (256.0 * clamp(r, 0.0, 0.999)) as i32,
        (256.0 * clamp(g, 0.0, 0.999)) as i32,
        (256.0 * clamp(b, 0.0, 0.999)) as i32
    );
}

/// Clamp all values of a vec to the range [min, max]
pub fn vec_clamp(vec: Vec3, min: f64, max: f64) -> Vec3 {
    quick_vec(
        clamp(vec.x(), min, max),
        clamp(vec.y(), min, max),
        clamp(vec.z(), min, max),
    )
}
