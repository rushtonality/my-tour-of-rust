#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    e : [f32; 3],
}

impl Vec3 {
    pub fn new(e0: f32, e1: f32, e2: f32) -> Vec3 {
        Vec3 { e: [e0, e1, e2] }
    }
    pub fn x(&self) -> f32 {
        self.e[0]
    }
    pub fn y(&self) -> f32 {
        self.e[1]
    }
    pub fn z(&self) -> f32 {
        self.e[2]
    }

    pub fn r(&self) -> f32 {
        self.e[0]
    }
    pub fn g(&self) -> f32 {
        self.e[1]
    }
    pub fn b(&self) -> f32 {
        self.e[2]
    }

    pub fn length(&self) -> f32 {
        f32::sqrt(self.e[0] * self.e[0] +
            self.e[1] * self.e[1] +
            self.e[2] * self.e[2])
    }

    pub fn squared_length(&self) -> f32 {
        self.e[0] * self.e[0] +
            self.e[1] * self.e[1] +
            self.e[2] * self.e[2]
    }

    pub fn make_unit_vector(v : Vec3) -> Vec3 {
        v / v.length()
    }

    pub fn dot(v1 : &Vec3, &v2 : &Vec3) -> f32 {
        v1.e[0] * v2.e[0] + v1.e[1] * v2.e[1] + v1.e[2] * v2.e[2]
    }

    pub fn cross(v1 : &Vec3, v2 : &Vec3) -> Vec3 {
        Vec3::new(v1.e[1] * v2.e[2] - v1.e[2]*v2.e[1],
            -(v1.e[0]*v2.e[2] - v1.e[2]*v2.e[0]),
            v1.e[0]*v2.e[1] - v1.e[1] * v2.e[0])
    }
}

impl std::cmp::PartialEq for Vec3 {

    fn eq(&self, other: &Vec3) -> bool {
        f32::abs(self.e[0] - other.e[0]) < core::f32::EPSILON &&
            f32::abs(self.e[1] - other.e[1]) < core::f32::EPSILON &&
            f32::abs(self.e[2] - other.e[2]) < core::f32::EPSILON
    }

    fn ne(&self, other: &Vec3) -> bool {
        !self.eq(other)
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.e[0], self.e[1], self.e[2])
    }
}

impl std::ops::Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, index: usize) -> &f32 {
        &self.e[index]
    }
}

impl std::ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut f32 {
        &mut self.e[index]
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3::new(
            -self.e[0],
            -self.e[1],
            -self.e[2])
    }
}

impl std::ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3::new(
            self.e[0] + rhs.e[0],
            self.e[1] + rhs.e[1],
            self.e[2] + rhs.e[2])
    }
}

impl std::ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3::new(
            self.e[0] - rhs.e[0],
            self.e[1] - rhs.e[1],
            self.e[2] - rhs.e[2])
    }
}

impl std::ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3::new(
            self.e[0] * rhs.e[0],
            self.e[1] * rhs.e[1],
            self.e[2] * rhs.e[2])
    }
}

impl std::ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Vec3 {
        Vec3::new(
            self.e[0] * rhs,
            self.e[1] * rhs,
            self.e[2] * rhs)
    }
}

impl std::ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3::new(
            self * rhs.e[0],
            self * rhs.e[1],
            self * rhs.e[2])
    }
}

impl std::ops::Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Vec3 {
        Vec3::new(
            self.e[0] / rhs.e[0],
            self.e[1] / rhs.e[1],
            self.e[2] / rhs.e[2])
    }
}

impl std::ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Vec3 {
        Vec3::new(
            self.e[0] / rhs,
            self.e[1] / rhs,
            self.e[2] / rhs)
    }
}


impl std::ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.e[0] += other.e[0];
        self.e[1] += other.e[1];
        self.e[2] += other.e[2];
    }
}

impl std::ops::AddAssign<f32> for Vec3 {
    fn add_assign(&mut self, other: f32) {
        self.e[0] += other;
        self.e[1] += other;
        self.e[2] += other;
    }
}

impl std::ops::SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        self.e[0] -= other.e[0];
        self.e[1] -= other.e[1];
        self.e[2] -= other.e[2];
    }
}

impl std::ops::SubAssign<f32> for Vec3 {
    fn sub_assign(&mut self, other: f32) {
        self.e[0] -= other;
        self.e[1] -= other;
        self.e[2] -= other;
    }
}

