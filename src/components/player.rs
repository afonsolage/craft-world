use amethyst::ecs::prelude::*;

#[derive(Default)]
pub struct Player {
    pub dir: u8,
    pub dir_dirty: bool,
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}