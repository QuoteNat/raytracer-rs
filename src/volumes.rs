use std::sync::Arc;

use crate::{
    aabb::AABB,
    hit::{HitRecord, Hittable},
    lights::Light,
    materials::Material,
    ray::Ray,
    scene::Scene,
    texture::Texture,
    utility::random_float_1,
    vector::{random_in_unit_sphere, Color, Vec3},
};

struct Isotropic {
    albedo: Arc<dyn Texture>,
    absorbance: f64,
}

impl Isotropic {
    pub fn new(a: &Arc<dyn Texture>, absorb: f64) -> Isotropic {
        Isotropic {
            albedo: Arc::clone(a),
            absorbance: absorb,
        }
    }
}

impl Material for Isotropic {
    // no idea if this will work for volumes but it just seems like standard diffuse code so idk
    fn apply(&self, r_in: &Ray, rec: &HitRecord, scene: &Scene, depth: i32) -> Color {
        // lambertian light contribution
        let cr =
            self.albedo.value(&rec.uv, &rec.p) * scene.lights.apply(r_in, rec, scene).contribution;

        let scattered = Ray::new(rec.p, random_in_unit_sphere());

        let scattered_color = scene.ray_color(&scattered, depth);

        return cr * self.absorbance
            + (1.0 - self.absorbance) * self.albedo.value(&rec.uv, &rec.p) * scattered_color;
    }
}

pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    phase_function: Arc<dyn Material>,
    neg_inv_density: f64,
}

impl ConstantMedium {
    pub fn new(
        b: &Arc<dyn Hittable>,
        d: f64,
        a: &Arc<dyn Texture>,
        absorbance: f64,
    ) -> ConstantMedium {
        ConstantMedium {
            boundary: Arc::clone(b),
            neg_inv_density: -1.0 / d,
            phase_function: Arc::new(Isotropic::new(a, absorbance)),
        }
    }
}

impl Hittable for ConstantMedium {
    fn bounding_box(&self) -> AABB {
        return self.boundary.bounding_box();
    }

    fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let enable_debug = false;
        let debugging = enable_debug && random_float_1() < 0.00001;

        let mut rec1 = match self.boundary.hit(r, -f64::INFINITY, f64::INFINITY) {
            Some(rec) => rec,
            None => {
                return None;
            }
        };

        let mut rec2 = match self.boundary.hit(r, rec1.t + 0.0001, f64::INFINITY) {
            Some(rec) => rec,
            None => {
                return None;
            }
        };

        if debugging {
            println!("t_min={}, t_max={}", rec1.t, rec2.t);
        }

        if rec1.t < t_min {
            rec1.t = t_min;
        }

        if rec2.t > t_max {
            rec2.t = t_max;
        }

        if rec1.t >= rec2.t {
            return None;
        }

        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }

        let ray_length = r.direction.length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * f64::ln(random_float_1());

        if hit_distance > distance_inside_boundary {
            return None;
        }

        let t = rec1.t + hit_distance / ray_length;
        let p = r.at(t);

        if debugging {
            println!("hit_distance = {}", hit_distance);
            println!("rec.t = {}", t);
            println!("rec.p = {}", p.to_string());
        }

        let normal = Vec3::new(1.0, 0.0, 0.0); // arbitrary
        let front_face = true; // also arbitrary

        Some(HitRecord {
            p,
            normal,
            material: Arc::clone(&self.phase_function),
            t,
            front_face,
            uv: rec1.uv, // book doesn't cover how to set the uv pointer for this, so I just use rec1's uv
        })
    }
}
