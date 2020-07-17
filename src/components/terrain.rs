use amethyst::ecs::prelude::*;

#[derive(Debug, Copy, Clone)]
pub enum TerrainType {
    Grass,
    GrassFlower,
    Sand,
    Dirt,
}

impl Default for TerrainType {
    fn default() -> Self {
        TerrainType::Grass
    }
}

impl Component for TerrainType {
    type Storage = DenseVecStorage<Self>;
}
