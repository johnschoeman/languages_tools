use bevy::prelude::*;

#[derive(Component)]
pub struct ProjectedCube;

#[derive(Component)]
pub enum InputField {
    RotationX,
    RotationY,
    RotationZ,
}

#[derive(Clone, Copy)]
pub enum InputFieldAxis {
    X,
    Y,
    Z,
}

#[derive(Component)]
pub struct IncrementButton {
    pub axis: InputFieldAxis,
    pub delta: f32,
}
