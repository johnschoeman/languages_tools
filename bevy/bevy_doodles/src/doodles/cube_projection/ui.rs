use crate::app_state::AppState;
use crate::shared::text_input::TextInput;
use bevy::prelude::*;

use super::components::*;

// Panel constants
const PANEL_BG_COLOR: (f32, f32, f32) = (0.1, 0.1, 0.1);
const PANEL_PADDING: f32 = 15.0;
const PANEL_ROW_GAP: f32 = 8.0;
const UI_PADDING: f32 = 20.0;

// Text constants
const PANEL_TITLE_FONT_SIZE: f32 = 16.0;
const INPUT_FONT_SIZE: f32 = 10.0;
const TITLE_TEXT_COLOR: (f32, f32, f32) = (0.9, 0.9, 0.9);
const SECTION_TEXT_COLOR: (f32, f32, f32) = (0.7, 0.7, 0.7);

// Input field constants
const INPUT_WIDTH: f32 = 80.0;
const INPUT_HEIGHT: f32 = 30.0;
const INPUT_PADDING: f32 = 5.0;
const INPUT_BG_COLOR: (f32, f32, f32) = (0.15, 0.15, 0.15);
const INPUT_COLUMN_GAP: f32 = 8.0;

// Increment button constants
const INCREMENT_BUTTON_SIZE: f32 = 30.0;
const INCREMENT_AMOUNT: f32 = 5.0;

pub fn setup_ui(mut commands: Commands) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(UI_PADDING),
                bottom: Val::Px(UI_PADDING),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(PANEL_ROW_GAP),
                padding: UiRect::all(Val::Px(PANEL_PADDING)),
                ..default()
            },
            BackgroundColor(Color::srgb(
                PANEL_BG_COLOR.0,
                PANEL_BG_COLOR.1,
                PANEL_BG_COLOR.2,
            )),
            DespawnOnExit(AppState::CubeProjection),
        ))
        .with_children(|panel: &mut ChildSpawnerCommands| {
            panel.spawn((
                Text::new("Rotation"),
                TextFont {
                    font_size: PANEL_TITLE_FONT_SIZE,
                    ..default()
                },
                TextColor(Color::srgb(
                    TITLE_TEXT_COLOR.0,
                    TITLE_TEXT_COLOR.1,
                    TITLE_TEXT_COLOR.2,
                )),
            ));

            spawn_input_row(panel, "X:", "0", InputField::RotationX, InputFieldAxis::X);
            spawn_input_row(panel, "Y:", "0", InputField::RotationY, InputFieldAxis::Y);
            spawn_input_row(panel, "Z:", "0", InputField::RotationZ, InputFieldAxis::Z);
        });
}

fn spawn_input_row(
    parent: &mut ChildSpawnerCommands,
    label: &str,
    initial: &str,
    field_type: InputField,
    axis: InputFieldAxis,
) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(INPUT_COLUMN_GAP),
            align_items: AlignItems::Center,
            ..default()
        })
        .with_children(|row| {
            // Label
            row.spawn((
                Text::new(label),
                TextFont {
                    font_size: INPUT_FONT_SIZE,
                    ..default()
                },
                TextColor(Color::srgb(
                    TITLE_TEXT_COLOR.0,
                    TITLE_TEXT_COLOR.1,
                    TITLE_TEXT_COLOR.2,
                )),
            ));

            // Decrement button
            spawn_increment_button(row, "▼", axis, -INCREMENT_AMOUNT);

            // Text input field
            row.spawn((
                Button,
                Node {
                    width: Val::Px(INPUT_WIDTH),
                    height: Val::Px(INPUT_HEIGHT),
                    padding: UiRect::all(Val::Px(INPUT_PADDING)),
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(Color::srgb(
                    INPUT_BG_COLOR.0,
                    INPUT_BG_COLOR.1,
                    INPUT_BG_COLOR.2,
                )),
                TextInput {
                    value: initial.to_string(),
                    is_focused: false,
                    cursor_visible: false,
                    cursor_timer: 0.0,
                },
                field_type,
            ))
            .with_children(|input| {
                input.spawn((
                    Text::new(initial),
                    TextFont {
                        font_size: INPUT_FONT_SIZE,
                        ..default()
                    },
                    TextColor(Color::srgb(
                        TITLE_TEXT_COLOR.0,
                        TITLE_TEXT_COLOR.1,
                        TITLE_TEXT_COLOR.2,
                    )),
                ));
            });

            // Increment button
            spawn_increment_button(row, "▲", axis, INCREMENT_AMOUNT);

            // Unit label
            row.spawn((
                Text::new("degrees"),
                TextFont {
                    font_size: INPUT_FONT_SIZE,
                    ..default()
                },
                TextColor(Color::srgb(
                    SECTION_TEXT_COLOR.0,
                    SECTION_TEXT_COLOR.1,
                    SECTION_TEXT_COLOR.2,
                )),
            ));
        });
}

fn spawn_increment_button(
    parent: &mut ChildSpawnerCommands,
    label: &str,
    axis: InputFieldAxis,
    delta: f32,
) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(INCREMENT_BUTTON_SIZE),
                height: Val::Px(INCREMENT_BUTTON_SIZE),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(
                INPUT_BG_COLOR.0,
                INPUT_BG_COLOR.1,
                INPUT_BG_COLOR.2,
            )),
            IncrementButton { axis, delta },
        ))
        .with_children(|button| {
            button.spawn((
                Text::new(label),
                TextFont {
                    font_size: INPUT_FONT_SIZE,
                    ..default()
                },
                TextColor(Color::srgb(
                    TITLE_TEXT_COLOR.0,
                    TITLE_TEXT_COLOR.1,
                    TITLE_TEXT_COLOR.2,
                )),
            ));
        });
}

pub fn handle_increment_button(
    button_query: Query<(&Interaction, &IncrementButton), Changed<Interaction>>,
    mut input_query: Query<(&InputField, &mut TextInput)>,
) {
    for (interaction, button) in &button_query {
        if *interaction != Interaction::Pressed {
            continue;
        }

        for (field, mut input) in &mut input_query {
            let matches = matches!(
                (field, button.axis),
                (InputField::RotationX, InputFieldAxis::X)
                    | (InputField::RotationY, InputFieldAxis::Y)
                    | (InputField::RotationZ, InputFieldAxis::Z)
            );

            if matches {
                let current: f32 = input.value.parse().unwrap_or(0.0);
                input.value = format!("{:.0}", current + button.delta);
            }
        }
    }
}

pub fn apply_rotation_from_inputs(
    input_query: Query<(&InputField, &TextInput)>,
    mut cube_query: Query<&mut Transform, With<ProjectedCube>>,
) {
    let mut rot_x = 0.0;
    let mut rot_y = 0.0;
    let mut rot_z = 0.0;

    for (field, input) in &input_query {
        let value = input.value.parse::<f32>().unwrap_or(0.0);
        match field {
            InputField::RotationX => rot_x = value,
            InputField::RotationY => rot_y = value,
            InputField::RotationZ => rot_z = value,
        }
    }

    for mut transform in &mut cube_query {
        transform.rotation = Quat::from_euler(
            EulerRot::XYZ,
            rot_x.to_radians(),
            rot_y.to_radians(),
            rot_z.to_radians(),
        );
    }
}
