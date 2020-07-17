use crate::{
    components::Player,
    resources::{AxisBinding, MainPlayer, MoveBindingTypes},
};
use amethyst::{
    core::{Time, Transform},
    ecs::prelude::*,
    input::InputHandler,
};

//TODO: Find a better way to do this
const MOVE_SPEED: f32 = 2.0;

#[derive(Default)]
pub struct PlayerMoveSystem;

impl<'a> System<'a> for PlayerMoveSystem {
    type SystemData = (
        Read<'a, MainPlayer>,
        Read<'a, InputHandler<MoveBindingTypes>>,
        Read<'a, Time>,
        WriteStorage<'a, Player>,
        WriteStorage<'a, Transform>,
    );

    fn run(&mut self, (main_player, input, time, mut players, mut transforms): Self::SystemData) {
        if let Some(entity) = main_player.entity {
            let player = match players.get_mut(entity) {
                None => return,
                Some(p) => p,
            };

            if let Some(transform) = transforms.get_mut(entity) {
                if let Some(x) = input.axis_value(&AxisBinding::Horizontal) {
                    transform.move_right(x * MOVE_SPEED * time.delta_seconds());
                    if x < 0.0 || x > 0.0 {
                        player.dir = if x > 0.0 { 3 } else { 1 };
                        player.dir_dirty = true;
                    }
                }

                if let Some(y) = input.axis_value(&AxisBinding::Vertical) {
                    transform.move_up(y * MOVE_SPEED * time.delta_seconds());
                    if y < 0.0 || y > 0.0 {
                        player.dir = if y > 0.0 { 2 } else { 0 };
                        player.dir_dirty = true;
                    }
                }
            }
        }
    }
}
