use crate::camera::{get_default_camera_projection, get_default_camera_transform};
use crate::impl_default;
use crate::ncube::NCube as InnerNCube;
use crate::resources::{FileDialog, IsHoveringFile, ShowControls, SIZE};
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
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn get_drag_drop_data() -> Option<String>;
    fn export_to_data_file(dimension: usize, data: String);
}

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
            .add_systems(Update, (info_panel, controls_panel));
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
    rotations: Vec<(usize, usize, f64, f64)>,
    #[serde(default)]
    camera_transform: CameraTransform,
    #[serde(default)]
    orthographic_projection: bool,
    #[serde(default)]
    edge_thickness: f32,
    #[serde(default)]
    edge_color: Color,
    #[serde(default)]
    face_color: Color,
    #[serde(default)]
    unlit: bool,
}

fn info_panel(
    (
        mut ncube_dimension,
        mut ncube,
        mut ncube_rotations,
        mut ncube_planes_of_rotation,
        mut ncube_edge_color,
        mut ncube_face_color,
        mut ncube_edge_thickness,
        mut ncube_vertices_3d,
        mut ncube_unlit,
        mut ncube_is_paused,
    ): (
        ResMut<NCubeDimension>,
        ResMut<NCube>,
        ResMut<NCubeRotations>,
        ResMut<NCubePlanesOfRotation>,
        ResMut<NCubeEdgeColor>,
        ResMut<NCubeFaceColor>,
        ResMut<NCubeEdgeThickness>,
        ResMut<NCubeVertices3D>,
        ResMut<NCubeUnlit>,
        ResMut<NCubeIsPaused>,
    ),
    (
        mut contexts,
        mut q_camera,
        mut drag_drop_event,
        mut show_controls,
        mut is_hovering_file,
        mut dialog,
    ): (
        EguiContexts,
        Query<(&mut Transform, &mut Projection), With<Camera>>,
        EventReader<FileDragAndDrop>,
        ResMut<ShowControls>,
        ResMut<IsHoveringFile>,
        ResMut<FileDialog>,
    ),
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
                            &mut show_controls,
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
                            &mut q_camera,
                            &mut drag_drop_event,
                            &mut is_hovering_file,
                            &mut dialog,
                        )
                    });
            });
        });
}

fn controls_panel(mut contexts: EguiContexts, mut show_controls: ResMut<ShowControls>) {
    egui::Window::new("controls")
        .open(&mut show_controls)
        .vscroll(false)
        .resizable(true)
        .show(&contexts.ctx_mut(), |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                egui::Grid::new("controls-grid")
                    .num_columns(2)
                    .spacing([40.0, 0.0])
                    .striped(true)
                    .show(ui, |ui| {
                        let mono = |ui: &mut Ui, text: &str| {
                            ui.label(
                                egui::RichText::new(text)
                                    .monospace()
                                    .color(egui::Color32::LIGHT_BLUE),
                            )
                        };

                        ui.label("load data file");
                        mono(ui, "drag and drop file");
                        ui.end_row();

                        ui.label("pause");
                        mono(ui, "space");
                        ui.end_row();

                        ui.label("rotate");
                        mono(ui, "hold right mouse button + move");
                        ui.end_row();

                        ui.label("zoom");
                        mono(ui, "hold control + scroll");
                        ui.end_row();

                        ui.label("toggle fullscreen");
                        mono(ui, "F");
                        ui.end_row();
                    })
            })
        });
}

