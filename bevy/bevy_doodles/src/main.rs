use bevy::prelude::*;

mod app_state;
mod doodles;
mod menu;
mod shared;

use app_state::AppState;
use shared::text_input::InputFocusState;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        .init_resource::<ClearColor>()
        .add_plugins((
            shared::text_input::TextInputPlugin,
            shared::debug::DebugPlugin,
            menu::MenuPlugin,
            doodles::cubes::CubesDoodlePlugin,
            doodles::cube_projection::CubeProjectionPlugin,
        ))
        .add_systems(Startup, auto_navigate_for_screenshot)
        .add_systems(
            Update,
            navigate_back.before(shared::text_input::handle_keyboard_input),
        )
        .run();
}

fn navigate_back(
    keyboard: Res<ButtonInput<KeyCode>>,
    focus_state: Res<InputFocusState>,
    current_state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape)
        && focus_state.focused_entity.is_none()
        && *current_state.get() != AppState::Menu
    {
        next_state.set(AppState::Menu);
    }
}

fn auto_navigate_for_screenshot(mut next_state: ResMut<NextState<AppState>>) {
    if std::env::var("AUTO_SCREENSHOT").is_ok() {
        next_state.set(AppState::Cubes);
    }
}
