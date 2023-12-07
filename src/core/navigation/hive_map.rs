use bevy::prelude::*;

use crate::core::{HIVE_IMAGE_SIZE, HIVE_WORLD_SIZE};

const HIVE_MAP_SIZE: usize = 80;

const ALLOWED_COLORS: [(u8, u8, u8, u8); 3] =
    [(246, 180, 0, 255), (246, 188, 0, 255), (255, 255, 255, 0)];

#[derive(Resource, Default)]
pub struct HiveMap {
    pub ready: bool,
    pub data: Vec<u32>,
}

impl HiveMap {
    pub fn get_obstruction_xy(&self, x: f32, y: f32) -> f32 {
        self.get_obstruction(Vec2::new(x, y))
    }

    pub fn get_obstruction(&self, pos: Vec2) -> f32 {
        if !self.ready {
            
        }
        if pos.x <= -HIVE_WORLD_SIZE / 2.0 || pos.x >= HIVE_WORLD_SIZE / 2.0 {
            return 0.0;
        }
        if pos.y <= -HIVE_WORLD_SIZE / 2.0 || pos.y >= HIVE_WORLD_SIZE / 2.0 {
            return 0.0;
        }
        let x = ((pos.x / HIVE_WORLD_SIZE + 0.5) * HIVE_MAP_SIZE as f32) as usize;
        let y = ((-pos.y / HIVE_WORLD_SIZE + 0.5) * HIVE_MAP_SIZE as f32) as usize;

        self.data[x + y * HIVE_MAP_SIZE] as f32
            / ((HIVE_IMAGE_SIZE * HIVE_IMAGE_SIZE) / (HIVE_MAP_SIZE * HIVE_MAP_SIZE)) as f32
    }
}

pub fn build_hive_map_system(
    mut hive_map: ResMut<HiveMap>,
    asset_server: ResMut<AssetServer>,
    images: Res<Assets<Image>>,
    mut image_handle: Local<Handle<Image>>,
) {
    if hive_map.ready {
        return;
    }

    if *image_handle == Handle::default() {
        *image_handle = asset_server.load("images/Hive.png");
    }

    if let Some(image) = images.get(image_handle.clone()) {
        assert!(image.width() == HIVE_IMAGE_SIZE as u32);
        assert!(image.height() == HIVE_IMAGE_SIZE as u32);

        hive_map.ready = true;
        hive_map.data.resize(HIVE_MAP_SIZE * HIVE_MAP_SIZE, 0);

        let scale = HIVE_IMAGE_SIZE / HIVE_MAP_SIZE;
        for i in 0..HIVE_MAP_SIZE {
            for j in 0..HIVE_MAP_SIZE {
                for dx in 0..scale {
                    for dy in 0..scale {
                        let idx = (i * scale + dx + (j * scale + dy) * HIVE_IMAGE_SIZE) * 4;
                        let color = (
                            image.data[idx],
                            image.data[idx + 1],
                            image.data[idx + 2],
                            image.data[idx + 3],
                        );
                        if !ALLOWED_COLORS.contains(&color) && color.3 == 255 {
                            hive_map.data[i + j * HIVE_MAP_SIZE] += 1;
                        }
                    }
                }
            }
        }
    }
}
