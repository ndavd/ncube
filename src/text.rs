use crate::resources::FontHandle;
use crate::NCubeDimension;
use crate::NCubePlanesOfRotation;
use crate::NCubeRotations;
use bevy::prelude::*;

pub struct TextPlugin;
impl Plugin for TextPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_title_text).add_systems(
            Update,
            (spawn_info_text, update_title_text, update_info_text).chain(),
        );
    }
}

#[derive(Component)]
struct TitleText;
#[derive(Component)]
struct InfoText;

fn spawn_title_text(mut commands: Commands, mut fonts: ResMut<Assets<Font>>) {
    let font = Vec::from(include_bytes!("../assets/gohufont-14.ttf") as &[u8]);
    let font = Font::try_from_bytes(font).unwrap();
    let font = fonts.add(font);

    commands.insert_resource(FontHandle(font.clone()));

    let text_style = TextStyle {
        font: font.clone(),
        font_size: 30.0,
        color: Color::WHITE,
    };
    commands.spawn((
        TextBundle {
            text: Text::from_section("n-cube", text_style).with_alignment(TextAlignment::Right),
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                right: Val::Px(20.0),
                ..default()
            },
            ..default()
        },
        TitleText,
    ));
    let text_style = TextStyle {
        font,
        font_size: 14.0,
        color: Color::ANTIQUE_WHITE,
    };
    commands.spawn(TextBundle {
        text: Text::from_section(
            format!(
                "{} v{} - {}\n{}",
                env!("CARGO_PKG_NAME"),
                env!("CARGO_PKG_VERSION"),
                env!("CARGO_PKG_DESCRIPTION"),
                env!("CARGO_PKG_AUTHORS"),
            ),
            text_style,
        ),
        style: Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(10.0),
            left: Val::Px(20.0),
            ..default()
        },
        ..default()
    });
}

fn update_title_text(
    ncube_dimension: Res<NCubeDimension>,
    mut q_title_text: Query<&mut Text, With<TitleText>>,
) {
    if ncube_dimension.is_changed() {
        let mut title_text = q_title_text.get_single_mut().unwrap();
        title_text.sections[0].value = format!("{}-cube", **ncube_dimension);
    }
}

fn spawn_info_text(
    mut commands: Commands,
    ncube_planes_of_rotation: Res<NCubePlanesOfRotation>,
    q_info_text_entities: Query<Entity, With<InfoText>>,
    font_handle: Res<FontHandle>,
) {
    if !ncube_planes_of_rotation.is_changed() {
        return;
    }

    q_info_text_entities.iter().for_each(|entity| {
        commands.entity(entity).despawn();
    });

    let text_style = TextStyle {
        font: font_handle.clone(),
        font_size: 20.0,
        color: Color::WHITE,
    };
    for (i, plane) in ncube_planes_of_rotation.iter().enumerate() {
        let plane_info = format!("q{}q{}: {:6.1}deg", plane.0 + 1, plane.1 + 1, 0.0);
        commands.spawn((
            TextBundle {
                text: Text::from_section(plane_info, text_style.clone())
                    .with_alignment(TextAlignment::Right),
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(20.0 + 20.0 * (i as f32 + 1.0)),
                    right: Val::Px(20.0),
                    ..default()
                },
                ..default()
            },
            InfoText,
        ));
    }
}

fn update_info_text(
    ncube_planes_of_rotation: Res<NCubePlanesOfRotation>,
    ncube_rotations: Res<NCubeRotations>,
    mut q_info_text: Query<&mut Text, With<InfoText>>,
) {
    q_info_text
        .iter_mut()
        .enumerate()
        .for_each(|(i, mut info_text)| {
            if let Some(plane) = ncube_planes_of_rotation.get(i) {
                if let Some(value) = ncube_rotations.get(&plane) {
                    let new_value = format!(
                        "q{}q{}: {:6.1}deg",
                        plane.0 + 1,
                        plane.1 + 1,
                        value.0.to_degrees() % 360.0
                    );
                    info_text.sections[0].style.color = if info_text.sections[0].value == new_value
                    {
                        Color::WHITE
                    } else {
                        Color::GREEN
                    };
                    info_text.sections[0].value = new_value;
                }
            }
        });
}
