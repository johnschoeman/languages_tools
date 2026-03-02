use bevy::prelude::*;

use crate::app_state::AppState;

mod scene;

pub struct CubeProjectionPlugin;

impl Plugin for CubeProjectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::CubeProjection), scene::setup)
            .add_systems(
                Update,
                scene::handle_back_button.run_if(in_state(AppState::CubeProjection)),
            );
    }
}
