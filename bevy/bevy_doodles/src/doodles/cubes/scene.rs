use crate::app_state::AppState;
use crate::shared::text_input::TextInput;
use bevy::{camera::ScalingMode, prelude::*};

use super::components::*;

// Rotation constants
const KEYBOARD_ROTATION_SPEED: f32 = 2.0;
const AUTO_ROTATION_SPEED_Y: f32 = 1.0;
const AUTO_ROTATION_SPEED_X: f32 = 0.5;

// Cube constants
const CUBE_SIZE: f32 = 1.0;
const CUBE_Y_POSITION: f32 = 0.5;
const SECOND_CUBE_X_OFFSET: f32 = 0.8;
const SECOND_CUBE_ROTATION_DEGREES: f32 = 45.0;

// Initial rotation (matches CUBE_CONFIG in ui.rs)
const MAIN_CUBE_INITIAL_ROTATION: (f32, f32, f32) = (90.0, 0.0, -100.0);

// Color constants
const MAIN_CUBE_COLOR: (f32, f32, f32) = (0.3, 0.3, 0.3);
const LEAF_CUBE_COLOR: (f32, f32, f32) = (0.5, 0.5, 0.5);
const BACKGROUND_COLOR: (f32, f32, f32) = (0.85, 0.85, 0.85);
const GROUND_COLOR: (f32, f32, f32) = (1.0, 1.0, 1.0);

// Light constants
const LIGHT_POSITION: (f32, f32, f32) = (-4.0, 12.0, 5.0);

// Camera constants (isometric: equal distance along each axis)
const CAMERA_DISTANCE: f32 = 10.0;
const CAMERA_VIEWPORT_HEIGHT: f32 = 6.0;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut clear_color: ResMut<ClearColor>,
) {
    clear_color.0 = Color::srgb(BACKGROUND_COLOR.0, BACKGROUND_COLOR.1, BACKGROUND_COLOR.2);

    // Main/central cube - this is the parent that everything rotates around
    commands
        .spawn((
            Mesh3d(meshes.add(Cuboid::new(CUBE_SIZE, CUBE_SIZE, CUBE_SIZE))),
            MeshMaterial3d(materials.add(Color::srgb(
                MAIN_CUBE_COLOR.0,
                MAIN_CUBE_COLOR.1,
                MAIN_CUBE_COLOR.2,
            ))),
            Transform::from_xyz(0.0, CUBE_Y_POSITION, 0.0).with_rotation(Quat::from_euler(
                EulerRot::XYZ,
                MAIN_CUBE_INITIAL_ROTATION.0.to_radians(),
                MAIN_CUBE_INITIAL_ROTATION.1.to_radians(),
                MAIN_CUBE_INITIAL_ROTATION.2.to_radians(),
            )),
            RotatingCube,
            DespawnOnExit(AppState::Cubes),
        ))
        .with_children(|parent| {
            parent.spawn((
                Mesh3d(meshes.add(Cuboid::new(CUBE_SIZE, CUBE_SIZE, CUBE_SIZE))),
                MeshMaterial3d(materials.add(Color::srgb(
                    LEAF_CUBE_COLOR.0,
                    LEAF_CUBE_COLOR.1,
                    LEAF_CUBE_COLOR.2,
                ))),
                Transform::from_xyz(SECOND_CUBE_X_OFFSET, 0.0, 0.0).with_rotation(
                    Quat::from_euler(
                        EulerRot::XYZ,
                        SECOND_CUBE_ROTATION_DEGREES.to_radians(),
                        SECOND_CUBE_ROTATION_DEGREES.to_radians(),
                        0.0,
                    ),
                ),
                LeafCube,
            ));
        });

    // Light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(LIGHT_POSITION.0, LIGHT_POSITION.1, LIGHT_POSITION.2),
        SceneLight,
        DespawnOnExit(AppState::Cubes),
    ));

    // Ground plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(10.0)))),
        MeshMaterial3d(materials.add(Color::srgb(GROUND_COLOR.0, GROUND_COLOR.1, GROUND_COLOR.2))),
        Transform::from_xyz(0.0, -1.5, 0.0),
        GroundPlane,
        DespawnOnExit(AppState::Cubes),
    ));

    // Camera (isometric)
    let d = CAMERA_DISTANCE;
    commands.spawn((
        Camera3d::default(),
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: CAMERA_VIEWPORT_HEIGHT,
            },
            ..OrthographicProjection::default_3d()
        }),
        Transform::from_xyz(d, d, d)
            .looking_at(Vec3::new(0.0, CUBE_Y_POSITION, 0.0), Vec3::Y),
        DespawnOnExit(AppState::Cubes),
    ));
}

