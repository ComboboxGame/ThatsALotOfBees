use bevy::{prelude::*, render::mesh::shape::Quad, sprite::Mesh2dHandle};

#[derive(Component)]
pub struct HiveVisual;

#[derive(Component)]
pub struct HiveTopVisual;

pub fn spawn_hive_visual(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    asset_server: &mut AssetServer,
) {
    commands.spawn((
        materials.add(ColorMaterial::from(asset_server.load("images/Hive.png"))),
        Mesh2dHandle(meshes.add(Quad::new(Vec2::new(320.0, 320.0)).into())),
        TransformBundle::from_transform(Transform::from_xyz(0., 0., -10.)),
        VisibilityBundle::default(),
        HiveVisual,
    ));

    commands.spawn((
        materials.add(ColorMaterial::from(asset_server.load("images/HiveTop.png"))),
        Mesh2dHandle(meshes.add(Quad::new(Vec2::new(320.0, 320.0)).into())),
        TransformBundle::from_transform(Transform::from_xyz(0., 0., -10.)),
        VisibilityBundle::default(),
        HiveTopVisual,
    ));
}
