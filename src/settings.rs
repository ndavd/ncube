use crate::NCube;
use crate::NCubeDimension;
use crate::NCubeEdgeColor;
use crate::NCubeEdgeThickness;
use crate::NCubeFaceColor;
use crate::NCubeIsPaused;
use crate::NCubePlanesOfRotation;
use crate::NCubeRotations;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin).add_systems(Update, info_panel);
    }
}

fn info_panel(
    mut ncube_dimension: ResMut<NCubeDimension>,
    mut ncube: ResMut<NCube>,
    mut ncube_rotations: ResMut<NCubeRotations>,
    mut ncube_planes_of_rotation: ResMut<NCubePlanesOfRotation>,
    mut ncube_edge_color: ResMut<NCubeEdgeColor>,
    mut ncube_face_color: ResMut<NCubeFaceColor>,
    mut ncube_edge_thickness: ResMut<NCubeEdgeThickness>,
    ncube_is_paused: Res<NCubeIsPaused>,
    mut contexts: EguiContexts,
    mut q_camera_transform: Query<&mut Transform, With<Camera>>,
) {
    egui::Window::new("settings")
        .default_pos((0.0, 0.0))
        .show(contexts.ctx_mut(), |ui| {
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
                        ui.label(
                            egui::RichText::new("paused").color(egui::Color32::from_rgb(255, 0, 0)),
                        );
                    }
                    ui.end_row();
                });
        });
}
