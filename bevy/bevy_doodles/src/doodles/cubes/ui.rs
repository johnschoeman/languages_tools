use crate::app_state::AppState;
use crate::shared::text_input::TextInput;
use bevy::prelude::*;

use super::components::*;

// UI constants
const UI_PADDING: f32 = 20.0;
const BUTTON_SPACING: f32 = 10.0;
const BUTTON_WIDTH: f32 = 100.0;
const BUTTON_HEIGHT: f32 = 50.0;
const BUTTON_FONT_SIZE: f32 = 10.0;
const BUTTON_BG_COLOR: (f32, f32, f32) = (0.15, 0.15, 0.15);
const BUTTON_TEXT_COLOR: (f32, f32, f32) = (0.9, 0.9, 0.9);

// Input panel constants
const PANEL_BG_COLOR: (f32, f32, f32) = (0.1, 0.1, 0.1);
const PANEL_PADDING: f32 = 15.0;
const PANEL_ROW_GAP: f32 = 8.0;
const MAIN_PANEL_BOTTOM_OFFSET: f32 = 360.0;

// Input field constants
const INPUT_WIDTH: f32 = 80.0;
const INPUT_HEIGHT: f32 = 30.0;
const INPUT_PADDING: f32 = 5.0;
const INPUT_BG_COLOR: (f32, f32, f32) = (0.15, 0.15, 0.15);
const INPUT_COLUMN_GAP: f32 = 8.0;

// Text constants
const PANEL_TITLE_FONT_SIZE: f32 = 16.0;
const SECTION_HEADER_FONT_SIZE: f32 = 14.0;
const INPUT_FONT_SIZE: f32 = 10.0;
const TITLE_TEXT_COLOR: (f32, f32, f32) = (0.9, 0.9, 0.9);
const SECTION_TEXT_COLOR: (f32, f32, f32) = (0.7, 0.7, 0.7);

// Increment button constants
const INCREMENT_BUTTON_SIZE: f32 = 30.0;
const INCREMENT_AMOUNT: f32 = 5.0;

// Initial configuration values
struct CubeConfig {
    main_rotation_x: f32,
    main_rotation_y: f32,
    main_rotation_z: f32,
    leaf_rotation_x: f32,
    leaf_rotation_y: f32,
    leaf_rotation_z: f32,
    leaf_position_x: f32,
    leaf_position_y: f32,
    leaf_position_z: f32,
}

const CUBE_CONFIG: CubeConfig = CubeConfig {
    main_rotation_x: 90.0,
    main_rotation_y: 0.0,
    main_rotation_z: -100.0,
    leaf_rotation_x: 0.0,
    leaf_rotation_y: 45.0,
    leaf_rotation_z: 45.0,
    leaf_position_x: 0.9,
    leaf_position_y: 0.0,
    leaf_position_z: -0.9,
};

// Light position constants (matches LIGHT_POSITION in scene.rs)
const LIGHT_POSITION_X: f32 = -4.0;
const LIGHT_POSITION_Y: f32 = 12.0;
const LIGHT_POSITION_Z: f32 = 5.0;

