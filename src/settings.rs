use crate::ncube::NCube as InnerNCube;
use crate::NCube;
use crate::NCubeDimension;
use crate::NCubeEdgeColor;
use crate::NCubeEdgeThickness;
use crate::NCubeFaceColor;
use crate::NCubeIsPaused;
use crate::NCubePlanesOfRotation;
use crate::NCubeRotations;
use crate::NCubeVertices3D;
use crate::S;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<IsHoveringFile>()
            .add_plugins(EguiPlugin)
            .add_systems(Update, info_panel);
    }
}
#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct CameraTransform {
    translation: Vec3,
    rotation: Quat,
    scale: Vec3,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct NCubeData {
    dimension: usize,
    rotations: Vec<(usize, usize, f32, f32)>,
    edge_thickness: f32,
    edge_color: Color,
    face_color: Color,
    camera_transform: CameraTransform,
}

#[derive(Resource, Deref, DerefMut, Default)]
struct IsHoveringFile(bool);

fn info_panel(
    mut ncube_dimension: ResMut<NCubeDimension>,
    mut ncube: ResMut<NCube>,
    mut ncube_rotations: ResMut<NCubeRotations>,
    mut ncube_planes_of_rotation: ResMut<NCubePlanesOfRotation>,
    mut ncube_edge_color: ResMut<NCubeEdgeColor>,
    mut ncube_face_color: ResMut<NCubeFaceColor>,
    mut ncube_edge_thickness: ResMut<NCubeEdgeThickness>,
    mut ncube_vertices_3d: ResMut<NCubeVertices3D>,
    mut ncube_is_paused: ResMut<NCubeIsPaused>,
    mut contexts: EguiContexts,
    mut q_camera_transform: Query<&mut Transform, With<Camera>>,
    mut drag_drop_event: EventReader<FileDragAndDrop>,
    mut is_hovering_file: ResMut<IsHoveringFile>,
) {
    egui::Window::new("settings")
        .default_pos((0.0, 0.0))
        .show(contexts.ctx_mut(), |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                egui::Grid::new("grid")
                    .num_columns(2)
                    .spacing([40.0, 4.0])
                    .striped(true)
                    .show(ui, |ui| {
                        ui.label("dimensions:");
                        let mut d = **ncube_dimension;
                        ui.add(egui::Slider::new(&mut d, 3..=9));
                        if d != **ncube_dimension {
                            **ncube_dimension = d;
                        }
                        ui.end_row();

                        ui.label("vertices:");
                        ui.label(format!("{}", ncube.vertices.0.len()));
                        ui.end_row();

                        ui.label("edges:");
                        ui.label(format!("{}", ncube.edges.0.len()));
                        ui.end_row();

                        ui.label("faces:");
                        ui.label(format!("{}", ncube.faces.0.len() / 2));
                        ui.end_row();

                        ui.label("edge thickness:");
                        ui.add(egui::Slider::new(&mut **ncube_edge_thickness, 0.0..=0.025));
                        ui.end_row();

                        ui.label("edge color:");
                        let mut color: [f32; 4] = [
                            ncube_edge_color.r(),
                            ncube_edge_color.g(),
                            ncube_edge_color.b(),
                            ncube_edge_color.a(),
                        ];
                        ui.color_edit_button_rgba_unmultiplied(&mut color);
                        ui.end_row();
                        **ncube_edge_color = Color::from(color);

                        ui.label("face color:");
                        let mut color: [f32; 4] = [
                            ncube_face_color.r(),
                            ncube_face_color.g(),
                            ncube_face_color.b(),
                            ncube_face_color.a(),
                        ];
                        ui.color_edit_button_rgba_unmultiplied(&mut color);
                        ui.end_row();
                        **ncube_face_color = Color::from(color);

                        for i in 0..ncube_planes_of_rotation.len() {
                            let plane = ncube_planes_of_rotation[i];
                            let (angle, vel) = *ncube_rotations.get(&plane).unwrap();
                            let mut tmp = vel;
                            ui.label(format!("q{}q{} w:", plane.0 + 1, plane.1 + 1));
                            ui.add(egui::Slider::new(&mut tmp, -3.0..=3.0));
                            ui.end_row();
                            ncube_rotations.insert(plane, (angle, tmp));
                        }

                        if ui.button("reset").clicked() {
                            *ncube_dimension = NCubeDimension::default();
                            *ncube = NCube::default();
                            *ncube_planes_of_rotation = NCubePlanesOfRotation::default();
                            *ncube_rotations = NCubeRotations::default();
                            *q_camera_transform.get_single_mut().unwrap() =
                                crate::camera::get_default_camera_transform();
                            *ncube_edge_thickness = NCubeEdgeThickness::default();
                            *ncube_face_color = NCubeFaceColor::default();
                            *ncube_edge_color = NCubeEdgeColor::default();
                        }
                        if **ncube_is_paused {
                            ui.colored_label(egui::Color32::RED, "paused");
                        }
                        ui.end_row();

                        if ui.button("export to data file").clicked() {
                            if let Some(home_dir) = &home::home_dir() {
                                if let Ok(mut file) = std::fs::File::create(
                                    std::path::Path::new(home_dir).join(format!(
                                        "{}cube-{}.data",
                                        **ncube_dimension,
                                        std::time::SystemTime::now()
                                            .duration_since(std::time::UNIX_EPOCH)
                                            .unwrap()
                                            .as_secs()
                                    )),
                                ) {
                                    let camera_transform =
                                        *q_camera_transform.get_single().unwrap();
                                    let ncube_data = NCubeData {
                                        dimension: **ncube_dimension,
                                        rotations: ncube_rotations
                                            .iter()
                                            .map(|(k, v)| (k.0, k.1, v.0, v.1))
                                            .collect(),
                                        edge_thickness: **ncube_edge_thickness,
                                        edge_color: **ncube_edge_color,
                                        face_color: **ncube_face_color,
                                        camera_transform: CameraTransform {
                                            translation: camera_transform.translation,
                                            rotation: camera_transform.rotation,
                                            scale: camera_transform.scale,
                                        },
                                    };
                                    serde_json::to_writer_pretty(&mut file, &ncube_data)
                                        .unwrap_or_else(|_| {});
                                }
                            }
                        }

                        ui.colored_label(
                            if **is_hovering_file {
                                egui::Color32::GREEN
                            } else {
                                egui::Color32::LIGHT_GRAY
                            },
                            "drop data file",
                        );
                        if let Some(e) = drag_drop_event.iter().nth(0) {
                            match e {
                                FileDragAndDrop::HoveredFile { .. } => {
                                    **is_hovering_file = true;
                                }
                                FileDragAndDrop::DroppedFile { path_buf, .. } => {
                                    let file = std::fs::File::open(&path_buf).unwrap();
                                    let reader = std::io::BufReader::new(file);
                                    if let Ok(data) =
                                        serde_json::from_reader::<_, NCubeData>(reader)
                                    {
                                        *q_camera_transform.get_single_mut().unwrap() = Transform {
                                            translation: data.camera_transform.translation,
                                            scale: data.camera_transform.scale,
                                            rotation: data.camera_transform.rotation,
                                        };
                                        **ncube_is_paused = true;
                                        **ncube_edge_thickness = data.edge_thickness;
                                        **ncube_edge_color = data.edge_color;
                                        **ncube_face_color = data.face_color;
                                        **ncube_dimension = data.dimension;
                                        **ncube = InnerNCube::new(**ncube_dimension, S);
                                        **ncube_rotations = std::collections::HashMap::new();
                                        **ncube_planes_of_rotation = Vec::new();
                                        let mut angles = Vec::new();
                                        for (d1, d2, angle, vel) in data.rotations {
                                            ncube_rotations.insert((d1, d2), (angle, vel));
                                            ncube_planes_of_rotation.push((d1, d2));
                                            angles.push(angle);
                                        }
                                        **ncube_vertices_3d = ncube
                                            .rotate(&ncube_planes_of_rotation, &angles)
                                            .perspective_project_vertices();
                                    }
                                    **is_hovering_file = false;
                                }
                                _ => {
                                    **is_hovering_file = false;
                                }
                            }
                        }
                        ui.end_row();
                    });
            });
        });
}
