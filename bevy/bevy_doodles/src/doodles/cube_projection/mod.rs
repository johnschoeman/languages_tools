use bevy::prelude::*;

use crate::app_state::AppState;

mod components;
mod scene;
mod ui;

pub struct CubeProjectionPlugin;

impl Plugin for CubeProjectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(AppState::CubeProjection),
            (scene::setup, ui::setup_ui),
        )
        .add_systems(
            Update,
            (
                scene::handle_back_button,
                ui::handle_increment_button,
                ui::apply_rotation_from_inputs,
            )
                .run_if(in_state(AppState::CubeProjection)),
        );
    }
}
