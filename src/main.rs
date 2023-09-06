use crate::vec::TriangleNormal;
use bevy::asset::ChangeWatcher;
use bevy::pbr::AlphaMode;
use bevy::pbr::NotShadowReceiver;
use bevy::prelude::*;
use bevy::render::mesh::PrimitiveTopology;
use bevy::utils::Duration;
use bevy_egui::EguiPlugin;
mod camera;
mod edge;
mod mat;
mod ncube;
mod vec;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(AssetPlugin {
                // TODO: REMOVE AFTER IS WORKING
                watch_for_changes: ChangeWatcher::with_delay(Duration::from_secs(2)),
                ..default()
            }),
            camera::CameraPlugin,
            EguiPlugin,
        ))
        .init_resource::<NCube>()
        .add_systems(Startup, (spawn_light, spawn_hypercube))
        .add_systems(Update, update_face_directions)
        .run();
}

fn spawn_light(mut commands: Commands) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(-4.0, 8.0, -4.0),
        ..default()
    });
}

#[derive(Resource)]
pub struct NCube {
    settings: ncube::NCube,
    vertices_3d: Vec<Vec3>,
}
impl std::default::Default for NCube {
    fn default() -> Self {
        let ncube = ncube::NCube::new(4, 1.0);
        Self {
            vertices_3d: ncube.perspective_project_vertices(),
            settings: ncube,
        }
    }
}

#[derive(Component)]
struct Face;

fn spawn_hypercube(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    ncube: Res<NCube>,
) {
    let mesh = shape::Cube::default();
    for (i, j) in &ncube.settings.edges.0 {
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(mesh.into()).into(),
                material: materials.add(StandardMaterial {
                    base_color: Color::CYAN,
                    ..default()
                }),
                transform: edge::Edge::transform(
                    0.01,
                    ncube.vertices_3d[*i],
                    ncube.vertices_3d[*j],
                ),
                ..default()
            },
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
                    base_color: Color::CYAN.with_a(0.2),
                    alpha_mode: AlphaMode::Add,
                    ..default()
                }),
                ..default()
            },
            Face,
            NotShadowReceiver,
        ));
    }
}

fn update_face_directions(
    q_face_meshes_handles: Query<&Handle<Mesh>, (With<Face>, Changed<Handle<Mesh>>)>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    q_face_meshes_handles.iter().for_each(|handle| {
        let mesh = meshes.get_mut(handle).unwrap();
        let normals = mesh
            .attribute(Mesh::ATTRIBUTE_NORMAL)
            .unwrap()
            .as_float3()
            .unwrap();
        let normal = Vec3::from(normals[0]);
        let vertices = mesh
            .attribute(Mesh::ATTRIBUTE_POSITION)
            .unwrap()
            .as_float3()
            .unwrap();
        // Detect whether the normal is the opposite of the origin
        if normal.dot(Vec3::from(vertices[0]) - Vec3::ZERO) <= 0.0 {
            mesh.insert_attribute(
                Mesh::ATTRIBUTE_POSITION,
                vec![vertices[2], vertices[1], vertices[0]],
            );
            mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![-normal; 3]);
        }
    });
}