pub fn setup_ui(mut commands: Commands) {
    // Back button (top-left, above other controls)
    commands
        .spawn((
            Button,
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(UI_PADDING),
                top: Val::Px(UI_PADDING),
                width: Val::Px(BUTTON_WIDTH),
                height: Val::Px(BUTTON_HEIGHT),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(
                BUTTON_BG_COLOR.0,
                BUTTON_BG_COLOR.1,
                BUTTON_BG_COLOR.2,
            )),
            BackButton,
            DespawnOnExit(AppState::Cubes),
        ))
        .with_children(|button| {
            button.spawn((
                Text::new("← Menu (Esc)"),
                TextFont {
                    font_size: BUTTON_FONT_SIZE,
                    ..default()
                },
                TextColor(Color::srgb(
                    BUTTON_TEXT_COLOR.0,
                    BUTTON_TEXT_COLOR.1,
                    BUTTON_TEXT_COLOR.2,
                )),
            ));
        });

    // Root UI container
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                padding: UiRect::new(
                    Val::Px(UI_PADDING),
                    Val::Px(UI_PADDING),
                    Val::Px(UI_PADDING + BUTTON_HEIGHT + BUTTON_SPACING),
                    Val::Px(UI_PADDING),
                ),
                ..default()
            },
            ToggleableUi,
            DespawnOnExit(AppState::Cubes),
        ))
        .with_children(|parent| {
            // Left side button panel
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(BUTTON_SPACING),
                    ..default()
                })
                .with_children(|panel| {
                    spawn_button(panel, "⏯ Auto (Space)", RotationButton::ToggleAuto);
                    spawn_button(panel, "↺ Reset (R)", RotationButton::Reset);
                });
        });

    // Configuration panels
    spawn_main_rotation_panel(&mut commands);
    spawn_leaf_config_panel(&mut commands);
    spawn_light_position_panel(&mut commands);
}

fn spawn_button(parent: &mut ChildSpawnerCommands, text: &str, button_type: RotationButton) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(BUTTON_WIDTH),
                height: Val::Px(BUTTON_HEIGHT),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(
                BUTTON_BG_COLOR.0,
                BUTTON_BG_COLOR.1,
                BUTTON_BG_COLOR.2,
            )),
            button_type,
        ))
        .with_children(|button| {
            button.spawn((
                Text::new(text),
                TextFont {
                    font_size: BUTTON_FONT_SIZE,
                    ..default()
                },
                TextColor(Color::srgb(
                    BUTTON_TEXT_COLOR.0,
                    BUTTON_TEXT_COLOR.1,
                    BUTTON_TEXT_COLOR.2,
                )),
            ));
        });
}

pub fn handle_button_interaction(
    interaction_query: Query<(&Interaction, &RotationButton), Changed<Interaction>>,
    mut cube_query: Query<&mut Transform, With<RotatingCube>>,
    mut auto_rotation: ResMut<AutoRotation>,
) {
    for (interaction, button_type) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match button_type {
                RotationButton::ToggleAuto => {
                    auto_rotation.enabled = !auto_rotation.enabled;
                }
                RotationButton::Reset => {
                    for mut transform in &mut cube_query {
                        transform.rotation = Quat::IDENTITY;
                    }
                }
            }
        }
    }
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
            if *field == button.target {
                let current: f32 = input.value.parse().unwrap_or(0.0);
                input.value = format!("{:.0}", current + button.delta);
            }
        }
    }
}

pub fn handle_back_button(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<BackButton>)>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            next_state.set(AppState::Menu);
        }
    }
}