fn render_ui(
    ui: &mut Ui,
    context: &mut egui::Context,
    show_controls: &mut ResMut<ShowControls>,
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
    q_camera: &mut Query<(&mut Transform, &mut Projection), With<Camera>>,
    drag_drop_event: &mut EventReader<FileDragAndDrop>,
    is_hovering_file: &mut ResMut<IsHoveringFile>,
    file_dialog: &mut ResMut<FileDialog>,
) {
    let (mut camera_transform, mut camera_projection) = q_camera.get_single_mut().unwrap();
    render_controls_and_reset(
        ui,
        show_controls,
        ncube_dimension,
        ncube,
        ncube_rotations,
        ncube_planes_of_rotation,
        ncube_edge_color,
        ncube_face_color,
        ncube_edge_thickness,
        ncube_is_paused,
        &mut camera_transform,
    );
    render_export_data_file(
        ui,
        context,
        file_dialog,
        ncube_dimension,
        ncube_rotations,
        ncube_edge_color,
        ncube_face_color,
        ncube_edge_thickness,
        ncube_unlit,
        &camera_transform,
        &camera_projection,
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
        drag_drop_event,
        is_hovering_file,
        &mut camera_transform,
        &mut camera_projection,
    );
    render_dimensions(ui, ncube_dimension);
    render_ncube_info(
        ui,
        ncube.vertices.0.len(),
        ncube.edges.0.len(),
        ncube.faces.0.len() / 2,
    );
    render_camera_projection(
        ui,
        &mut camera_projection,
        camera_transform.translation.length(),
    );
    render_lighting(ui, ncube_unlit);
    render_edge_thickness(ui, ncube_edge_thickness);
    render_edge_color(ui, ncube_edge_color);
    render_face_color(ui, ncube_face_color);
    render_planes_of_rotation(ui, ncube_rotations, ncube_planes_of_rotation);
}

macro_rules! render_row {
    ($label:expr, $ui:ident => $content:expr) => {
        $ui.label(format!("{}:", $label));
        $content
        $ui.end_row()
    }
}

fn render_camera_projection(ui: &mut Ui, camera_projection: &mut Projection, d: f32) {
    render_row!("camera projection", ui => {
        ui.scope(|ui| {
            let mut is_ortho = matches!(camera_projection, Projection::Orthographic(_));
            let backup = is_ortho;
            ui.radio_value(&mut is_ortho, false, "perspective");
            ui.radio_value(&mut is_ortho, true, "orthographic");
            if is_ortho != backup {
                *camera_projection = get_default_camera_projection(is_ortho.then(|| d));
            }
        });
    });
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
        render_row!(format!("q{}q{} w", plane.0 + 1, plane.1 + 1), ui => {
            ui.add(egui::Slider::new(&mut tmp, -3.0..=3.0));
        });
        if tmp != vel {
            ncube_rotations.insert(plane, (angle, tmp));
        }
    }
}

fn render_controls_and_reset(
    ui: &mut Ui,
    show_controls: &mut ResMut<ShowControls>,
    ncube_dimension: &mut ResMut<NCubeDimension>,
    ncube: &mut ResMut<NCube>,
    ncube_rotations: &mut ResMut<NCubeRotations>,
    ncube_planes_of_rotation: &mut ResMut<NCubePlanesOfRotation>,
    ncube_edge_color: &mut ResMut<NCubeEdgeColor>,
    ncube_face_color: &mut ResMut<NCubeFaceColor>,
    ncube_edge_thickness: &mut ResMut<NCubeEdgeThickness>,
    ncube_is_paused: &mut ResMut<NCubeIsPaused>,
    camera_transform: &mut Transform,
) {
    ui.scope(|ui| {
        ui.spacing_mut().item_spacing = egui::vec2(4.0, 0.0);
        if ui.button("controls").clicked() {
            ***show_controls = true;
        }
        if ui.button("reset").clicked() {
            **ncube_dimension = NCubeDimension::default();
            **ncube = NCube::default();
            **ncube_planes_of_rotation = NCubePlanesOfRotation::default();
            **ncube_rotations = NCubeRotations::default();
            *camera_transform = crate::camera::get_default_camera_transform();
            **ncube_edge_thickness = NCubeEdgeThickness::default();
            **ncube_face_color = NCubeFaceColor::default();
            **ncube_edge_color = NCubeEdgeColor::default();
        }
    });
    if ***ncube_is_paused {
        ui.colored_label(egui::Color32::RED, "paused");
    } else {
        ui.colored_label(egui::Color32::GREEN, "running");
    }
    ui.end_row();
}

