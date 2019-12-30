use::amethyst::ecs::{Component, DenseVecStorage};

pub struct Ball {
    pub velocity: [f32; 2],
    pub radius: f32,
    pub has_launched: bool,
}

impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Default)]
pub struct Paddle {
    pub velocity: f32,
    pub width: f32,
    pub height: f32,
}

impl Component for Paddle {
    type Storage = DenseVecStorage<Self>;
}

pub struct Block {
    pub width: f32,
    pub height: f32,
    pub max_hits: i32,
    pub cur_hits: i32,
    pub cur_damage_state: usize,
}

impl Component for Block {
    type Storage = DenseVecStorage<Self>;
}