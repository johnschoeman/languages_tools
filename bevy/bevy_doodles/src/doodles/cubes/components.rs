use bevy::prelude::*;

#[derive(Component)]
pub struct RotatingCube;

#[derive(Component)]
pub struct LeafCube;

#[derive(Component)]
pub struct SceneLight;

#[derive(Component)]
pub struct GroundPlane;

#[derive(Resource)]
pub struct AutoRotation {
    pub enabled: bool,
}

impl Default for AutoRotation {
    fn default() -> Self {
        Self { enabled: false }
    }
}

#[derive(Resource)]
pub struct UiVisibility {
    pub visible: bool,
}

impl Default for UiVisibility {
    fn default() -> Self {
        Self { visible: true }
    }
}

#[derive(Component)]
pub struct ToggleableUi;

#[derive(Component)]
pub enum RotationButton {
    ToggleAuto,
    Reset,
}

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub enum InputField {
    MainRotationX,
    MainRotationY,
    MainRotationZ,
    LeafRotationX,
    LeafRotationY,
    LeafRotationZ,
    LeafTranslationX,
    LeafTranslationY,
    LeafTranslationZ,
    LightPositionX,
    LightPositionY,
    LightPositionZ,
}

#[derive(Component)]
pub struct IncrementButton {
    pub target: InputField,
    pub delta: f32,
}

#[derive(Component)]
pub struct BackButton;