pub fn rotate_cube(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut auto_rotation: ResMut<AutoRotation>,
    mut query: Query<&mut Transform, With<RotatingCube>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        auto_rotation.enabled = !auto_rotation.enabled;
    }

    if keyboard.just_pressed(KeyCode::KeyR) {
        for mut transform in &mut query {
            transform.rotation = Quat::IDENTITY;
        }
        return;
    }

    let keyboard_delta = time.delta_secs() * KEYBOARD_ROTATION_SPEED;

    for mut transform in &mut query {
        if auto_rotation.enabled {
            transform.rotate_y(time.delta_secs() * AUTO_ROTATION_SPEED_Y);
            transform.rotate_x(time.delta_secs() * AUTO_ROTATION_SPEED_X);
        }

        if keyboard.pressed(KeyCode::KeyJ) {
            transform.rotate_local_x(keyboard_delta);
        }
        if keyboard.pressed(KeyCode::KeyU) {
            transform.rotate_local_x(-keyboard_delta);
        }
        if keyboard.pressed(KeyCode::KeyK) {
            transform.rotate_local_y(keyboard_delta);
        }
        if keyboard.pressed(KeyCode::KeyI) {
            transform.rotate_local_y(-keyboard_delta);
        }
        if keyboard.pressed(KeyCode::KeyL) {
            transform.rotate_local_z(keyboard_delta);
        }
        if keyboard.pressed(KeyCode::KeyO) {
            transform.rotate_local_z(-keyboard_delta);
        }
    }
}

pub fn apply_leaf_rotation_from_inputs(
    input_query: Query<(&InputField, &TextInput)>,
    parent_query: Query<&Children, With<RotatingCube>>,
    mut leaf_query: Query<&mut Transform, With<LeafCube>>,
) {
    let mut rot_x = 0.0;
    let mut rot_y = 0.0;
    let mut rot_z = 0.0;
    let mut trans_x = 0.0;
    let mut trans_y = 0.0;
    let mut trans_z = 0.0;

    for (field, input) in &input_query {
        let value = input.value.parse::<f32>().unwrap_or(0.0);
        match field {
            InputField::LeafRotationX => rot_x = value,
            InputField::LeafRotationY => rot_y = value,
            InputField::LeafRotationZ => rot_z = value,
            InputField::LeafTranslationX => trans_x = value,
            InputField::LeafTranslationY => trans_y = value,
            InputField::LeafTranslationZ => trans_z = value,
            _ => {}
        }
    }

    for children in &parent_query {
        for child in children.iter() {
            if let Ok(mut transform) = leaf_query.get_mut(child) {
                transform.rotation = Quat::from_euler(
                    EulerRot::XYZ,
                    rot_x.to_radians(),
                    rot_y.to_radians(),
                    rot_z.to_radians(),
                );
                transform.translation = Vec3::new(trans_x, trans_y, trans_z);
            }
        }
    }
}

pub fn sync_main_rotation_to_inputs(
    auto_rotation: Res<AutoRotation>,
    main_query: Query<&Transform, With<RotatingCube>>,
    mut input_query: Query<(&InputField, &mut TextInput)>,
) {
    if auto_rotation.enabled {
        return;
    }

    let Some(transform) = main_query.iter().next() else {
        return;
    };

    let (x, y, z) = transform.rotation.to_euler(EulerRot::XYZ);

    for (field, mut input) in &mut input_query {
        if input.is_focused {
            continue;
        }

        let new_value = match field {
            InputField::MainRotationX => format!("{:.0}", x.to_degrees()),
            InputField::MainRotationY => format!("{:.0}", y.to_degrees()),
            InputField::MainRotationZ => format!("{:.0}", z.to_degrees()),
            _ => continue,
        };

        input.value = new_value;
    }
}

pub fn apply_main_rotation_from_inputs(
    changed_query: Query<&InputField, Changed<TextInput>>,
    input_query: Query<(&InputField, &TextInput)>,
    mut main_query: Query<&mut Transform, With<RotatingCube>>,
) {
    let mut has_main_rotation_change = false;
    for field in &changed_query {
        match field {
            InputField::MainRotationX | InputField::MainRotationY | InputField::MainRotationZ => {
                has_main_rotation_change = true;
                break;
            }
            _ => {}
        }
    }

    if !has_main_rotation_change {
        return;
    }

    let mut rot_x = 0.0;
    let mut rot_y = 0.0;
    let mut rot_z = 0.0;

    for (field, input) in &input_query {
        let value = input.value.parse::<f32>().unwrap_or(0.0);
        match field {
            InputField::MainRotationX => rot_x = value,
            InputField::MainRotationY => rot_y = value,
            InputField::MainRotationZ => rot_z = value,
            _ => {}
        }
    }

    for mut transform in &mut main_query {
        transform.rotation = Quat::from_euler(
            EulerRot::XYZ,
            rot_x.to_radians(),
            rot_y.to_radians(),
            rot_z.to_radians(),
        );
    }
}

pub fn apply_light_position_from_inputs(
    input_query: Query<(&InputField, &TextInput)>,
    mut light_query: Query<&mut Transform, With<SceneLight>>,
) {
    let mut pos_x = LIGHT_POSITION.0;
    let mut pos_y = LIGHT_POSITION.1;
    let mut pos_z = LIGHT_POSITION.2;

    for (field, input) in &input_query {
        let value = input.value.parse::<f32>().unwrap_or(0.0);
        match field {
            InputField::LightPositionX => pos_x = value,
            InputField::LightPositionY => pos_y = value,
            InputField::LightPositionZ => pos_z = value,
            _ => {}
        }
    }

    for mut transform in &mut light_query {
        transform.translation = Vec3::new(pos_x, pos_y, pos_z);
    }
}
