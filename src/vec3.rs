#[derive(Default, Clone, Copy)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn new<T: Into<f64>>(x: T, y: T, z: T) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
            z: z.into(),
        }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    fn len_sqr(&self) -> f64 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }

    pub fn len(&self) -> f64 {
        self.len_sqr().sqrt()
    }

    pub fn unit(self) -> Vec3 {
        self / self.len()
    }
}

impl std::ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x() + rhs.x(),
            y: self.y() + rhs.y(),
            z: self.z() + rhs.z(),
        }
    }
}

impl std::ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x() - rhs.x(),
            y: self.y() - rhs.y(),
            z: self.z() - rhs.z(),
        }
    }
}

impl<T: Into<f64>> std::ops::Mul<T> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: T) -> Self::Output {
        let rhs: f64 = rhs.into();

        Self::Output {
            x: self.x() * rhs,
            y: self.y() * rhs,
            z: self.z() * rhs,
        }
    }
}

impl<T: Into<f64>> std::ops::Div<T> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: T) -> Self::Output {
        let rhs: f64 = rhs.into();

        Self::Output {
            x: self.x() / rhs,
            y: self.y() / rhs,
            z: self.z() / rhs,
        }
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}
