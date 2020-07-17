use amethyst::{
    core::transform::TransformBundle,
    input::InputBundle,
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};
use resources::MoveBindingTypes;
use states::WorldState;

mod components;
mod resources;
mod states;
mod systems;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let assets_dir = app_root.join("assets");
    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");

    let input_config = config_dir.join("bindings.ron");
    let input_bundle =
        InputBundle::<MoveBindingTypes>::new().with_bindings_from_file(input_config)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with(systems::TerrainRenderSystem, "terrain_system", &[])
        .with(systems::PlayerRenderSystem, "player_system", &[])
        .with(
            systems::PlayerMoveSystem,
            "player_move_system",
            &["input_system"],
        )
        .with(
            systems::TerrainMoveSystem::new(10),
            "terrain_move_system",
            &[],
        );

    let mut game = Application::new(assets_dir, WorldState, game_data)?;
    game.run();

    Ok(())
}
