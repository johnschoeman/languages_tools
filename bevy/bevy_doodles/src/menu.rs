use bevy::prelude::*;

use crate::app_state::AppState;

const MENU_BG_COLOR: (f32, f32, f32) = (0.12, 0.12, 0.12);
const TITLE_FONT_SIZE: f32 = 48.0;
const BUTTON_FONT_SIZE: f32 = 20.0;
const BUTTON_WIDTH: f32 = 200.0;
const BUTTON_HEIGHT: f32 = 60.0;
const BUTTON_BG_COLOR: (f32, f32, f32) = (0.2, 0.2, 0.2);
const BUTTON_TEXT_COLOR: (f32, f32, f32) = (0.9, 0.9, 0.9);
const TITLE_TEXT_COLOR: (f32, f32, f32) = (0.85, 0.85, 0.85);

pub struct MenuPlugin;

#[derive(Component)]
struct MenuButton(AppState);

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Menu), setup_menu)
            .add_systems(
                Update,
                handle_menu_interaction.run_if(in_state(AppState::Menu)),
            );
    }
}

fn setup_menu(mut commands: Commands, mut clear_color: ResMut<ClearColor>) {
    clear_color.0 = Color::srgb(MENU_BG_COLOR.0, MENU_BG_COLOR.1, MENU_BG_COLOR.2);

    // Camera
    commands.spawn((Camera2d, DespawnOnExit(AppState::Menu)));

    // Root UI container (centered)
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(40.0),
                ..default()
            },
            DespawnOnExit(AppState::Menu),
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("Bevy Doodles"),
                TextFont {
                    font_size: TITLE_FONT_SIZE,
                    ..default()
                },
                TextColor(Color::srgb(
                    TITLE_TEXT_COLOR.0,
                    TITLE_TEXT_COLOR.1,
                    TITLE_TEXT_COLOR.2,
                )),
            ));

            // Cubes button
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
                    MenuButton(AppState::Cubes),
                ))
                .with_children(|button| {
                    button.spawn((
                        Text::new("Cubes"),
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

            // Cube Projection button
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
                    MenuButton(AppState::CubeProjection),
                ))
                .with_children(|button| {
                    button.spawn((
                        Text::new("Cube Projection"),
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
        });
}

fn handle_menu_interaction(
    interaction_query: Query<(&Interaction, &MenuButton), Changed<Interaction>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for (interaction, menu_button) in &interaction_query {
        if *interaction == Interaction::Pressed {
            next_state.set(menu_button.0.clone());
        }
    }
}
