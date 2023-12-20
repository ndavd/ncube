use crate::ncube::ExtendedMathOps;
use crate::ncube::{self, NCorrection};
use bevy::prelude::*;
use std::collections::HashMap;

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<NCubeDimension>()
            .init_resource::<NCube>()
            .init_resource::<NCubeVertices3D>()
            .init_resource::<NCubePlanesOfRotation>()
            .init_resource::<NCubeRotations>()
            .init_resource::<NCubeCorrection>()
            .init_resource::<NCubeIsPaused>()
            .init_resource::<NCubeEdgeColor>()
            .init_resource::<NCubeFaceColor>()
            .init_resource::<NCubeEdgeThickness>()
            .init_resource::<NCubeUnlit>()
            .init_resource::<IsHoveringFile>()
            .init_resource::<FileDialog>()
            .init_resource::<ShowControls>()
            .init_resource::<FontHandle>()
            .init_resource::<OrthographicCamera>();
    }
}

pub const SIZE: f32 = 1.0;

#[macro_export]
macro_rules! impl_default {
    ($s:ident => $i:expr) => {
        impl std::default::Default for $s {
            fn default() -> Self {
                $i
            }
        }
    };
}
#[macro_export]
macro_rules! create_resource {
    (
        $(#[doc = $doc:expr])*
        $s:ident($t:ty) => $i:expr
    ) => {
        $(#[doc = $doc])*
        #[derive(Resource, Deref, DerefMut)]
        pub struct $s(pub $t);
        impl_default!($s => $i);
    };
}

create_resource!(NCubeDimension(usize) => Self(5));

create_resource!(NCube(ncube::NCube) => {
    let d = NCubeDimension::default();
    Self(ncube::NCube::new(*d, SIZE.into()))
});

create_resource!(NCubeVertices3D(Vec<Vec3>) => {
    let ncube = NCube::default();
    Self(ncube.perspective_project_vertices())
});

create_resource!(
    /// Dimension indices
    NCubePlanesOfRotation(Vec<(usize, usize)>) => {
        let d = NCubeDimension::default();
        Self(usize::pair_permutations(0, *d - 1))
    }
);

create_resource!(
    /// k: Plane
    /// v: Angle, angular velocity
    NCubeRotations(
        std::collections::HashMap<(usize, usize), (f64, f64)>
    ) => {
        let planes_of_rotation = NCubePlanesOfRotation::default();
        let mut rotations: HashMap<(usize, usize), (f64, f64)> = HashMap::new();
        for plane in &*planes_of_rotation {
            rotations.insert(*plane, (0.0, 0.0));
        }
        rotations.insert((1, 2), (0.0, 1.0));
        rotations.insert((0, 3), (0.0, 0.5));
        Self(rotations)
    }
);

pub fn get_slowest_rotation(
    rotations: &std::collections::HashMap<(usize, usize), (f64, f64)>,
) -> (usize, usize, f64, f64) {
    let values = rotations
        .iter()
        .filter(|(_, x)| x.1 != 0.0)
        .min_by(|(_, a), (_, b)| a.1.partial_cmp(&b.1).unwrap())
        .unwrap();
    (values.0 .0, values.0 .1, values.1 .0, values.1 .1)
}

create_resource!(
    /// Used for correcting floating point errors every full rotation
    NCubeCorrection(NCorrection) => {
        let rotations = NCubeRotations::default();
        let vertices_3d = NCubeVertices3D::default();
        Self (
            NCorrection {
                slowest_rotation: get_slowest_rotation(&rotations),
                rotations: rotations.0,
                vertices_3d: vertices_3d.0,
                vertices: ncube::NVertices(Vec::new()),
                current_angle: 0.0,
            }
        )
    }
);
impl NCubeCorrection {
    pub fn new(
        rotations: std::collections::HashMap<(usize, usize), (f64, f64)>,
        vertices_3d: Vec<Vec3>,
        vertices: ncube::NVertices,
    ) -> Self {
        println!("NEW CORRECTION SAVED");
        Self(NCorrection {
            slowest_rotation: get_slowest_rotation(&rotations),
            rotations,
            vertices_3d,
            vertices,
            current_angle: 0.0,
        })
    }
}

create_resource!(NCubeIsPaused(bool) => Self(false));

create_resource!(NCubeEdgeColor(Color) => Self(Color::CYAN));

create_resource!(NCubeFaceColor(Color) => Self(Color::CYAN.with_a(0.1)));

create_resource!(NCubeEdgeThickness(f32) => Self(0.01 * SIZE));

create_resource!(NCubeUnlit(bool) => Self(false));

create_resource!(IsHoveringFile(bool) => Self(false));

#[cfg(not(target_family = "wasm"))]
create_resource!(FileDialog(Option<egui_file::FileDialog>) => Self(None));
#[cfg(target_family = "wasm")]
create_resource!(FileDialog(()) => Self(()));

create_resource!(ShowControls(bool) => Self(false));

create_resource!(FontHandle(Handle<Font>) => Self(Handle::default()));

create_resource!(OrthographicCamera(bool) => Self(false));
