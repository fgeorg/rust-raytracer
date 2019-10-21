use std::ops;

#[derive(Debug, PartialEq, Copy)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Vec3 {
    pub fn x(&self) -> f64 {
        self.0
    }
    pub fn y(&self) -> f64 {
        self.1
    }
    pub fn z(&self) -> f64 {
        self.2
    }
    pub fn r(&self) -> f64 {
        self.0
    }
    pub fn g(&self) -> f64 {
        self.1
    }
    pub fn b(&self) -> f64 {
        self.2
    }
    pub fn length(&self) -> f64 {
        (self.0 * self.0 + self.1 * self.1 + self.2 * self.2).sqrt()
    }
    pub fn squared_length(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }
    pub fn normalized(&self) -> (Vec3) {
        *self / self.length()
    }
    pub fn dot(&self, rhs: Vec3) -> f64 {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }
    pub fn cross(&self, rhs: Vec3) -> Vec3 {
        Vec3(
            self.1 * rhs.2 - self.2 * rhs.1,
            -(self.0 * rhs.2 - self.2 * rhs.0),
            self.0 * rhs.1 - self.1 * rhs.0,
        )
    }
}

impl Clone for Vec3 {
    fn clone(&self) -> Self {
        *self
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3(-self.0, -self.1, -self.2)
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) -> () {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl ops::SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) -> () {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Vec3 {
        Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3(self * rhs.0, self * rhs.1, self * rhs.2)
    }
}

impl ops::Div<Vec3> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: Vec3) -> Vec3 {
        Vec3(self.0 / rhs.0, self.1 / rhs.1, self.2 / rhs.2)
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Vec3 {
        Vec3(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl ops::MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, rhs: Vec3) -> () {
        self.0 *= rhs.0;
        self.1 *= rhs.1;
        self.2 *= rhs.2;
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) -> () {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}

impl ops::DivAssign<Vec3> for Vec3 {
    fn div_assign(&mut self, rhs: Vec3) -> () {
        self.0 /= rhs.0;
        self.1 /= rhs.1;
        self.2 /= rhs.2;
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) -> () {
        self.0 /= rhs;
        self.1 /= rhs;
        self.2 /= rhs;
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn accessors_work() {
        let v = Vec3(1.0, 2.0, 3.0);
        assert_eq!(v.x(), v.0);
        assert_eq!(v.y(), v.1);
        assert_eq!(v.z(), v.2);
        assert_eq!(v.r(), v.0);
        assert_eq!(v.g(), v.1);
        assert_eq!(v.b(), v.2);
    }

    #[test]
    fn adds_two_vectors() {
        let v = Vec3(1.0, 2.0, 3.0);
        let v2 = Vec3(5.0, 1.0, 0.0);
        assert_eq!(v + v2, Vec3(1.0 + 5.0, 2.0 + 1.0, 3.0 + 0.0));
    }

    #[test]
    fn negative_operator() {
        let v = Vec3(1.0, 2.0, 3.0);
        assert_eq!(-v, Vec3(-1.0, -2.0, -3.0));
    }

    #[test]
    fn subtracts_two_vectors() {
        let v = Vec3(1.0, 2.0, 3.0);
        let v2 = Vec3(5.0, 1.0, 0.0);
        assert_eq!(v - v2, Vec3(1.0 - 5.0, 2.0 - 1.0, 3.0 - 0.0));
    }

    #[test]
    fn add_assign() {
        let mut v = Vec3(1.0, 2.0, 3.0);
        let v2 = Vec3(5.0, 1.0, 0.0);
        v += v2;
        assert_eq!(v, Vec3(1.0 + 5.0, 2.0 + 1.0, 3.0 + 0.0));
    }
    #[test]
    fn sub_assign() {
        let mut v = Vec3(1.0, 2.0, 3.0);
        let v2 = Vec3(5.0, 1.0, 0.0);
        v -= v2;
        assert_eq!(v, Vec3(1.0 - 5.0, 2.0 - 1.0, 3.0 - 0.0));
    }
    #[test]
    fn mul_two_vectors() {
        let v = Vec3(1.0, 2.0, 3.0);
        let v2 = Vec3(5.0, 1.0, 0.0);
        assert_eq!(v * v2, Vec3(1.0 * 5.0, 2.0 * 1.0, 3.0 * 0.0));
    }
    #[test]
    fn mul_scalar_and_vector() {
        let v = Vec3(1.0, 2.0, 3.0);
        let s = 5.0;
        assert_eq!(s * v, Vec3(s * 1.0, s * 2.0, s * 3.0));
        assert_eq!(v * s, Vec3(s * 1.0, s * 2.0, s * 3.0));
    }
    #[test]
    fn mul_assign_with_scalar() {
        let mut v = Vec3(1.0, 2.0, 3.0);
        let s = 5.0;
        v *= s;
        assert_eq!(v, Vec3(s * 1.0, s * 2.0, s * 3.0));
    }
    #[test]
    fn mul_assign_with_vector() {
        let mut v = Vec3(1.0, 2.0, 3.0);
        let v2 = Vec3(2.0, 3.0, 4.0);
        v *= v2;
        assert_eq!(v, Vec3(2.0 * 1.0, 3.0 * 2.0, 4.0 * 3.0));
    }
    #[test]
    fn div_assign_with_scalar() {
        let mut v = Vec3(1.0, 2.0, 3.0);
        let s = 5.0;
        v /= s;
        assert_eq!(v, Vec3(1.0 / s, 2.0 / s, 3.0 / s));
    }
    #[test]
    fn div_assign_with_vector() {
        let mut v = Vec3(2.0, 3.0, 4.0);
        let v2 = Vec3(1.0, 2.0, 3.0);
        v /= v2;
        assert_eq!(v, Vec3(2.0 / 1.0, 3.0 / 2.0, 4.0 / 3.0));
    }

    #[test]
    fn assign_by_copying() {
        let mut v = Vec3(1.0, 2.0, 3.0);
        let v2 = v;
        v.0 = 7.0;
        assert_eq!(v.0, 7.0);
        assert_eq!(v2.0, 1.0);
    }
    #[test]
    fn assign_by_copying_mut() {
        let mut v = Vec3(1.0, 2.0, 3.0);
        let v2 = v;
        v.0 = 7.0;
        assert_eq!(v.0, 7.0);
        assert_eq!(v2.0, 1.0);
    }
    #[test]
    fn copy_ref() {
        let mut v = Vec3(1.0, 2.0, 3.0);
        assert_eq!(v.0, 1.0);
        let ref mut v2 = v;
        v2.0 = 7.0;
        assert_eq!(v2.0, 7.0);
        assert_eq!(v.0, 7.0);
        v.0 = 5.0;
        assert_eq!(v.0, 5.0);
    }
    #[test]
    fn div_two_vectors() {
        let v = Vec3(1.0, 2.0, 3.0);
        let v2 = Vec3(5.0, 1.0, 0.0);
        assert_eq!(v / v2, Vec3(1.0 / 5.0, 2.0 / 1.0, 3.0 / 0.0));
    }
    #[test]
    fn div_scalar_and_vector() {
        let v = Vec3(1.0, 2.0, 3.0);
        let s = 5.0;
        assert_eq!(v / s, Vec3(1.0 / s, 2.0 / s, 3.0 / s));
    }
    #[test]
    fn length() {
        let v = Vec3(3.0, 4.0, 0.0);
        assert_eq!(v.length(), 5.0);
    }
    #[test]
    fn squared_length() {
        let v = Vec3(3.0, 2.0, 1.0);
        assert_eq!(v.squared_length(), 14.0);
    }
    #[test]
    fn normalized() {
        let v = Vec3(3.0, 2.0, 1.0);
        assert_eq!(
            v.normalized(),
            Vec3(3.0, 2.0, 1.0) / Vec3(3.0, 2.0, 1.0).length()
        );
        assert_eq!(Vec3(6.0, 0.0, 0.0).normalized(), Vec3(1.0, 0.0, 0.0));
    }
    #[test]
    fn dot() {
        let v = Vec3(3.0, 2.0, 1.0);
        let v2 = Vec3(1.0, 2.0, 1.0);
        assert_eq!(v.dot(v2), 3.0 + 4.0 + 1.0);
        let v = Vec3(3.0, -5.0, 4.0);
        let v2 = Vec3(2.0, 6.0, 5.0);
        assert_eq!(v.dot(v2), -4.0);
    }
    #[test]
    fn cross() {
        let v = Vec3(3.0, -5.0, 4.0);
        let v2 = Vec3(2.0, 6.0, 5.0);
        assert_eq!(v.cross(v2), Vec3(-49.0, -7.0, 28.0));
    }

    #[test]
    fn cross_is_right_handed() {
        let x = Vec3(1.0, 0.0, 0.0);
        let y = Vec3(0.0, 1.0, 0.0);
        let z = Vec3(0.0, 0.0, 1.0);
        assert_eq!(x.cross(y), z);
        assert_eq!(y.cross(z), x);
        assert_eq!(z.cross(x), y);
        assert_eq!(x.cross(z), -y);
    }
}
