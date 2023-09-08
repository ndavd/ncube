use crate::ncube::ExtendedMathOps;
use crate::vec::TriangleNormal;
use bevy::pbr::AlphaMode;
use bevy::pbr::NotShadowReceiver;
use bevy::prelude::*;
use bevy::render::mesh::PrimitiveTopology;
use std::collections::HashMap;
mod camera;
mod edge;
mod mat;
mod ncube;
mod settings;
mod text;
mod vec;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            camera::CameraPlugin,
            settings::SettingsPlugin,
            text::TextPlugin,
        ))
        .init_resource::<NCubeDimension>()
        .init_resource::<NCube>()
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

#[derive(Resource)]
pub struct NCubeDimension(usize);
impl std::default::Default for NCubeDimension {
    fn default() -> Self {
        Self(5)
    }
}

#[derive(Resource)]
pub struct NCube {
    settings: ncube::NCube,
    vertices_3d: Vec<Vec3>,
    /// Dimension indices
    planes_of_rotation: Vec<(usize, usize)>,
    /// k: Plane
    /// v: Angle, angular velocity
    rotations: std::collections::HashMap<(usize, usize), (f32, f32)>,
    paused: bool,
    edge_color: Color,
    face_color: Color,
}
impl std::default::Default for NCube {
    fn default() -> Self {
        let d = NCubeDimension::default().0;
        let ncube = ncube::NCube::new(d, 1.0);
        let planes_of_rotation = usize::pair_permutations(0, d - 1);
        let mut rotations: HashMap<(usize, usize), (f32, f32)> = HashMap::new();
        for plane in &planes_of_rotation {
            rotations.insert(*plane, (0.0_f32, 0.0_f32));
        }
        rotations.insert((1, 2), (0.0, 1.0));
        rotations.insert((0, 3), (0.0, 0.5));
        Self {
            vertices_3d: ncube.perspective_project_vertices(),
            settings: ncube,
            planes_of_rotation,
            rotations,
            paused: false,
            edge_color: Color::CYAN,
            face_color: Color::CYAN.with_a(0.1),
        }
    }
}

fn spawn_hypercube(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    ncube_dimension: Res<NCubeDimension>,
    mut ncube: ResMut<NCube>,
    q_ncube_entities: Query<Entity, With<NCubeMesh>>,
) {
    let creating = ncube.is_added();
    if ncube_dimension.0 == ncube.settings.dimensions && !creating {
        return;
    }

    if !creating {
        q_ncube_entities.iter().for_each(|entity| {
            commands.entity(entity).despawn();
        });

        ncube.settings = ncube::NCube::new(ncube_dimension.0, 1.0);
        let planes_of_rotation = usize::pair_permutations(0, ncube_dimension.0 - 1);
        let mut rotations: HashMap<(usize, usize), (f32, f32)> = HashMap::new();
        for plane in &planes_of_rotation {
            let v = match ncube.rotations.get(plane) {
                Some(v) => *v,
                None => (0.0_f32, 0.0_f32),
            };
            rotations.insert(*plane, v);
            ncube.settings.rotate([plane.0, plane.1], v.1);
        }
        ncube.planes_of_rotation = planes_of_rotation;
        ncube.rotations = rotations;
        ncube.vertices_3d = ncube.settings.perspective_project_vertices();
    }

    let mesh = shape::Cube::default();
    for (i, j) in &ncube.settings.edges.0 {
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(mesh.into()).into(),
                material: materials.add(StandardMaterial {
                    base_color: ncube.edge_color,
                    double_sided: true,
                    cull_mode: None,
                    ..default()
                }),
                transform: edge::Edge::transform(
                    0.01,
                    ncube.vertices_3d[*i],
                    ncube.vertices_3d[*j],
                ),
                ..default()
            },
            Edge,
            NCubeMesh,
            NotShadowReceiver,
        ));
    }
    for (i, j, k) in &ncube.settings.faces.0 {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_POSITION,
            vec![
                ncube.vertices_3d[*i],
                ncube.vertices_3d[*j],
                ncube.vertices_3d[*k],
            ],
        );
        let normal = ncube.vertices_3d[*i].normal(&ncube.vertices_3d[*j], &ncube.vertices_3d[*k]);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![normal; 3]);
        commands.spawn((
            MaterialMeshBundle {
                mesh: meshes.add(mesh.into()).into(),
                material: materials.add(StandardMaterial {
                    base_color: ncube.face_color,
                    alpha_mode: AlphaMode::Add,
                    double_sided: true,
                    cull_mode: None,
                    ..default()
                }),
                ..default()
            },
            Face,
            NCubeMesh,
            NotShadowReceiver,
        ));
    }
}

fn update_ncube_meshes(
    ncube: Res<NCube>,
    mut q_edges: Query<(&mut Transform, &Handle<StandardMaterial>), With<Edge>>,
    q_face_handles: Query<(&Handle<Mesh>, &Handle<StandardMaterial>), With<Face>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if !ncube.is_changed() {
        return;
    };
    q_edges
        .iter_mut()
        .enumerate()
        .for_each(|(i, (mut transform, material_handle))| {
            if let Some(edge) = ncube.settings.edges.0.get(i) {
                let edge_material = materials.get_mut(material_handle).unwrap();
                if edge_material.base_color != ncube.edge_color {
                    edge_material.base_color = ncube.edge_color;
                }
                *transform = edge::Edge::transform(
                    0.01,
                    ncube.vertices_3d[edge.0],
                    ncube.vertices_3d[edge.1],
                );
            }
        });
    q_face_handles
        .iter()
        .enumerate()
        .for_each(|(i, (mesh_handle, material_handle))| {
            if let Some(face) = ncube.settings.faces.0.get(i) {
                let face_material = materials.get_mut(material_handle).unwrap();
                if face_material.base_color != ncube.face_color {
                    face_material.base_color = ncube.face_color;
                }
                meshes.get_mut(mesh_handle).unwrap().insert_attribute(
                    Mesh::ATTRIBUTE_POSITION,
                    vec![
                        ncube.vertices_3d[face.0],
                        ncube.vertices_3d[face.1],
                        ncube.vertices_3d[face.2],
                    ],
                )
            }
        });
}

fn rotate_ncube(time: Res<Time>, mut ncube: ResMut<NCube>) {
    if ncube.paused {
        return;
    }
    let dt = time.delta_seconds();
    for i in 0..ncube.planes_of_rotation.len() {
        let plane = ncube.planes_of_rotation[i];
        let (angle, vel) = *ncube.rotations.get(&plane).unwrap();
        let da = dt * vel;
        ncube.settings.rotate([plane.0, plane.1], da);
        ncube
            .rotations
            .insert((plane.0, plane.1), (angle + da, vel));
    }
    ncube.vertices_3d = ncube.settings.perspective_project_vertices();
}

fn update_pause(
    keyboard_input: Res<Input<bevy::input::keyboard::KeyCode>>,
    mut ncube: ResMut<NCube>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        ncube.paused = !ncube.paused;
    }
}
