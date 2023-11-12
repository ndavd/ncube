use crate::camera::get_default_camera_transform;
use crate::impl_default;
use crate::ncube::NCube as InnerNCube;
use crate::resources::SIZE;
use crate::NCube;
use crate::NCubeDimension;
use crate::NCubeEdgeColor;
use crate::NCubeEdgeThickness;
use crate::NCubeFaceColor;
use crate::NCubeIsPaused;
use crate::NCubePlanesOfRotation;
use crate::NCubeRotations;
use crate::NCubeUnlit;
use crate::NCubeVertices3D;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use egui::Ui;
use egui_file::FileDialog;

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<IsHoveringFile>()
            .init_resource::<Dialog>()
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
impl_default!(CameraTransform => {
    let transform = get_default_camera_transform();
    Self {
        translation: transform.translation,
        rotation: transform.rotation,
        scale: transform.scale,
    }
});

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct NCubeData {
    dimension: usize,
    rotations: Vec<(usize, usize, f32, f32)>,
    #[serde(default)]
    camera_transform: CameraTransform,
    #[serde(default)]
    edge_thickness: f32,
    #[serde(default)]
    edge_color: Color,
    #[serde(default)]
    face_color: Color,
    #[serde(default)]
    unlit: bool,
}

#[derive(Resource, Deref, DerefMut, Default)]
struct IsHoveringFile(bool);

#[derive(Resource, Deref, DerefMut, Default)]
struct Dialog(Option<egui_file::FileDialog>);

fn info_panel(
    mut ncube_dimension: ResMut<NCubeDimension>,
    mut ncube: ResMut<NCube>,
    mut ncube_rotations: ResMut<NCubeRotations>,
    mut ncube_planes_of_rotation: ResMut<NCubePlanesOfRotation>,
    mut ncube_edge_color: ResMut<NCubeEdgeColor>,
    mut ncube_face_color: ResMut<NCubeFaceColor>,
    mut ncube_edge_thickness: ResMut<NCubeEdgeThickness>,
    mut ncube_vertices_3d: ResMut<NCubeVertices3D>,
    mut ncube_unlit: ResMut<NCubeUnlit>,
    mut ncube_is_paused: ResMut<NCubeIsPaused>,
    mut contexts: EguiContexts,
    mut q_camera_transform: Query<&mut Transform, With<Camera>>,
    mut drag_drop_event: EventReader<FileDragAndDrop>,
    mut is_hovering_file: ResMut<IsHoveringFile>,
    mut dialog: ResMut<Dialog>,
) {
    let context = contexts.ctx_mut();
    egui::Window::new("settings")
        .default_pos((0.0, 0.0))
        .show(&context.clone(), |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                egui::Grid::new("grid")
                    .num_columns(2)
                    .spacing([40.0, 4.0])
                    .striped(true)
                    .show(ui, |ui| {
                        render_ui(
                            ui,
                            context,
                            &mut ncube_dimension,
                            &mut ncube,
                            &mut ncube_rotations,
                            &mut ncube_planes_of_rotation,
                            &mut ncube_edge_color,
                            &mut ncube_face_color,
                            &mut ncube_edge_thickness,
                            &mut ncube_vertices_3d,
                            &mut ncube_unlit,
                            &mut ncube_is_paused,
                            &mut q_camera_transform,
                            &mut drag_drop_event,
                            &mut is_hovering_file,
                            &mut dialog,
                        )
                    });
            });
        });
}

