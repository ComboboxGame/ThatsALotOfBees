use bevy::prelude::*;

use super::UiSize;

#[derive(Default)]
pub struct Target {
    pub right: f32,
    pub left: f32,
    pub top: f32,
    pub bottom: f32,
}

#[derive(Component)]
pub struct MovingUi {
    pub target: Target,
}

pub fn move_ui(mut mui: Query<(&MovingUi, &mut Style)>, ui: Res<UiSize>, time: Res<Time>) {
    for (mui, mut style) in mui.iter_mut() {
        let limit = ui.size as f32 * time.delta_seconds() * 950.0;
        style.right = transform(style.right, mui.target.right, limit);
        style.left = transform(style.left, mui.target.left, limit);
        style.top = transform(style.top, mui.target.top, limit);
        style.bottom = transform(style.bottom, mui.target.bottom, limit);
    }
}

fn transform(current: Val, target: f32, limit: f32) -> Val {
    if let Val::Px(current) = current {
        let new = (target - current).clamp(-limit, limit) + current;
        return Val::Px(new);
    }
    current
}
