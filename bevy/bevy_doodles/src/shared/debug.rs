use bevy::{
    app::AppExit,
    prelude::*,
    render::view::screenshot::{Screenshot, save_to_disk},
};

const AXIS_LENGTH: f32 = 2.0;
const UI_PADDING: f32 = 20.0;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DebugMode>()
            .add_systems(Startup, setup_debug_ui)
            .add_systems(
                Update,
                (
                    toggle_debug_mode,
                    draw_debug_axes,
                    update_debug_text,
                    screenshot_on_f12,
                    auto_screenshot,
                ),
            );
    }
}

#[derive(Resource)]
pub struct DebugMode {
    pub enabled: bool,
}

impl Default for DebugMode {
    fn default() -> Self {
        let enabled = std::env::var("AUTO_DEBUG").is_ok();
        Self { enabled }
    }
}

#[derive(Component)]
pub struct DebugText;

fn setup_debug_ui(mut commands: Commands) {
    commands
        .spawn(Node {
            position_type: PositionType::Absolute,
            right: Val::Px(UI_PADDING),
            bottom: Val::Px(UI_PADDING),
            padding: UiRect::all(Val::Px(10.0)),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                Text::new("Debug: Press D"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.8, 0.8)),
                DebugText,
                Visibility::Hidden,
            ));
        });
}

fn toggle_debug_mode(keyboard: Res<ButtonInput<KeyCode>>, mut debug_mode: ResMut<DebugMode>) {
    if keyboard.just_pressed(KeyCode::KeyD) {
        debug_mode.enabled = !debug_mode.enabled;
        info!(
            "Debug mode: {}",
            if debug_mode.enabled { "ON" } else { "OFF" }
        );
    }
}

fn draw_debug_axes(mut gizmos: Gizmos, debug_mode: Res<DebugMode>) {
    if !debug_mode.enabled {
        return;
    }
    gizmos.axes(Transform::IDENTITY, AXIS_LENGTH);
}

pub fn update_debug_text(
    debug_mode: Res<DebugMode>,
    mut text_query: Query<(&mut Text, &mut Visibility), With<DebugText>>,
) {
    for (mut text, mut visibility) in &mut text_query {
        if debug_mode.enabled {
            *visibility = Visibility::Visible;
            **text = "Debug Mode (D)\n\nAxes: Red=X, Green=Y, Blue=Z".to_string();
        } else {
            *visibility = Visibility::Hidden;
        }
    }
}

fn screenshot_on_f12(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut counter: Local<u32>,
) {
    if keyboard.just_pressed(KeyCode::F12) {
        if let Err(e) = std::fs::create_dir_all("./tmp") {
            error!("Failed to create tmp directory: {}", e);
            return;
        }

        let path = format!("./tmp/bevy_screenshot_{}.png", *counter);
        *counter += 1;
        info!("Taking screenshot: {}", path);
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk(path));
    }
}

fn auto_screenshot(
    mut commands: Commands,
    time: Res<Time>,
    mut screenshot_taken: Local<bool>,
    mut exit: MessageWriter<AppExit>,
) {
    if std::env::var("AUTO_SCREENSHOT").is_err() {
        return;
    }

    if !*screenshot_taken && time.elapsed_secs() > 1.0 {
        *screenshot_taken = true;

        if let Err(e) = std::fs::create_dir_all("./tmp") {
            error!("Failed to create tmp directory: {}", e);
            exit.write(AppExit::Error(std::num::NonZero::new(1).unwrap()));
            return;
        }

        let path = "./tmp/bevy_screenshot_auto.png";
        info!("Auto-screenshot mode: Taking screenshot to {}", path);

        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk(path.to_string()));
    }

    if *screenshot_taken && time.elapsed_secs() > 1.5 {
        info!("Exiting after auto-screenshot");
        exit.write(AppExit::Success);
    }
}
