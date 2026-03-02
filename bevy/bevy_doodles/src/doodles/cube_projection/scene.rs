use bevy::{camera::ScalingMode, prelude::*};

use super::components::ProjectedCube;
use crate::app_state::AppState;

const BACKGROUND_COLOR: (f32, f32, f32) = (0.85, 0.85, 0.85);
const CUBE_COLOR: (f32, f32, f32) = (0.3, 0.3, 0.3);

const CAMERA_DISTANCE: f32 = 10.0;
const CAMERA_VIEWPORT_HEIGHT: f32 = 6.0;

const BACK_BUTTON_FONT_SIZE: f32 = 10.0;
const BACK_BUTTON_WIDTH: f32 = 100.0;
const BACK_BUTTON_HEIGHT: f32 = 50.0;
const BACK_BUTTON_BG_COLOR: (f32, f32, f32) = (0.15, 0.15, 0.15);
const BACK_BUTTON_TEXT_COLOR: (f32, f32, f32) = (0.9, 0.9, 0.9);
const UI_PADDING: f32 = 20.0;

#[derive(Component)]
pub(super) struct BackButton;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut clear_color: ResMut<ClearColor>,
) {
    clear_color.0 = Color::srgb(BACKGROUND_COLOR.0, BACKGROUND_COLOR.1, BACKGROUND_COLOR.2);

    // Cube at origin
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb(CUBE_COLOR.0, CUBE_COLOR.1, CUBE_COLOR.2))),
        ProjectedCube,
        DespawnOnExit(AppState::CubeProjection),
    ));

    // Directional light
    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::default().looking_at(Vec3::new(-1.0, -2.0, -1.0), Vec3::Y),
        DespawnOnExit(AppState::CubeProjection),
    ));

    // Camera: Z points out of screen, slight X/Y tilt for 3D perspective
    commands.spawn((
        Camera3d::default(),
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: CAMERA_VIEWPORT_HEIGHT,
            },
            ..OrthographicProjection::default_3d()
        }),
        Transform::from_xyz(0.0, 0.0, CAMERA_DISTANCE)
            .looking_at(Vec3::ZERO, Vec3::Y),
        DespawnOnExit(AppState::CubeProjection),
    ));

    // Back button
    commands
        .spawn((
            Button,
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(UI_PADDING),
                top: Val::Px(UI_PADDING),
                width: Val::Px(BACK_BUTTON_WIDTH),
                height: Val::Px(BACK_BUTTON_HEIGHT),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(
                BACK_BUTTON_BG_COLOR.0,
                BACK_BUTTON_BG_COLOR.1,
                BACK_BUTTON_BG_COLOR.2,
            )),
            BackButton,
            DespawnOnExit(AppState::CubeProjection),
        ))
        .with_children(|button| {
            button.spawn((
                Text::new("← Menu (Esc)"),
                TextFont {
                    font_size: BACK_BUTTON_FONT_SIZE,
                    ..default()
                },
                TextColor(Color::srgb(
                    BACK_BUTTON_TEXT_COLOR.0,
                    BACK_BUTTON_TEXT_COLOR.1,
                    BACK_BUTTON_TEXT_COLOR.2,
                )),
            ));
        });
}

pub(super) fn handle_back_button(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<BackButton>)>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            next_state.set(AppState::Menu);
        }
    }
}
