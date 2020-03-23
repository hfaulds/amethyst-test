use amethyst::ecs::{Component, DenseVecStorage};

#[derive(Clone)]
pub struct Character {
    pub cost: u8,
}

impl Component for Character {
    type Storage = DenseVecStorage<Self>;
}