fn render_export_data_file(
    ui: &mut Ui,
    _context: &mut egui::Context,
    _file_dialog: &mut ResMut<FileDialog>,
    ncube_dimension: &ResMut<NCubeDimension>,
    ncube_rotations: &ResMut<NCubeRotations>,
    ncube_edge_color: &ResMut<NCubeEdgeColor>,
    ncube_face_color: &ResMut<NCubeFaceColor>,
    ncube_edge_thickness: &ResMut<NCubeEdgeThickness>,
    ncube_unlit: &ResMut<NCubeUnlit>,
    camera_transform: &Transform,
    camera_projection: &Projection,
) {
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
        orthographic_projection: matches!(camera_projection, Projection::Orthographic(_)),
    };

    if ui.button("export to data file").clicked() {
        #[cfg(not(target_family = "wasm"))]
        {
            let mut dialog = egui_file::FileDialog::select_folder(home::home_dir())
                .title(format!("select folder to save data file").as_str());
            dialog.open();
            ***_file_dialog = Some(dialog);
        }
        #[cfg(target_family = "wasm")]
        if let Ok(data) = serde_json::to_string(&ncube_data) {
            export_to_data_file(***ncube_dimension, data);
        }
    }

    #[cfg(not(target_family = "wasm"))]
    {
        let dialog = match &mut ***_file_dialog {
            Some(v) => {
                v.show(_context);
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

        let mut file = match dialog
            .path()
            .and_then(|file_path| std::fs::File::create(file_path.join(file_name)).ok())
        {
            Some(v) => v,
            None => {
                return;
            }
        };

        serde_json::to_writer_pretty(&mut file, &ncube_data).unwrap_or_else(|_| {});
    }
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
    drag_drop_event: &mut EventReader<FileDragAndDrop>,
    is_hovering_file: &mut ResMut<IsHoveringFile>,
    camera_transform: &mut Transform,
    camera_projection: &mut Projection,
) {
    let mut handle_ncube_data = |data: NCubeData| {
        *camera_transform = Transform {
            translation: data.camera_transform.translation,
            scale: data.camera_transform.scale,
            rotation: data.camera_transform.rotation,
        };
        *camera_projection = get_default_camera_projection(
            data.orthographic_projection
                .then(|| camera_transform.translation.length()),
        );
        ***ncube_is_paused = true;
        ***ncube_edge_thickness = data.edge_thickness;
        ***ncube_edge_color = data.edge_color;
        ***ncube_face_color = data.face_color;
        ***ncube_unlit = data.unlit;
        ***ncube_dimension = data.dimension;
        ***ncube = InnerNCube::new(***ncube_dimension, SIZE.into());
        ***ncube_rotations = std::collections::HashMap::new();
        ***ncube_planes_of_rotation = Vec::new();
        let mut angles = Vec::new();
        for (d1, d2, angle, vel) in data.rotations {
            let angle = angle % std::f64::consts::TAU; // To ensure backwards compatibility
            ncube_rotations.insert((d1, d2), (angle, vel));
            ncube_planes_of_rotation.push((d1, d2));
            angles.push(angle);
        }
        ***ncube_vertices_3d = ncube
            .rotate(&ncube_planes_of_rotation, &angles)
            .perspective_project_vertices();
    };

    ui.colored_label(
        if ***is_hovering_file {
            egui::Color32::GREEN
        } else {
            egui::Color32::LIGHT_GRAY
        },
        "drop data file",
    );
    ui.end_row();

    if cfg!(target_family = "wasm") {
        if let Some(data_str) = get_drag_drop_data() {
            match serde_json::from_str::<NCubeData>(&data_str) {
                Ok(data) => handle_ncube_data(data),
                Err(e) => eprintln!("ERR {e}"),
            }
        }
        return;
    }

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
                Ok(data) => handle_ncube_data(data),
                Err(e) => eprintln!("ERR {e}"),
            }
            ***is_hovering_file = false;
        }
        _ => {
            ***is_hovering_file = false;
        }
    }
}
