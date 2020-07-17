use crate::{
    components::Player,
    resources::{MainPlayer, PlayerSpriteAsset},
};
use amethyst::{ecs::prelude::*, renderer::SpriteRender};
use log::error;

#[derive(Default)]
pub struct PlayerRenderSystem;

impl<'a> System<'a> for PlayerRenderSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, SpriteRender>,
        ReadStorage<'a, Player>,
        ReadExpect<'a, PlayerSpriteAsset>,
    );

    fn run(&mut self, (entities, mut renders, players, player_sprite_asset): Self::SystemData) {
        for (e, p) in (&entities, &players).join() {
            if !p.dir_dirty {
                return;
            }

            if let Err(e) = renders.insert(e, player_sprite_asset.create_render(p.dir as usize)) {
                error!("Error inserting player sprite {}", e);
            }
        }
    }
}