impl std::ops::MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, other: Vec3) {
        self.e[0] *= other.e[0];
        self.e[1] *= other.e[1];
        self.e[2] *= other.e[2];
    }
}

impl std::ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, other: f32) {
        self.e[0] *= other;
        self.e[1] *= other;
        self.e[2] *= other;
    }
}

impl std::ops::DivAssign<Vec3> for Vec3 {
    fn div_assign(&mut self, other: Vec3) {
        self.e[0] /= other.e[0];
        self.e[1] /= other.e[1];
        self.e[2] /= other.e[2];
    }
}

impl std::ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, other: f32) {
        self.e[0] /= other;
        self.e[1] /= other;
        self.e[2] /= other;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eq() {
        let a = Vec3::new(1.0,2.0, 3.0000001);
        let b = Vec3::new(1.0,2.0, 3.0);
        assert_eq!(a, b);
    }

    #[test]
    fn test_ne() {
        let a = Vec3::new(1.0,2.0, 3.1);
        let b = Vec3::new(1.0,2.0, 3.0);
        assert_ne!(a, b);
    }

    #[test]
    fn test_index() {
        let a = Vec3::new(1.0,2.0, 3.1);
        assert_eq!(a[0], 1.0);
        assert_eq!(a[1], 2.0);
        assert_eq!(a[2], 3.1);
    }

    #[test]
    fn test_index_assign() {
        let mut a = Vec3::new(1.0,2.0, 3.1);
        a[0] = 2.0;
        assert_eq!(a[0], 2.0);
        a[1] = 3.0;
        assert_eq!(a[1], 3.0);
        a[2] = 4.0;
        assert_eq!(a[2], 4.0);
    }

    #[test]
    fn test_neg() {
        let a = Vec3::new(1.0,2.0, 3.1);
        let answer = Vec3::new(-1.0,-2.0, -3.1);
        assert_eq!(-a, answer);
    }

    #[test]
    fn test_add() {
        let a = Vec3::new(1.0,2.0, 3.1);
        let b = Vec3::new(1.0,2.0, 3.1);
        let answer = Vec3::new(2.0,4.0, 6.2);
        assert_eq!(a+b, answer);
    }

    #[test]
    fn test_sub() {
        let a = Vec3::new(1.0,2.0, 3.1);
        let b = Vec3::new(1.0,2.0, 3.1);
        let answer = Vec3::new(0.0,0.0, 0.0);
        assert_eq!(a-b, answer);
    }

    #[test]
    fn test_mul() {
        let a = Vec3::new(1.0,2.0, 3.1);
        let b = Vec3::new(1.0,2.0, 3.1);
        let answer = Vec3::new(1.0,4.0, 9.61);
        assert_eq!(a*b, answer);
    }

    #[test]
    fn test_div() {
        let a = Vec3::new(4.0,9.0, 12.0);
        let b = Vec3::new(2.0,3.0, 4.0);
        let answer = Vec3::new(2.0,3.0, 3.0);
        assert_eq!(a/b, answer);
    }

    #[test]
    fn test_add_assign() {
        let mut a = Vec3::new(1.0,2.0, 3.1);
        let b = Vec3::new(1.0,2.0, 3.1);
        let answer = Vec3::new(2.0,4.0, 6.2);
        a += b;
        assert_eq!(a, answer);
    }

    #[test]
    fn test_sub_assign() {
        let mut a = Vec3::new(1.0,2.0, 3.1);
        let b = Vec3::new(1.0,2.0, 3.1);
        let answer = Vec3::new(0.0,0.0, 0.0);
        a -= b;
        assert_eq!(a, answer);
    }

    #[test]
    fn test_make_unit_vector() {
        let a = Vec3::new(1.0,2.0, 3.0);
        let u = Vec3::make_unit_vector(a);
        assert!(f32::abs(1.0 - u.length()) < core::f32::EPSILON);
    }
}


fn main() {
    let nx = 200;
    let ny = 100;

    print!("P3\n{} {}\n255\n", nx, ny);
    for j in (0..ny).rev() {
        for i in 0..nx {
            let col = Vec3::new(
                i as f32 / nx as f32,
                j as f32 / ny as f32,
                0.2);

            let ir = (255.00*col[0]) as i32;
            let ig = (255.00*col[1]) as i32;
            let ib = (255.00*col[2]) as i32;

            print!("{} {} {}\n", ir, ig, ib);
        }
    }
}
