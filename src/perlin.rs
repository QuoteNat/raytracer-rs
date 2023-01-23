use crate::{
    utility::{random_float, random_float_1, random_int},
    vector::Point3,
};

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

        for _ in 0..point_count {
            ranfloat.push(random_float_1());
        }

        let perm_x = Perlin::perlin_generate_perm(point_count);
        let perm_y = Perlin::perlin_generate_perm(point_count);
        let perm_z = Perlin::perlin_generate_perm(point_count);

        Perlin {
            point_count,
            ranfloat,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, p: &Point3) -> f64 {
        let u = p.x() - f64::floor(p.x());
        let v = p.y() - f64::floor(p.y());
        let w = p.z() - f64::floor(p.z());
        let u = u * u * (3.0 - 2.0 * u);
        let v = v * v * (3.0 - 2.0 * v);
        let w = w * w * (3.0 - 2.0 * w);

        let i = f64::floor(p.x()) as i32;
        let j = f64::floor(p.y()) as i32;
        let k = f64::floor(p.z()) as i32;
        let mut c = [[[0.0; 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.ranfloat[self.perm_x[((i + di as i32) & 255) as usize]
                        ^ self.perm_y[((j + dj as i32) & 255) as usize]
                        ^ self.perm_z[((k + dk as i32) & 255) as usize]];
                }
            }
        }

        return Perlin::trilinear_interp(&c, u, v, w);
    }

    fn trilinear_interp(c: &[[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let fi = i as f64;
                    let fj = j as f64;
                    let fk = k as f64;
                    accum += (fi * u + (1.0 - fi) * (1.0 - u))
                        * (fj * v + (1.0 - fj) * (1.0 - v))
                        * (fk * w + (1.0 - fk) * (1.0 - w))
                        * c[i][j][k];
                }
            }
        }

        return accum;
    }

    fn perlin_generate_perm(point_count: usize) -> Vec<usize> {
        let mut p: Vec<usize> = Vec::new();

        for i in 0..point_count {
            p.push(i);
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
