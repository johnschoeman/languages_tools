use bevy::prelude::*;

mod debug;
mod scene;
mod text_input;
mod ui;

use debug::{
    DebugMode, auto_screenshot, draw_debug_axes, screenshot_on_f12, setup_debug_ui,
    toggle_debug_mode, update_debug_text,
};
use scene::{
    AutoRotation, ColorAnimation, animate_cube_colors, apply_leaf_rotation_from_inputs,
    apply_light_position_from_inputs, apply_main_rotation_from_inputs, rotate_cube,
    setup as setup_scene, sync_main_rotation_to_inputs,
};
use text_input::{
    InputFocusState, handle_keyboard_input, handle_text_input_focus, update_cursor_blink,
    update_text_input_display,
};
use ui::{
    UiVisibility, handle_button_interaction, setup_ui, toggle_ui_visibility, update_ui_visibility,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<AutoRotation>()
        .init_resource::<ColorAnimation>()
        .init_resource::<ClearColor>()
        .init_resource::<DebugMode>()
        .init_resource::<InputFocusState>()
        .init_resource::<UiVisibility>()
        .add_systems(Startup, (setup_scene, setup_ui, setup_debug_ui))
        .add_systems(
            Update,
            (rotate_cube, handle_button_interaction, screenshot_on_f12),
        )
        .add_systems(
            Update,
            (toggle_debug_mode, draw_debug_axes, update_debug_text),
        )
        .add_systems(Update, (toggle_ui_visibility, update_ui_visibility))
        .add_systems(
            Update,
            (
                handle_text_input_focus,
                handle_keyboard_input,
                update_cursor_blink,
                update_text_input_display,
            ),
        )
        .add_systems(Update, apply_leaf_rotation_from_inputs)
        .add_systems(Update, sync_main_rotation_to_inputs)
        .add_systems(Update, apply_main_rotation_from_inputs)
        .add_systems(Update, apply_light_position_from_inputs)
        .add_systems(Update, animate_cube_colors)
        .add_systems(Update, auto_screenshot)
        .run();
}
