use crate::NCube;
use crate::NCubeDimension;
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
    mut contexts: EguiContexts,
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
                    ui.add(egui::Slider::new(&mut ncube_dimension.0, 3..=9));
                    ui.end_row();

                    ui.label("vertices:");
                    ui.label(format!("{}", ncube.settings.vertices.0.len()));
                    ui.end_row();

                    ui.label("edges:");
                    ui.label(format!("{}", ncube.settings.edges.0.len()));
                    ui.end_row();

                    ui.label("faces:");
                    ui.label(format!("{}", ncube.settings.faces.0.len() / 2));
                    ui.end_row();

                    ui.label("color:");
                    let mut color: [f32; 3] = [ncube.color.r(), ncube.color.g(), ncube.color.b()];
                    ui.color_edit_button_rgb(&mut color);
                    ui.end_row();
                    ncube.color = Color::from(color);

                    for i in 0..ncube.planes_of_rotation.len() {
                        let plane = ncube.planes_of_rotation[i];
                        let (angle, vel) = *ncube.rotations.get(&plane).unwrap();
                        let mut tmp = vel;
                        ui.label(format!("q{}q{} w:", plane.0 + 1, plane.1 + 1));
                        ui.add(egui::Slider::new(&mut tmp, -3.0..=3.0));
                        ui.end_row();
                        ncube.rotations.insert(plane, (angle, tmp));
                    }

                    if ui.button("reset").clicked() {
                        *ncube_dimension = NCubeDimension::default();
                        *ncube = NCube {
                            settings: ncube.settings.clone(),
                            ..default()
                        };
                    }
                    if ncube.paused {
                        ui.label(
                            egui::RichText::new("paused").color(egui::Color32::from_rgb(255, 0, 0)),
                        );
                    }
                    ui.end_row();
                });
        });
}