fn render_ui(
    ui: &mut Ui,
    context: &mut egui::Context,
    ncube_dimension: &mut ResMut<NCubeDimension>,
    ncube: &mut ResMut<NCube>,
    ncube_rotations: &mut ResMut<NCubeRotations>,
    ncube_planes_of_rotation: &mut ResMut<NCubePlanesOfRotation>,
    ncube_edge_color: &mut ResMut<NCubeEdgeColor>,
    ncube_face_color: &mut ResMut<NCubeFaceColor>,
    ncube_edge_thickness: &mut ResMut<NCubeEdgeThickness>,
    ncube_vertices_3d: &mut ResMut<NCubeVertices3D>,
    ncube_unlit: &mut ResMut<NCubeUnlit>,
    ncube_is_paused: &mut ResMut<NCubeIsPaused>,
    q_camera_transform: &mut Query<&mut Transform, With<Camera>>,
    drag_drop_event: &mut EventReader<FileDragAndDrop>,
    is_hovering_file: &mut ResMut<IsHoveringFile>,
    dialog: &mut ResMut<Dialog>,
) {
    render_dimensions(ui, ncube_dimension);
    render_ncube_info(
        ui,
        ncube.vertices.0.len(),
        ncube.edges.0.len(),
        ncube.faces.0.len() / 2,
    );
    render_lighting(ui, ncube_unlit);
    render_edge_thickness(ui, ncube_edge_thickness);
    render_edge_color(ui, ncube_edge_color);
    render_face_color(ui, ncube_face_color);
    render_planes_of_rotation(ui, ncube_rotations, ncube_planes_of_rotation);
    render_reset(
        ui,
        ncube_dimension,
        ncube,
        ncube_rotations,
        ncube_planes_of_rotation,
        ncube_edge_color,
        ncube_face_color,
        ncube_edge_thickness,
        ncube_is_paused,
        q_camera_transform,
    );
    render_export_data_file(
        ui,
        context,
        dialog,
        ncube_dimension,
        q_camera_transform,
        ncube_rotations,
        ncube_edge_color,
        ncube_face_color,
        ncube_edge_thickness,
        ncube_unlit,
    );
    render_drop_data_file(
        ui,
        ncube_dimension,
        ncube,
        ncube_rotations,
        ncube_planes_of_rotation,
        ncube_edge_color,
        ncube_face_color,
        ncube_edge_thickness,
        ncube_vertices_3d,
        ncube_unlit,
        ncube_is_paused,
        q_camera_transform,
        drag_drop_event,
        is_hovering_file,
    );
}

macro_rules! render_row {
    ($label:expr, $ui:ident => $content:expr) => {
        $ui.label(format!("{}:", $label));
        $content
        $ui.end_row()
    }
}

fn render_dimensions(ui: &mut Ui, ncube_dimension: &mut ResMut<NCubeDimension>) {
    render_row!("dimensions", ui => {
        let mut d = ***ncube_dimension;
        ui.add(egui::Slider::new(&mut d, 3..=9));
        if d != ***ncube_dimension {
            ***ncube_dimension = d;
        }
    });
}

fn render_ncube_info(ui: &mut Ui, vertices: usize, edges: usize, faces: usize) {
    render_row!("vertices", ui => { ui.label(vertices.to_string()); });
    render_row!("edges", ui => { ui.label(edges.to_string()); });
    render_row!("faces", ui => { ui.label(faces.to_string()); });
}

fn render_lighting(ui: &mut Ui, ncube_unlit: &mut ResMut<NCubeUnlit>) {
    render_row!("realistic lighting", ui => {
        let mut lit = !***ncube_unlit;
        ui.add(egui::Checkbox::new(&mut lit, ""));
        ***ncube_unlit = !lit;
    });
}

fn render_edge_thickness(ui: &mut Ui, ncube_edge_thickness: &mut ResMut<NCubeEdgeThickness>) {
    render_row!("edge thickness", ui => {
        ui.add(egui::Slider::new(&mut ***ncube_edge_thickness, 0.0..=0.025));
    });
}

fn render_edge_color(ui: &mut Ui, ncube_edge_color: &mut ResMut<NCubeEdgeColor>) {
    render_row!("edge color", ui => {
        let mut color: [f32; 4] = [
            ncube_edge_color.r(),
            ncube_edge_color.g(),
            ncube_edge_color.b(),
            ncube_edge_color.a(),
        ];
        ui.color_edit_button_rgba_unmultiplied(&mut color);
        ***ncube_edge_color = Color::from(color);
    });
}

