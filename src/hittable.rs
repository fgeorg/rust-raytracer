use crate::materials::Material;
use crate::ray::*;
use crate::vec3::*;

pub struct HitRecord<'a> {
    pub t: f32,
    pub normal: Vec3,
    pub material: Option<&'a (dyn Material + Send + Sync)>,
}

impl HitRecord<'_> {
    fn new_miss() -> Self {
        HitRecord {
            t: -1.0,
            normal: Vec3(0.0, 0.0, 0.0),
            material: None,
        }
    }
}

pub trait Hittable {
    fn hit(&self, _ray: &Ray, _t_min: f32, _t_max: f32) -> HitRecord {
        HitRecord::new_miss()
    }
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Box<dyn Material + Send + Sync>,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> HitRecord {
        let oc = ray.pos - self.center;
        let a = ray.dir.dot(ray.dir);
        let b = 2.0 * oc.dot(ray.dir);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return HitRecord::new_miss();
        }
        let mut t = (-b - discriminant.sqrt()) / (2.0 * a);
        if t < t_min || t > t_max {
            t = (-b + discriminant.sqrt()) / (2.0 * a);
        }
        if t < t_min || t > t_max {
            return HitRecord::new_miss();
        }
        let normal = (ray.point_at_t(t) - self.center).normalized();
        HitRecord {
            t,
            normal,
            material: Some(&*self.material),
        }
    }
}

pub struct HittableList {
    list: Vec<Box<dyn Hittable + Send + Sync>>,
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> HitRecord {
        let mut closest = HitRecord::new_miss();
        for h in &self.list {
            let hit_record = h.hit(ray, t_min, t_max);
            if hit_record.t > 0.0 {
                if closest.t < 0.0 || hit_record.t < closest.t {
                    closest = hit_record
                }
            }
        }
        closest
    }
}

impl HittableList {
    pub fn new() -> Self {
        Self { list: Vec::new() }
    }

    pub fn push<T: Hittable + 'static + Send + Sync>(&mut self, h: T) -> () {
        self.list.push(Box::new(h))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    struct TestMaterial {}
    impl Material for TestMaterial {}

    #[test]
    fn sphere_is_hittable() {
        let s = Sphere {
            center: Vec3(0.0, 0.0, -1.0),
            radius: 0.5,
            material: Box::new(TestMaterial {}),
        };
        let r = Ray {
            pos: Vec3(0.0, 0.0, 0.0),
            dir: Vec3(0.0, 1.0, 0.0),
        };
        assert_relative_eq!(s.hit(&r, 0.0, std::f32::MAX).t, -1.0); // misses
        let r2 = Ray {
            pos: Vec3(0.0, 0.0, 0.0),
            dir: Vec3(0.0, 0.0, -1.0),
        };
        assert_relative_eq!(s.hit(&r2, 0.0, std::f32::MAX).t, 0.5); // hits the sphere
        assert_relative_eq!(s.hit(&r2, 0.0, std::f32::MAX).normal.length(), 1.0);
        assert_relative_eq!(s.hit(&r2, 0.0, std::f32::MAX).normal.z(), 1.0);
    }
    #[test]
    fn list_can_add_stuff() {
        let mut l = HittableList::new();
        let mut l2 = HittableList::new();
        let s = Sphere {
            center: Vec3(0.0, 0.0, -1.0),
            radius: 0.0,
            material: Box::new(TestMaterial {}),
        };
        let s2 = Sphere {
            center: Vec3(0.0, 0.0, -1.0),
            radius: 0.0,
            material: Box::new(TestMaterial {}),
        };
        let r = Ray {
            pos: Vec3(0.0, 0.0, 0.0),
            dir: Vec3(0.0, 1.0, 0.0),
        };
        l.push(s);
        l2.push(s2);
        l.push(l2);
        assert_relative_eq!(l.hit(&r, 0.0, std::f32::MAX).t, -1.0);
    }

    #[test]
    fn list_is_hittable() {
        let mut l = HittableList::new();
        let r = Ray {
            pos: Vec3(0.0, 0.0, 0.0),
            dir: Vec3(0.0, 0.0, -1.0),
        };
        assert_relative_eq!(l.hit(&r, 0.0, std::f32::MAX).t, -1.0);
        l.push(Sphere {
            center: Vec3(0.0, 0.0, -1.0),
            radius: 0.5,
            material: Box::new(TestMaterial {}),
        });
        assert_relative_eq!(l.hit(&r, 0.0, std::f32::MAX).t, 0.5);
        l.push(Sphere {
            center: Vec3(0.0, 0.0, -2.0),
            radius: 0.5,
            material: Box::new(TestMaterial {}),
        });
        assert_relative_eq!(l.hit(&r, 0.0, std::f32::MAX).t, 0.5);
        l.push(Sphere {
            center: Vec3(0.0, 0.0, -0.9),
            radius: 0.5,
            material: Box::new(TestMaterial {}),
        });
        assert_relative_eq!(l.hit(&r, 0.0, std::f32::MAX).t, 0.4);
    }
}
