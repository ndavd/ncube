use bevy::prelude::Vec3;

#[derive(Debug)]
pub struct SphericalCoordinate {
    pub r: f32,
    pub theta: f32,
    pub phi: f32,
}

impl SphericalCoordinate {
    /// Generates a new `SphericalCoordinate`.
    /// Keep in mind `theta` and `phi` are in radians.
    pub fn new(r: f32, theta: f32, phi: f32) -> Self {
        Self { r, theta, phi }
    }
}

// NOTE: In bevy, ^y is "UP" by default
pub trait SphericalCoordinateSystem {
    fn from_spherical(spherical_coordinate: SphericalCoordinate) -> Self;
    fn to_spherical(&self) -> SphericalCoordinate;
}

impl SphericalCoordinateSystem for Vec3 {
    /// Generates a `Vec3` from the given `SphericalCoordinate`.
    fn from_spherical(sc: SphericalCoordinate) -> Self {
        Self {
            x: sc.r * sc.theta.sin() * sc.phi.sin(),
            y: sc.r * sc.theta.cos(),
            z: sc.r * sc.theta.sin() * sc.phi.cos(),
        }
    }
    /// Converts a cartesian coordinate to spherical.
    fn to_spherical(&self) -> SphericalCoordinate {
        let r = self.length();
        let theta = (self.y / r).acos();
        let phi = (self.x).atan2(self.z);
        SphericalCoordinate { r, theta, phi }
    }
}

pub trait TriangleNormal {
    fn normal(&self, b: &Self, c: &Self) -> Self;
}

impl TriangleNormal for Vec3 {
    /// Computes the normal for a triangle based on its vertices
    fn normal(&self, b: &Self, c: &Self) -> Self {
        let a = self;
        (*b - *a).cross(*c - *b).normalize()
    }
}

pub trait MathOps {
    // Computes the dot product between 2 vectors
    fn dot(&self, b: &Self) -> f64;
    // Computes the hadamard product between 2 vectors
    fn hadamard(&self, b: &Self) -> Self;
    /// Checks how many dimensions are shared across a group of `points` (equal in value)
    /// Useful for checking whether the points lie within an n dimensional slice.
    fn shared_dimensions(points: &[&Self]) -> Vec<usize>;
    /// Computes the distance between 2 vectors
    fn distance(&self, b: &Self) -> f64;
}

impl MathOps for Vec<f64> {
    fn dot(&self, b: &Self) -> f64 {
        let len = self.len();
        assert_eq!(len, b.len());
        (0..len).map(|d| self[d] * b[d]).sum()
    }

    fn hadamard(&self, b: &Self) -> Self {
        let len = self.len();
        assert_eq!(len, b.len());
        (0..len).map(|d| self[d] * b[d]).collect()
    }

    fn shared_dimensions(points: &[&Self]) -> Vec<usize> {
        let mut common_d = Vec::new();
        for d in 0..points[0].len() {
            if points.iter().all(|p| p[d] == points[0][d]) {
                common_d.push(d);
            }
        }
        common_d
    }

    fn distance(&self, b: &Self) -> f64 {
        let d = self.len();
        assert_eq!(d, b.len());
        self.iter()
            .enumerate()
            .map(|(i, x)| (b[i] - x).powi(2))
            .sum::<f64>()
            .sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn spherical_coordinates() {
        let a = Vec3::new(3.0, -2.5, 9.0);
        let a_spherical = a.to_spherical();
        let a_prime = Vec3::from_spherical(a_spherical);
        let precision = 0.00001;
        println!("Comparing {a:?} with {a_prime:?} +- {precision}");
        assert!(a.cmpgt(a_prime - precision).all() && a.cmplt(a_prime + precision).all());
    }
}