fn render_face_color(ui: &mut Ui, ncube_face_color: &mut ResMut<NCubeFaceColor>) {
    render_row!("face color", ui => {
        let mut color: [f32; 4] = [
            ncube_face_color.r(),
            ncube_face_color.g(),
            ncube_face_color.b(),
            ncube_face_color.a(),
        ];
        ui.color_edit_button_rgba_unmultiplied(&mut color);
        ***ncube_face_color = Color::from(color);
    });
}

fn render_planes_of_rotation(
    ui: &mut Ui,
    ncube_rotations: &mut ResMut<NCubeRotations>,
    ncube_planes_of_rotation: &mut ResMut<NCubePlanesOfRotation>,
) {
    for i in 0..ncube_planes_of_rotation.len() {
        let plane = ncube_planes_of_rotation[i];
        let (angle, vel) = *ncube_rotations.get(&plane).unwrap();
        let mut tmp = vel;
        render_row!(format!("q{}q{} w:", plane.0 + 1, plane.1 + 1), ui => {
            ui.add(egui::Slider::new(&mut tmp, -3.0..=3.0));
        });
        ncube_rotations.insert(plane, (angle, tmp));
    }
}

fn render_reset(
    ui: &mut Ui,
    ncube_dimension: &mut ResMut<NCubeDimension>,
    ncube: &mut ResMut<NCube>,
    ncube_rotations: &mut ResMut<NCubeRotations>,
    ncube_planes_of_rotation: &mut ResMut<NCubePlanesOfRotation>,
    ncube_edge_color: &mut ResMut<NCubeEdgeColor>,
    ncube_face_color: &mut ResMut<NCubeFaceColor>,
    ncube_edge_thickness: &mut ResMut<NCubeEdgeThickness>,
    ncube_is_paused: &mut ResMut<NCubeIsPaused>,
    q_camera_transform: &mut Query<&mut Transform, With<Camera>>,
) {
    if ui.button("reset").clicked() {
        **ncube_dimension = NCubeDimension::default();
        **ncube = NCube::default();
        **ncube_planes_of_rotation = NCubePlanesOfRotation::default();
        **ncube_rotations = NCubeRotations::default();
        *q_camera_transform.get_single_mut().unwrap() =
            crate::camera::get_default_camera_transform();
        **ncube_edge_thickness = NCubeEdgeThickness::default();
        **ncube_face_color = NCubeFaceColor::default();
        **ncube_edge_color = NCubeEdgeColor::default();
    }
    if ***ncube_is_paused {
        ui.colored_label(egui::Color32::RED, "paused");
    }
    ui.end_row();
}

fn render_export_data_file(
    ui: &mut Ui,
    context: &mut egui::Context,
    dialog: &mut ResMut<Dialog>,
    ncube_dimension: &mut ResMut<NCubeDimension>,
    q_camera_transform: &mut Query<&mut Transform, With<Camera>>,
    ncube_rotations: &mut ResMut<NCubeRotations>,
    ncube_edge_color: &mut ResMut<NCubeEdgeColor>,
    ncube_face_color: &mut ResMut<NCubeFaceColor>,
    ncube_edge_thickness: &mut ResMut<NCubeEdgeThickness>,
    ncube_unlit: &mut ResMut<NCubeUnlit>,
) {
    if ui.button("export to data file").clicked() {
        let mut d = FileDialog::select_folder(home::home_dir())
            .title(format!("select folder to save data file").as_str());
        d.open();
        ***dialog = Some(d);
    }

    let d = match &mut ***dialog {
        Some(v) => {
            v.show(context);
            if !v.selected() {
                return;
            }
            v
        }
        None => {
            return;
        }
    };

    let file_name = format!(
        "{}cube-{}.data",
        ***ncube_dimension,
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    );

    let mut file = match d
        .path()
        .and_then(|file_path| std::fs::File::create(file_path.join(file_name)).ok())
    {
        Some(v) => v,
        None => {
            return;
        }
    };

    let camera_transform = *q_camera_transform.get_single().unwrap();
    let ncube_data = NCubeData {
        dimension: ***ncube_dimension,
        rotations: ncube_rotations
            .iter()
            .map(|(k, v)| (k.0, k.1, v.0, v.1))
            .collect(),
        edge_thickness: ***ncube_edge_thickness,
        edge_color: ***ncube_edge_color,
        face_color: ***ncube_face_color,
        camera_transform: CameraTransform {
            translation: camera_transform.translation,
            rotation: camera_transform.rotation,
            scale: camera_transform.scale,
        },
        unlit: ***ncube_unlit,
    };
    serde_json::to_writer_pretty(&mut file, &ncube_data).unwrap_or_else(|_| {});
}

