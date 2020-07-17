use crate::{components::TerrainType, resources::MainPlayer};
use amethyst::{
    core::{math::Vector3, Transform},
    ecs::prelude::*,
};
use log::error;

pub struct TerrainMoveSystem {
    player_pos_x: i32,
    player_pos_y: i32,
    viewport_size: usize,
}

impl TerrainMoveSystem {
    pub fn new(viewport_size: usize) -> TerrainMoveSystem {
        TerrainMoveSystem {
            player_pos_x: i32::MAX,
            player_pos_y: i32::MAX,
            viewport_size,
        }
    }

    fn viewport_half_size(&self) -> usize {
        self.viewport_size / 2
    }

    fn is_on_viewport(&self, p_x: i32, p_y: i32) -> bool {
        p_x <= self.player_pos_x + self.viewport_half_size() as i32
            && p_x >= self.player_pos_x - self.viewport_half_size() as i32
            && p_y <= self.player_pos_y + self.viewport_half_size() as i32
            && p_y >= self.player_pos_y - self.viewport_half_size() as i32
    }

    fn gen_viewport_range(&self) -> Vec<(i32, i32)> {
        (self.player_pos_x - self.viewport_half_size() as i32
            ..=self.player_pos_x + self.viewport_half_size() as i32)
            .flat_map(|x| {
                (self.player_pos_y - self.viewport_half_size() as i32
                    ..=self.player_pos_y + self.viewport_half_size() as i32)
                    .map(move |y| (x, y))
            })
            .collect()
    }
}

impl<'a> System<'a> for TerrainMoveSystem {
    type SystemData = (
        WriteStorage<'a, TerrainType>,
        WriteStorage<'a, Transform>,
        Read<'a, MainPlayer>,
        Entities<'a>,
    );

    fn run(&mut self, (mut types, mut transforms, main_player, entities): Self::SystemData) {
        if let Some(entity) = main_player.entity {
            if let Some(player_transform) = transforms.get(entity) {
                let p_x = player_transform.translation().x as i32;
                let p_y = player_transform.translation().y as i32;

                if p_x == self.player_pos_x && p_y == self.player_pos_y {
                    //Nothing to do
                    return;
                }

                self.player_pos_x = p_x;
                self.player_pos_y = p_y;

                let existing_tiles = (&types, &entities, &transforms)
                    .join()
                    .filter_map(|(_, e, t)| {
                        if self.is_on_viewport(t.translation().x as i32, t.translation().y as i32) {
                            Some((t.translation().x as i32, t.translation().y as i32))
                        } else {
                            if let Err(e) = entities.delete(e) {
                                error!("Failed to remove entity. Error: {}", e);
                            }
                            None
                        }
                    })
                    .collect::<Vec<_>>();

                self.gen_viewport_range()
                    .into_iter()
                    .filter(|(x, y)| !existing_tiles.contains(&(*x, *y)))
                    .for_each(|(x, y)| {
                        let mut transform = Transform::default();
                        transform.set_translation_xyz(x as f32, y as f32, 0.0);
                        transform.set_scale(Vector3::new(1. / 32., 1. / 32., 1. / 32.));

                        entities
                            .build_entity()
                            .with(transform, &mut transforms)
                            .with(TerrainType::Dirt, &mut types)
                            .build();
                    });
            }
        }
    }
}
