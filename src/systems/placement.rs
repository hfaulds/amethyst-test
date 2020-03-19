use amethyst::{
    core::{Transform, SystemDesc},
    derive::SystemDesc,
    ecs::*,
    input::{InputHandler, StringBindings},
    winit::MouseButton,
    window::ScreenDimensions,
};
use crate::resources::Sprites;

/// This system is responsible for placing characters.
#[derive(SystemDesc)]
pub struct PlacementSystem;

impl<'s> System<'s> for PlacementSystem {
    type SystemData = (
        ReadStorage<'s, Tile>,
        ReadStorage<'s, Transform>,
        Read<'s, InputHandler<StringBindings>>,
        Entities<'s>,
        Read<'s, LazyUpdate>,
        ReadExpect<'s, Sprites>,
        ReadExpect<'s, ScreenDimensions>,
    );

    fn run(&mut self, (tiles, transforms, input, entities, updater, sprites, screen): Self::SystemData) {
        // Move every ball according to its speed, and the time passed.
        if input.mouse_button_is_down(MouseButton::Left) {
            let (x, y) = input.mouse_position().unwrap();
            let x = x * 0.5;
            let y = (screen.height() - y) * 0.5;
            for (tile, transform) in (&tiles, &transforms).join() {
                let left = transform.translation().x - (tile.width * 0.5);
                let bottom = transform.translation().y - (tile.height * 0.5);
                let right = transform.translation().x + (tile.width * 0.5);
                let top = transform.translation().y + (tile.height * 0.5);
                if point_in_rect(x, y, left, bottom, right, top) {
                    let mut transform = Transform::default();
                    transform.set_translation_xyz(left + (tile.width * 0.5), bottom + (tile.height * 0.5), 0.1);
                    let character = entities.create();
                    updater.insert(character, sprites.sprite_render(1));
                    updater.insert(character, transform.clone());
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
}

impl Component for Character {
    type Storage = DenseVecStorage<Self>;
}

fn point_in_rect(x: f32, y: f32, left: f32, bottom: f32, right: f32, top: f32) -> bool {
    x >= left && x <= right && y >= bottom && y <= top
}
