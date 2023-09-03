use crate::vec::TriangleNormal;
use bevy::asset::ChangeWatcher;
use bevy::pbr::AlphaMode;
use bevy::pbr::NotShadowReceiver;
use bevy::prelude::*;
use bevy::render::mesh::PrimitiveTopology;
use bevy::utils::Duration;
mod camera;
mod edge;
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
        ))
        .init_resource::<NCube>()
        .add_systems(Startup, (spawn_light, spawn_tesseract))
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
pub struct NCube(ncube::NCube, Vec<Vec3>);
impl std::default::Default for NCube {
    fn default() -> Self {
        let ncube = ncube::NCube::new(3, 0.5);
        let vertices_3d = Vec::with_capacity(ncube.face_count(0));
        Self(ncube, vertices_3d)
    }
}

#[derive(Component)]
struct Face;

fn spawn_tesseract(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    ncube: Res<NCube>,
) {
    let vertices_as_vec3 = ncube
        .0
        .vertices
        .0
        .iter()
        .map(|pos| Vec3::new(pos[0], pos[1], pos[2]))
        .collect::<Vec<_>>();

    let mesh = shape::Cube::default();
    for (i, j) in &ncube.0.edges.0 {
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(mesh.into()).into(),
                material: materials.add(StandardMaterial {
                    base_color: Color::CYAN,
                    ..default()
                }),
                transform: edge::Edge::transform(0.01, vertices_as_vec3[*i], vertices_as_vec3[*j]),
                ..default()
            },
            NotShadowReceiver,
        ));
    }

    for (i, j, k) in &ncube.0.faces.0 {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_POSITION,
            vec![
                vertices_as_vec3[*i],
                vertices_as_vec3[*j],
                vertices_as_vec3[*k],
            ],
        );
        let normal = vertices_as_vec3[*i].normal(&vertices_as_vec3[*j], &vertices_as_vec3[*k]);
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
