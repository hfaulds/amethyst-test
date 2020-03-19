use amethyst::core::{Transform, SystemDesc};
use amethyst::derive::SystemDesc;
use amethyst::ecs::{Component, DenseVecStorage, Join, Read, ReadStorage, System, SystemData, World, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};
use amethyst::winit::MouseButton;

/// This system is responsible for placing characters.
#[derive(SystemDesc)]
pub struct PlacementSystem;

impl<'s> System<'s> for PlacementSystem {
    type SystemData = (
        ReadStorage<'s, Tile>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, Character>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (tiles, transforms, mut chars, input): Self::SystemData) {
        // Move every ball according to its speed, and the time passed.
        if input.mouse_button_is_down(MouseButton::Left) {
            let (x, y) = input.mouse_position().unwrap();
            for (tile, transform) in (&tiles, &transforms).join() {
                let left = transform.translation().x;
                let bottom = transform.translation().y;
                let right = transform.translation().x + tile.width;
                let top = transform.translation().y + tile.height;
                if point_in_rect(x / 2., y / 2., left, bottom, right, top) {
                    println!("hit")
                }
            }
        }
    }
}

pub struct Tile {
    pub width: f32,
    pub height: f32,
}

impl Component for Tile {
    type Storage = DenseVecStorage<Self>;
}

pub struct Character {
    x: u32,
    y: u32,
}

impl Component for Character {
    type Storage = DenseVecStorage<Self>;
}

fn point_in_rect(x: f32, y: f32, left: f32, bottom: f32, right: f32, top: f32) -> bool {
    x >= left && x <= right && y >= bottom && y <= top
}
