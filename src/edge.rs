use bevy::prelude::*;

#[derive(Debug)]
pub struct Edge();

impl Edge {
    /// Gets the transform for a unit cube so that it behaves like a 3
    /// dimensional segment from point `from` to point `to`.
    pub fn transform(thickness: f32, from: Vec3, to: Vec3) -> Transform {
        let diff = to - from;
        Transform {
            translation: (from + to) / 2.0,
            scale: Vec3::new(thickness, thickness, diff.length() + thickness),
            rotation: Quat::from_rotation_arc(Vec3::Z, diff.normalize()),
        }
    }
}