fn spawn_main_rotation_panel(commands: &mut Commands) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(UI_PADDING),
                bottom: Val::Px(MAIN_PANEL_BOTTOM_OFFSET),
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
            ToggleableUi,
            DespawnOnExit(AppState::Cubes),
        ))
        .with_children(|panel: &mut ChildSpawnerCommands| {
            panel.spawn((
                Text::new("Main Cube Rotation"),
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

            spawn_input_row(
                panel,
                "X:",
                &CUBE_CONFIG.main_rotation_x.to_string(),
                InputField::MainRotationX,
                true,
            );
            spawn_input_row(
                panel,
                "Y:",
                &CUBE_CONFIG.main_rotation_y.to_string(),
                InputField::MainRotationY,
                true,
            );
            spawn_input_row(
                panel,
                "Z:",
                &CUBE_CONFIG.main_rotation_z.to_string(),
                InputField::MainRotationZ,
                true,
            );
        });
}

fn spawn_leaf_config_panel(commands: &mut Commands) {
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
            ToggleableUi,
            DespawnOnExit(AppState::Cubes),
        ))
        .with_children(|panel: &mut ChildSpawnerCommands| {
            panel.spawn((
                Text::new("Leaf Cube Configuration"),
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

            panel.spawn((
                Text::new("Rotation"),
                TextFont {
                    font_size: SECTION_HEADER_FONT_SIZE,
                    ..default()
                },
                TextColor(Color::srgb(
                    SECTION_TEXT_COLOR.0,
                    SECTION_TEXT_COLOR.1,
                    SECTION_TEXT_COLOR.2,
                )),
            ));
            spawn_input_row(
                panel,
                "X:",
                &CUBE_CONFIG.leaf_rotation_x.to_string(),
                InputField::LeafRotationX,
                true,
            );
            spawn_input_row(
                panel,
                "Y:",
                &CUBE_CONFIG.leaf_rotation_y.to_string(),
                InputField::LeafRotationY,
                true,
            );
            spawn_input_row(
                panel,
                "Z:",
                &CUBE_CONFIG.leaf_rotation_z.to_string(),
                InputField::LeafRotationZ,
                true,
            );

            panel.spawn((
                Text::new("Position"),
                TextFont {
                    font_size: SECTION_HEADER_FONT_SIZE,
                    ..default()
                },
                TextColor(Color::srgb(
                    SECTION_TEXT_COLOR.0,
                    SECTION_TEXT_COLOR.1,
                    SECTION_TEXT_COLOR.2,
                )),
            ));
            spawn_input_row(
                panel,
                "X:",
                &CUBE_CONFIG.leaf_position_x.to_string(),
                InputField::LeafTranslationX,
                false,
            );
            spawn_input_row(
                panel,
                "Y:",
                &CUBE_CONFIG.leaf_position_y.to_string(),
                InputField::LeafTranslationY,
                false,
            );
            spawn_input_row(
                panel,
                "Z:",
                &CUBE_CONFIG.leaf_position_z.to_string(),
                InputField::LeafTranslationZ,
                false,
            );
        });
}

fn spawn_light_position_panel(commands: &mut Commands) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                right: Val::Px(UI_PADDING),
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
            ToggleableUi,
            DespawnOnExit(AppState::Cubes),
        ))
        .with_children(|panel: &mut ChildSpawnerCommands| {
            panel.spawn((
                Text::new("Light Position"),
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

            spawn_input_row(
                panel,
                "X:",
                &LIGHT_POSITION_X.to_string(),
                InputField::LightPositionX,
                false,
            );
            spawn_input_row(
                panel,
                "Y:",
                &LIGHT_POSITION_Y.to_string(),
                InputField::LightPositionY,
                false,
            );
            spawn_input_row(
                panel,
                "Z:",
                &LIGHT_POSITION_Z.to_string(),
                InputField::LightPositionZ,
                false,
            );
        });
}

fn spawn_input_row(
    parent: &mut ChildSpawnerCommands,
    label: &str,
    initial: &str,
    field_type: InputField,
    with_buttons: bool,
) {
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(INPUT_COLUMN_GAP),
            align_items: AlignItems::Center,
            ..default()
        })
        .with_children(|row| {
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

            if with_buttons {
                spawn_increment_button(row, "▼", field_type, -INCREMENT_AMOUNT);
            }

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

            if with_buttons {
                spawn_increment_button(row, "▲", field_type, INCREMENT_AMOUNT);
            }

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
    target: InputField,
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
            IncrementButton { target, delta },
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

pub fn toggle_ui_visibility(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut ui_visibility: ResMut<UiVisibility>,
) {
    if keyboard.just_pressed(KeyCode::KeyH) {
        ui_visibility.visible = !ui_visibility.visible;
    }
}

pub fn update_ui_visibility(
    ui_visibility: Res<UiVisibility>,
    mut ui_query: Query<&mut Node, With<ToggleableUi>>,
) {
    for mut node in &mut ui_query {
        node.display = if ui_visibility.visible {
            Display::Flex
        } else {
            Display::None
        };
    }
}