fn render_drop_data_file(
    ui: &mut Ui,
    ncube_dimension: &mut ResMut<NCubeDimension>,
    ncube: &mut ResMut<NCube>,
    ncube_rotations: &mut ResMut<NCubeRotations>,
    ncube_planes_of_rotation: &mut ResMut<NCubePlanesOfRotation>,
    ncube_edge_color: &mut ResMut<NCubeEdgeColor>,
    ncube_face_color: &mut ResMut<NCubeFaceColor>,
    ncube_edge_thickness: &mut ResMut<NCubeEdgeThickness>,
    ncube_vertices_3d: &mut ResMut<NCubeVertices3D>,
    ncube_unlit: &mut ResMut<NCubeUnlit>,
    ncube_is_paused: &mut ResMut<NCubeIsPaused>,
    q_camera_transform: &mut Query<&mut Transform, With<Camera>>,
    drag_drop_event: &mut EventReader<FileDragAndDrop>,
    is_hovering_file: &mut ResMut<IsHoveringFile>,
) {
    ui.colored_label(
        if ***is_hovering_file {
            egui::Color32::GREEN
        } else {
            egui::Color32::LIGHT_GRAY
        },
        "drop data file",
    );
    ui.end_row();

    let event = match drag_drop_event.read().nth(0) {
        Some(v) => v,
        None => {
            return;
        }
    };

    match event {
        FileDragAndDrop::HoveredFile { .. } => {
            ***is_hovering_file = true;
        }
        FileDragAndDrop::DroppedFile { path_buf, .. } => {
            let file = std::fs::File::open(&path_buf).unwrap();
            let reader = std::io::BufReader::new(file);
            match serde_json::from_reader::<_, NCubeData>(reader) {
                Ok(data) => {
                    *q_camera_transform.get_single_mut().unwrap() = Transform {
                        translation: data.camera_transform.translation,
                        scale: data.camera_transform.scale,
                        rotation: data.camera_transform.rotation,
                    };
                    ***ncube_is_paused = true;
                    ***ncube_edge_thickness = data.edge_thickness;
                    ***ncube_edge_color = data.edge_color;
                    ***ncube_face_color = data.face_color;
                    ***ncube_unlit = data.unlit;
                    ***ncube_dimension = data.dimension;
                    ***ncube = InnerNCube::new(***ncube_dimension, SIZE);
                    ***ncube_rotations = std::collections::HashMap::new();
                    ***ncube_planes_of_rotation = Vec::new();
                    let mut angles = Vec::new();
                    for (d1, d2, angle, vel) in data.rotations {
                        ncube_rotations.insert((d1, d2), (angle, vel));
                        ncube_planes_of_rotation.push((d1, d2));
                        angles.push(angle);
                    }
                    ***ncube_vertices_3d = ncube
                        .rotate(&ncube_planes_of_rotation, &angles)
                        .perspective_project_vertices();
                }
                Err(e) => println!("ERR {e}"),
            }
            ***is_hovering_file = false;
        }
        _ => {
            ***is_hovering_file = false;
        }
    }
}
