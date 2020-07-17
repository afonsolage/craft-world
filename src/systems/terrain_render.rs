use crate::{components::TerrainType, resources::TerrainSpriteAsset};
use amethyst::{derive::SystemDesc, ecs::prelude::*, renderer::SpriteRender};

#[derive(SystemDesc)]
pub struct TerrainRenderSystem;

impl<'a> System<'a> for TerrainRenderSystem {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, TerrainSpriteAsset>,
        ReadStorage<'a, TerrainType>,
        WriteStorage<'a, SpriteRender>,
    );

    fn run(&mut self, (entities, spr_asset, ter_types, mut spr_renderers): Self::SystemData) {
        let mut new_renders = vec![];
        
        for (e, t, _) in (&entities, &ter_types, !&spr_renderers).join() {
            new_renders.push((e, *t));
        }
        
        for (e, t) in new_renders {
            spr_renderers
                .insert(e, spr_asset.create_render(t as usize))
                .unwrap();
        }
    }
}
