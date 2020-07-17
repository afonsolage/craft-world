use crate::{
    components::Player,
    resources::{MainPlayer, PlayerSpriteAsset, TerrainSpriteAsset},
};
use amethyst::{
    core::{math::Vector3, Transform},
    prelude::*,
    renderer::{camera::Projection, Camera},
    SimpleState,
};

pub struct WorldState;

impl SimpleState for WorldState {
    fn on_start(&mut self, data: amethyst::StateData<'_, amethyst::GameData<'_, '_>>) {
        let world = data.world;
        init_camera(world);
        init_sprites(world);
        init_entities(world);
    }
}

fn init_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(0., 30., 1.);

    let mut camera = Camera::standard_3d(30., 30.);
    camera.set_projection(Projection::orthographic(0.0, 30., 0.0, 30., 0., 20.));

    world.create_entity().with(camera).with(transform).build();
}

fn init_sprites(world: &mut World) {
    TerrainSpriteAsset::init(world);
    PlayerSpriteAsset::init(world);
}

fn init_entities(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(20., 20., 0.1);
    transform.set_scale(Vector3::new(1. / 32., 1. / 32., 1. / 32.));

    let render = world.fetch::<PlayerSpriteAsset>().create_render(0);

    let main_player = world
        .create_entity()
        .with(transform)
        .with(Player::default())
        .with(render)
        .build();

    world.insert(MainPlayer::new(main_player));
}
