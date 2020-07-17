use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    prelude::WorldExt,
    renderer::{ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    shred::World,
};


fn load_sprite_sheet(world: &World, name: &str) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            format!("sprites/{}.png", name),
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();

    loader.load(
        format!("sprites/{}.ron", name),
        SpriteSheetFormat(texture_handle),
        (),
        &sheet_storage,
    )
}


//TODO: Define Macro implement sprite assets
macro_rules! impl_sprite_asset {
    ($tp:ident, $name:literal) => {
        #[derive(Clone)]
        pub struct $tp {
            handle: Handle<SpriteSheet>,
        }

        impl $tp {
            pub fn init(world: &mut World) {
                let asset = $tp {
                    handle: load_sprite_sheet(world, $name),
                };
                world.insert(asset);
            }
        
            pub fn create_render(&self, sprite_number: usize) -> SpriteRender {
                SpriteRender {
                    sprite_sheet: self.handle.clone(),
                    sprite_number: sprite_number,
                }
            }
        }
    }
}

impl_sprite_asset!(TerrainSpriteAsset, "terrain");
impl_sprite_asset!(PlayerSpriteAsset, "player");