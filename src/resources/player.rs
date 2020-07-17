use amethyst::ecs::prelude::*;

#[derive(Default)]
pub struct MainPlayer {
    pub entity: Option<Entity>,
}

impl MainPlayer {
    pub fn new(entity: Entity) -> Self {
        MainPlayer {
            entity: Some(entity),
        }
    }
}
