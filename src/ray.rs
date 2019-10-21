use crate::vec3::*;

pub struct Ray {
    pub pos: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn point_at_t(&self, t: f64) -> Vec3 {
        self.pos + self.dir * t
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ray_struct() {
        let r = Ray {
            pos: Vec3(0.0, 0.0, 0.0),
            dir: Vec3(1.0, 0.0, 0.0),
        };
        assert_eq!(r.pos.x(), 0.0);
        assert_eq!(r.dir.x(), 1.0);
        assert_eq!(r.dir.0, 1.0);
        assert_eq!(r.dir.r(), 1.0);
    }
    #[test]
    fn point_at_t() {
        let r = Ray {
            pos: Vec3(0.0, 0.0, 0.0),
            dir: Vec3(1.0, 0.0, 0.0),
        };
        assert_eq!(r.point_at_t(4.0), Vec3(4.0, 0.0, 0.0));
    }
}
