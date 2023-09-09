use crate::ncube::ExtendedMathOps;
use crate::vec::TriangleNormal;
use bevy::pbr::AlphaMode;
use bevy::prelude::*;
use bevy::render::mesh::PrimitiveTopology;
use bevy_embedded_assets::EmbeddedAssetPlugin;
use std::collections::HashMap;
mod camera;
mod edge;
mod mat;
mod ncube;
mod settings;
mod text;
mod vec;

pub const S: f32 = 1.0;

fn main() {
    App::new()
        .init_resource::<NCubeDimension>()
        .init_resource::<NCube>()
        .init_resource::<NCubeVertices3D>()
        .init_resource::<NCubePlanesOfRotation>()
        .init_resource::<NCubeRotations>()
        .init_resource::<NCubeIsPaused>()
        .init_resource::<NCubeEdgeColor>()
        .init_resource::<NCubeFaceColor>()
        .init_resource::<NCubeEdgeThickness>()
        .add_plugins((
            DefaultPlugins
                .build()
                .add_before::<bevy::asset::AssetPlugin, _>(EmbeddedAssetPlugin),
            camera::CameraPlugin,
            settings::SettingsPlugin,
            text::TextPlugin,
        ))
        .add_systems(
            Update,
            (
                spawn_hypercube,
                rotate_ncube,
                update_ncube_meshes,
                update_pause,
            ),
        )
        .run();
}

#[derive(Component)]
struct Edge;
#[derive(Component)]
struct Face;
#[derive(Component)]
struct NCubeMesh;

#[derive(Resource, Deref, DerefMut)]
pub struct NCubeDimension(usize);
impl std::default::Default for NCubeDimension {
    fn default() -> Self {
        Self(5)
    }
}

#[derive(Resource, Deref, DerefMut)]
pub struct NCube(ncube::NCube);
impl std::default::Default for NCube {
    fn default() -> Self {
        let d = NCubeDimension::default();
        Self(ncube::NCube::new(*d, S))
    }
}

#[derive(Resource, Deref, DerefMut)]
pub struct NCubeVertices3D(Vec<Vec3>);
impl std::default::Default for NCubeVertices3D {
    fn default() -> Self {
        let ncube = NCube::default();
        Self(ncube.perspective_project_vertices())
    }
}

/// Dimension indices
#[derive(Resource, Deref, DerefMut)]
pub struct NCubePlanesOfRotation(Vec<(usize, usize)>);
impl std::default::Default for NCubePlanesOfRotation {
    fn default() -> Self {
        let d = NCubeDimension::default();
        Self(usize::pair_permutations(0, *d - 1))
    }
}

/// k: Plane
/// v: Angle, angular velocity
#[derive(Resource, Deref, DerefMut)]
pub struct NCubeRotations(std::collections::HashMap<(usize, usize), (f32, f32)>);
impl std::default::Default for NCubeRotations {
    fn default() -> Self {
        let planes_of_rotation = NCubePlanesOfRotation::default();
        let mut rotations: HashMap<(usize, usize), (f32, f32)> = HashMap::new();
        for plane in &*planes_of_rotation {
            rotations.insert(*plane, (0.0_f32, 0.0_f32));
        }
        rotations.insert((1, 2), (0.0, 1.0));
        rotations.insert((0, 3), (0.0, 0.5));
        Self(rotations)
    }
}

#[derive(Resource, Deref, DerefMut)]
pub struct NCubeIsPaused(bool);
impl std::default::Default for NCubeIsPaused {
    fn default() -> Self {
        Self(false)
    }
}

#[derive(Resource, Deref, DerefMut)]
pub struct NCubeEdgeColor(Color);
impl std::default::Default for NCubeEdgeColor {
    fn default() -> Self {
        Self(Color::CYAN)
    }
}

#[derive(Resource, Deref, DerefMut)]
pub struct NCubeFaceColor(Color);
impl std::default::Default for NCubeFaceColor {
    fn default() -> Self {
        Self(Color::CYAN.with_a(0.1))
    }
}

#[derive(Resource, Deref, DerefMut)]
pub struct NCubeEdgeThickness(f32);
impl std::default::Default for NCubeEdgeThickness {
    fn default() -> Self {
        Self(0.01)
    }
}

