use crate::{utility::{random_float_1, random_int}, vector::Point3};

pub struct Perlin {
    point_count: usize,
    ranfloat: Vec<f64>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

impl Perlin {
    pub fn new() -> Perlin {
        let point_count = 256;
        let mut ranfloat: Vec<f64> = Vec::new();

        for i in 0..point_count {
            ranfloat[i] = random_float_1();
        }

        let perm_x = Perlin::perlin_generate_perm(point_count);
        let perm_y = Perlin::perlin_generate_perm(point_count);
        let perm_z = Perlin::perlin_generate_perm(point_count);

        Perlin {
            point_count,
            ranfloat,
            perm_x,
            perm_y,
            perm_z
        }
    }

    pub fn noise(&self, p: &Point3) -> f64 {
        let i = (4.0 * p.x()) as i32 & 255;
        let j = (4.0 * p.y()) as i32 & 255;
        let k = (4.0 * p.z()) as i32 & 255;

        return self.ranfloat[self.perm_x[i as usize] ^ self.perm_y[j as usize] ^ self.perm_z[k as usize]];
    }

    fn perlin_generate_perm(point_count: usize) -> Vec<usize> {
        let mut p: Vec<usize> = Vec::new();

        for i in 0..point_count {
            p[i] = i as usize;
        }

        Perlin::permute(&mut p, point_count);

        return p;
    }

    fn permute(p: &mut Vec<usize>, n: usize) {
        for i in (0..n).rev() {
            let target = random_int(0, i as i32) as usize;
            let tmp = p[i];
            p[i] = p[target];
            p[target] = tmp;
        }
    }
}