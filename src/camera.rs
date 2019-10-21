use crate::ray::*;
use crate::vec3::*;
use rand::rngs::ThreadRng;
use rand::Rng;

#[derive(Debug)]
pub struct Camera {
    pub origin: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lower_left: Vec3,
    pub lens_radius: f64,
    pub u: Vec3,
    pub v: Vec3,
}

impl Camera {
    fn random_in_unit_disk(rng: &mut ThreadRng) -> Vec3 {
        loop {
            let p = 2.0 * Vec3(rng.gen::<f64>() - 0.5, rng.gen::<f64>() - 0.5, 0.0);
            if p.squared_length() < 1.0 {
                return p;
            }
        }
    }

    pub fn new(
        origin: Vec3,
        look_at: Vec3,
        up: Vec3,
        fov: f64,
        aspect: f64,
        aperture: f64,
        focus: f64,
    ) -> Camera {
        let theta = fov * std::f64::consts::PI / 180.0;
        let half_width = (0.5 * theta).tan();
        let half_height = half_width / aspect;
        // let theta = vfov * std::f64::consts::PI / 180.0;
        // let half_height = (theta * 0.5).tan();
        // let half_width = aspect * half_height;
        let w = (origin - look_at).normalized();
        let u = up.cross(w).normalized();
        let v = w.cross(u).normalized();

        let focus_dist = focus * (look_at - origin).length();

        Camera {
            origin,
            horizontal: 2.0 * half_width * focus_dist * u,
            vertical: 2.0 * half_height * focus_dist * v,
            lower_left: origin
                - half_width * focus_dist * u
                - half_height * focus_dist * v
                - focus_dist * w,
            lens_radius: 0.5 * aperture,
            u,
            v,
        }
    }
    pub fn get_ray(&self, s: f64, t: f64, rng: &mut ThreadRng) -> Ray {
        let rd = self.lens_radius * Self::random_in_unit_disk(rng);
        let offset = self.u * rd.x() + self.v * rd.y();
        Ray {
            pos: self.origin + offset,
            dir: self.lower_left + s * self.horizontal + t * self.vertical - self.origin - offset,
        }
    }
}