fn spawn_hypercube(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    ncube_dimension: Res<NCubeDimension>,
    mut ncube: ResMut<NCube>,
    mut ncube_rotations: ResMut<NCubeRotations>,
    mut ncube_planes_of_rotation: ResMut<NCubePlanesOfRotation>,
    mut ncube_vertices_3d: ResMut<NCubeVertices3D>,
    ncube_edge_color: Res<NCubeEdgeColor>,
    ncube_face_color: Res<NCubeFaceColor>,
    q_ncube_entities: Query<Entity, With<NCubeMesh>>,
) {
    let is_changed = ncube_dimension.is_changed();
    if !ncube_dimension.is_added() && !is_changed {
        return;
    }

    if is_changed {
        q_ncube_entities.iter().for_each(|entity| {
            commands.entity(entity).despawn();
        });

        **ncube = ncube::NCube::new(**ncube_dimension, 1.0);
        let planes_of_rotation = usize::pair_permutations(0, **ncube_dimension - 1);
        let mut rotations: HashMap<(usize, usize), (f32, f32)> = HashMap::new();
        let mut angles = Vec::new();
        for plane in &planes_of_rotation {
            let v = match ncube_rotations.get(plane) {
                Some(v) => *v,
                None => (0.0_f32, 0.0_f32),
            };
            rotations.insert(*plane, v);
            angles.push(v.0);
        }
        **ncube_rotations = rotations;
        **ncube_vertices_3d = ncube
            .rotate(&planes_of_rotation, &angles)
            .perspective_project_vertices();
        **ncube_planes_of_rotation = planes_of_rotation;
    }

    let mesh = shape::Cube::default();
    for (i, j) in &ncube.edges.0 {
        commands.spawn((
            MaterialMeshBundle {
                mesh: meshes.add(mesh.into()).into(),
                material: materials.add(StandardMaterial {
                    base_color: **ncube_edge_color,
                    double_sided: true,
                    cull_mode: None,
                    ..default()
                }),
                transform: edge::Edge::transform(
                    0.01,
                    ncube_vertices_3d[*i],
                    ncube_vertices_3d[*j],
                ),
                ..default()
            },
            Edge,
            NCubeMesh,
        ));
    }
    for (i, j, k) in &ncube.faces.0 {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_POSITION,
            vec![
                ncube_vertices_3d[*i],
                ncube_vertices_3d[*j],
                ncube_vertices_3d[*k],
            ],
        );
        let normal = ncube_vertices_3d[*i].normal(&ncube_vertices_3d[*j], &ncube_vertices_3d[*k]);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![normal; 3]);
        commands.spawn((
            MaterialMeshBundle {
                mesh: meshes.add(mesh.into()).into(),
                material: materials.add(StandardMaterial {
                    base_color: **ncube_face_color,
                    alpha_mode: AlphaMode::Add,
                    double_sided: true,
                    cull_mode: None,
                    ..default()
                }),
                ..default()
            },
            Face,
            NCubeMesh,
        ));
    }
}

fn update_ncube_meshes(
    ncube: Res<NCube>,
    ncube_edge_color: Res<NCubeEdgeColor>,
    ncube_face_color: Res<NCubeFaceColor>,
    ncube_edge_thickness: Res<NCubeEdgeThickness>,
    ncube_vertices_3d: Res<NCubeVertices3D>,
    mut q_edges: Query<(&mut Transform, &Handle<StandardMaterial>), With<Edge>>,
    q_face_handles: Query<(&Handle<Mesh>, &Handle<StandardMaterial>), With<Face>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if ncube_edge_color.is_changed() {
        q_edges.iter().for_each(|(_, material_handle)| {
            materials.get_mut(material_handle).unwrap().base_color = **ncube_edge_color;
        });
    }
    if ncube_face_color.is_changed() {
        q_face_handles.iter().for_each(|(_, material_handle)| {
            materials.get_mut(material_handle).unwrap().base_color = **ncube_face_color;
        });
    }

    if !ncube.is_changed() {
        return;
    };

    q_edges
        .iter_mut()
        .enumerate()
        .for_each(|(i, (mut transform, _))| {
            if let Some(edge) = ncube.edges.0.get(i) {
                *transform = edge::Edge::transform(
                    **ncube_edge_thickness,
                    ncube_vertices_3d[edge.0],
                    ncube_vertices_3d[edge.1],
                );
            }
        });
    q_face_handles
        .iter()
        .enumerate()
        .for_each(|(i, (mesh_handle, _))| {
            if let Some(face) = ncube.faces.0.get(i) {
                meshes.get_mut(mesh_handle).unwrap().insert_attribute(
                    Mesh::ATTRIBUTE_POSITION,
                    vec![
                        ncube_vertices_3d[face.0],
                        ncube_vertices_3d[face.1],
                        ncube_vertices_3d[face.2],
                    ],
                )
            }
        });
}

fn rotate_ncube(
    time: Res<Time>,
    mut ncube: ResMut<NCube>,
    mut ncube_rotations: ResMut<NCubeRotations>,
    ncube_planes_of_rotation: Res<NCubePlanesOfRotation>,
    mut ncube_vertices_3d: ResMut<NCubeVertices3D>,
    ncube_is_paused: Res<NCubeIsPaused>,
) {
    if **ncube_is_paused {
        return;
    }
    let dt = time.delta_seconds();
    let mut das = Vec::new();
    for i in 0..ncube_planes_of_rotation.len() {
        let plane = ncube_planes_of_rotation[i];
        let (angle, vel) = *ncube_rotations.get(&plane).unwrap();
        let da = dt * vel;
        das.push(da);
        ncube_rotations.insert((plane.0, plane.1), (angle + da, vel));
    }
    ncube.rotate(&ncube_planes_of_rotation, &das);
    **ncube_vertices_3d = ncube.perspective_project_vertices();
}

fn update_pause(
    keyboard_input: Res<Input<bevy::input::keyboard::KeyCode>>,
    mut ncube_is_paused: ResMut<NCubeIsPaused>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        **ncube_is_paused = !**ncube_is_paused;
    }
}
