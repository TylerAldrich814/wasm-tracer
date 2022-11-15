use std::ops;

pub type Color = Vec3<f32>;
pub type Point3 = Vec3<f32>;

impl Color {
    #[inline]
    pub fn white() -> Color {
        Color::new(1.0, 1.0, 1.0)
    }
    #[inline]
    pub fn black() -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
    #[inline]
    pub fn red() -> Color {
        Color::new(1.0, 0.0, 0.0)
    }
    #[inline]
    pub fn green() -> Color {
        Color::new(0.0, 1.0, 0.0)
    }
    #[inline]
    pub fn blue() -> Color {
        Color::new(0.0, 0.0, 1.0)
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Vec3<V: Sized + Clone> {
    d: [V; 3],
}

impl Vec3<f32> {
    pub fn new(v0: f32, v1: f32, v2: f32) -> Vec3<f32> {
        Vec3 { d: [v0, v1, v2] }
    }

    #[inline]
    pub fn x(&self) -> f32 {
        self.d[0]
    }
    #[inline]
    pub fn y(&self) -> f32 {
        self.d[1]
    }
    #[inline]
    pub fn z(&self) -> f32 {
        self.d[2]
    }

    pub fn length(&self) -> f32 {
        return self.length_squared().sqrt();
    }
    pub fn length_squared(&self) -> f32 {
        let v = &self.d;
        return v[0] * v[0] + v[1] * v[1] + v[2] * v[2];
    }

    pub fn as_u32_be(&self) -> u32 {
        ((self.d[0] as u32) << 24) + // R
        ((self.d[1] as u32) << 16) + // G
        ((self.d[2] as u32) << 8) +  // B
        (0xFF << 0) // A
    }
    pub fn as_u32_le(&self) -> u32 {
        ((self.d[0] as u32) << 0)
            + ((self.d[1] as u32) << 8)
            + ((self.d[2] as u32) << 16)
            + ((0xFF as u32) << 24)
    }
}

#[inline]
pub fn dot(lhs: &Vec3<f32>, rhs: &Vec3<f32>) -> f32 {
    let v = &lhs.d;
    let k = &rhs.d;
    return v[0] * k[0] + v[1] * k[1] + v[2] * k[2];
}
#[inline]
pub fn cross(lhs: &Vec3<f32>, rhs: &Vec3<f32>) -> Vec3<f32> {
    let v = &lhs.d;
    let k = &rhs.d;
    return Vec3 {
        d: [
            v[1] * k[2] - v[2] * k[1],
            v[2] * k[0] - v[0] * k[2],
            v[0] * k[1] - v[1] * k[0],
        ],
    };
}
#[inline]
pub fn unit_vector(v: Vec3<f32>) -> Vec3<f32> {
    let len = v.length();
    return v / len;
}

impl ops::AddAssign<Vec3<f32>> for Vec3<f32> {
    fn add_assign(&mut self, rhs: Vec3<f32>) {
        self.d[0] += rhs.d[0];
        self.d[1] += rhs.d[1];
        self.d[2] += rhs.d[2];
    }
}

impl ops::MulAssign<f32> for Vec3<f32> {
    fn mul_assign(&mut self, rhs: f32) {
        self.d[0] *= rhs;
        self.d[1] *= rhs;
        self.d[2] *= rhs;
    }
}
impl ops::DivAssign<Vec3<f32>> for Vec3<f32> {
    fn div_assign(&mut self, rhs: Vec3<f32>) {
        self.d[0] /= rhs.d[0];
        self.d[1] /= rhs.d[1];
        self.d[2] /= rhs.d[2];
    }
}

impl ops::Neg for &Vec3<f32> {
    type Output = Vec3<f32>;

    fn neg(self) -> Vec3<f32> {
        let k = self.d;
        Vec3 {
            d: [k[0] * -1.0, k[1] * -1.0, k[2] * -1.0],
        }
    }
}
impl ops::Neg for Vec3<f32> {
    type Output = Vec3<f32>;

    fn neg(self) -> Vec3<f32> {
        let r = self.d;
        Vec3 {
            d: [r[0] * -1., r[1] * -1., r[2] * -1.],
        }
    }
}
impl ops::Index<usize> for Vec3<f32> {
    type Output = f32;

    fn index(&self, index: usize) -> &f32 {
        &self.d[index]
    }
}
impl ops::Add<Vec3<f32>> for Vec3<f32> {
    type Output = Vec3<f32>;

    fn add(self, rhs: Vec3<f32>) -> Vec3<f32> {
        let v = &self.d;
        let k = &rhs.d;
        Vec3 {
            d: [v[0] + k[0], v[1] + k[1], v[2] + k[2]],
        }
    }
}

impl ops::Add<Vec3<f32>> for &Vec3<f32> {
    type Output = Vec3<f32>;

    fn add(self, rhs: Vec3<f32>) -> Vec3<f32> {
        let v = &self.d;
        let k = &rhs.d;
        Vec3 {
            d: [v[0] + k[0], v[1] + k[1], v[2] + k[2]],
        }
    }
}

impl<'a, 'b> ops::Add<&'b Vec3<f32>> for &'a Vec3<f32> {
    type Output = Vec3<f32>;

    fn add(self, rhs: &'b Vec3<f32>) -> Vec3<f32> {
        let v = &self.d;
        let k = &rhs.d;
        Vec3 {
            d: [v[0] + k[0], v[1] + k[1], v[2] + k[2]],
        }
    }
}
impl ops::Sub<Vec3<f32>> for Vec3<f32> {
    type Output = Vec3<f32>;

    fn sub(self, rhs: Vec3<f32>) -> Vec3<f32> {
        let v = &self.d;
        let k = &rhs.d;
        Vec3 {
            d: [v[0] - k[0], v[1] - k[1], v[2] - k[2]],
        }
    }
}

impl<'a, 'b> ops::Sub<&'b Vec3<f32>> for &'a Vec3<f32> {
    type Output = Vec3<f32>;

    fn sub(self, rhs: &'b Vec3<f32>) -> Vec3<f32> {
        let v = &self.d;
        let k = &rhs.d;
        Vec3 {
            d: [v[0] - k[0], v[1] - k[1], v[2] - k[2]],
        }
    }
}

impl<'a, 'b> ops::Mul<&'b Vec3<f32>> for &'a Vec3<f32> {
    type Output = Vec3<f32>;

    fn mul(self, rhs: &'b Vec3<f32>) -> Vec3<f32> {
        let v = &self.d;
        let k = rhs.d;
        Vec3 {
            d: [v[0] * k[0], v[1] * k[1], v[2] * k[2]],
        }
    }
}

impl ops::Mul<Vec3<f32>> for f32 {
    type Output = Vec3<f32>;

    fn mul(self, rhs: Vec3<f32>) -> Vec3<f32> {
        let v = rhs.d;
        Vec3 {
            d: [v[0] * self, v[1] * self, v[2] * self],
        }
    }
}
impl ops::Mul<f32> for &Vec3<f32> {
    type Output = Vec3<f32>;

    fn mul(self, rhs: f32) -> Vec3<f32> {
        let v = self.d;
        Vec3 {
            d: [v[0] * rhs, v[1] * rhs, v[2] * rhs],
        }
    }
}

impl ops::Div<f32> for Vec3<f32> {
    type Output = Vec3<f32>;

    fn div(self, rhs: f32) -> Vec3<f32> {
        let v = self.d;
        Vec3 {
            d: [v[0] / rhs, v[1] / rhs, v[2] / rhs],
        }
    }
}
