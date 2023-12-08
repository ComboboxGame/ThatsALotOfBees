use bevy::prelude::*;

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

pub fn move_ui(mut ui: Query<(&MovingUi, &mut Style)>) {
    let (ui, mut style) = ui.single_mut();
    style.right = transform(style.right, ui.target.right);
    style.left = transform(style.left, ui.target.left);
    style.top = transform(style.top, ui.target.top);
    style.bottom = transform(style.bottom, ui.target.bottom);
}

fn transform(current: Val, target: f32) -> Val {
    if let Val::Px(current) = current {
        return Val::Px(Vec2::new(current, 0.).lerp(Vec2::new(target, 0.), 0.1).x);
    }
    current
}
