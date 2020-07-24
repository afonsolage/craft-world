use simdnoise::NoiseBuilder;
use crate::components::TerrainType;

const OFFSET: i32 = i32::MAX / 2;

#[derive(Debug)]
pub struct TerrainData {
    data: Vec<f32>,
    width: usize,
    height: usize,
}

impl TerrainData {
    pub fn new(width: usize, height: usize) -> TerrainData {
        TerrainData {
            data: NoiseBuilder::gradient_2d(width, height).generate_scaled(0.0, 1.0),
            width,
            height,
        }
    }

    fn at(&self, x: usize, y: usize) -> f32 {
        self.data[x * self.width + y]
    }

    pub fn type_at(&self, x: i32, y: i32) -> TerrainType {
        let height = self.at((x + OFFSET) as usize % 200, (y + OFFSET) as usize % 200);

        match height {
            h if h < 0.1 => TerrainType::Grass,
            h if h < 0.2 => TerrainType::GrassFlower,
            h if h < 0.5 => TerrainType::Dirt,
            h if h < 0.8 => TerrainType::Sand,
            _ => TerrainType::Grass,
        }
    }
}
