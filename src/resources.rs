use crate::ncube;
use crate::ncube::ExtendedMathOps;
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
    Self(ncube::NCube::new(*d, SIZE))
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
        std::collections::HashMap<(usize, usize), (f32, f32)>
    ) => {
        let planes_of_rotation = NCubePlanesOfRotation::default();
        let mut rotations: HashMap<(usize, usize), (f32, f32)> = HashMap::new();
        for plane in &*planes_of_rotation {
            rotations.insert(*plane, (0.0_f32, 0.0_f32));
        }
        rotations.insert((1, 2), (0.0, 1.0));
        rotations.insert((0, 3), (0.0, 0.5));
        Self(rotations)
    }
);

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
