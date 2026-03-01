use bevy::prelude::*;

use crate::app_state::AppState;
use crate::shared::debug::{DebugMode, DebugText};

mod components;
pub mod scene;
pub mod ui;

pub use components::*;

pub struct CubesDoodlePlugin;

impl Plugin for CubesDoodlePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(bevy::light::PointLightShadowMap { size: 4096 })
            .init_resource::<AutoRotation>()
            .init_resource::<UiVisibility>()
            .add_systems(
                OnEnter(AppState::Cubes),
                (scene::setup, ui::setup_ui, reset_cubes_resources),
            )
            .add_systems(
                Update,
                (
                    scene::rotate_cube,
                    ui::handle_button_interaction,
                    ui::handle_back_button,
                    ui::toggle_ui_visibility,
                    ui::update_ui_visibility,
                    scene::apply_leaf_rotation_from_inputs,
                    scene::sync_main_rotation_to_inputs,
                    scene::apply_main_rotation_from_inputs,
                    scene::apply_light_position_from_inputs,
                    update_cubes_debug_text
                        .after(crate::shared::debug::update_debug_text),
                )
                    .run_if(in_state(AppState::Cubes)),
            );
    }
}

fn reset_cubes_resources(
    mut auto_rotation: ResMut<AutoRotation>,
    mut ui_visibility: ResMut<UiVisibility>,
) {
    *auto_rotation = AutoRotation::default();
    *ui_visibility = UiVisibility::default();
}

fn update_cubes_debug_text(
    debug_mode: Res<DebugMode>,
    cube_query: Query<&Transform, With<RotatingCube>>,
    mut text_query: Query<&mut Text, With<DebugText>>,
) {
    if !debug_mode.enabled {
        return;
    }

    if let Ok(transform) = cube_query.single() {
        let (axis, angle): (Vec3, f32) = transform.rotation.to_axis_angle();
        let euler = transform.rotation.to_euler(EulerRot::XYZ);

        for mut text in &mut text_query {
            **text = format!(
                "Debug Mode (D)\n\
                \n\
                Axes: Red=X, Green=Y, Blue=Z\n\
                \n\
                Rotation (Euler XYZ):\n\
                X: {:.1}\u{00b0}\n\
                Y: {:.1}\u{00b0}\n\
                Z: {:.1}\u{00b0}\n\
                \n\
                Axis-Angle:\n\
                Axis: ({:.2}, {:.2}, {:.2})\n\
                Angle: {:.1}\u{00b0}",
                euler.0.to_degrees(),
                euler.1.to_degrees(),
                euler.2.to_degrees(),
                axis.x,
                axis.y,
                axis.z,
                angle.to_degrees()
            );
        }
    }
}
