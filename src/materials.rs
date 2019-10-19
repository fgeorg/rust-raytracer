use crate::hittable::*;
use crate::ray::*;
use crate::vec3::*;

use rand::rngs::ThreadRng;
use rand::Rng;

fn random_in_unit_sphere(rng: &mut ThreadRng) -> Vec3 {
    let mut v = Vec3(1.0, 1.0, 1.0);
    while v.squared_length() > 1.0 {
        v = Vec3(
            2.0 * rng.gen::<f32>() - 1.0,
            2.0 * rng.gen::<f32>() - 1.0,
            2.0 * rng.gen::<f32>() - 1.0,
        );
    }
    v
}

pub trait Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, rng: &mut ThreadRng) -> (Ray, Vec3) {
        let new_ray = Ray {
            pos: ray.point_at_t(hit_record.t),
            dir: hit_record.normal + random_in_unit_sphere(rng),
        };
        return (new_ray, Vec3(0.5, 0.5, 0.5));
    }
}

pub struct DiffuseMaterial {
    pub albedo: Vec3,
}

impl Material for DiffuseMaterial {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, rng: &mut ThreadRng) -> (Ray, Vec3) {
        let new_ray = Ray {
            pos: ray.point_at_t(hit_record.t),
            dir: hit_record.normal + random_in_unit_sphere(rng),
        };
        return (new_ray, self.albedo);
    }
}

pub struct MetalMaterial {
    pub albedo: Vec3,
    pub fuzz: f32,
}

impl Material for MetalMaterial {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, rng: &mut ThreadRng) -> (Ray, Vec3) {
        let new_ray = Ray {
            pos: ray.point_at_t(hit_record.t),
            dir: reflect(&ray.dir.normalized(), &hit_record.normal)
                + self.fuzz * random_in_unit_sphere(rng),
        };
        return (new_ray, self.albedo);
    }
}

pub struct GlassMaterial {
    pub albedo: Vec3,
    pub ref_idx: f32,
}

impl Material for GlassMaterial {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord, rng: &mut ThreadRng) -> (Ray, Vec3) {
        let outward_normal: Vec3;
        let ni_over_nt: f32;
        let cosine: f32;

        if ray.dir.dot(hit_record.normal) > 0.0 {
            outward_normal = -hit_record.normal;
            ni_over_nt = self.ref_idx;
            cosine = self.ref_idx * ray.dir.dot(hit_record.normal) / ray.dir.length();
        } else {
            outward_normal = hit_record.normal;
            ni_over_nt = 1.0 / self.ref_idx;
            cosine = -ray.dir.dot(hit_record.normal) / ray.dir.length();
        }

        if rng.gen::<f32>() > schlick(cosine, self.ref_idx) {
            // borrowed this code from https://github.com/perliedman/raytracing-in-one-weekend/blob/master/src/material.rs
            match refract(&ray.dir, &outward_normal, ni_over_nt) {
                Some(refraction) => {
                    return (
                        Ray {
                            pos: ray.point_at_t(hit_record.t),
                            dir: refraction,
                        },
                        self.albedo,
                    );
                }
                None => {}
            }
        }

        (
            Ray {
                pos: ray.point_at_t(hit_record.t),
                dir: reflect(&ray.dir.normalized(), &hit_record.normal),
            },
            self.albedo,
        )
    }
}

// borrowed this code from https://github.com/perliedman/raytracing-in-one-weekend/blob/master/src/material.rs
fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - 2.0 * v.dot(*n) * *n
}

// borrowed this code from https://github.com/perliedman/raytracing-in-one-weekend/blob/master/src/material.rs
fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let uv = v.normalized();
    let dt = uv.dot(*n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        Some(ni_over_nt * (uv - *n * dt) - discriminant.sqrt() * *n)
    } else {
        None
    }
}

// borrowed this code from https://github.com/perliedman/raytracing-in-one-weekend/blob/master/src/material.rs
fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0sq = r0 * r0;
    r0sq + (1.0 - r0sq) * (1.0 - cosine).powf(5.0)
}
