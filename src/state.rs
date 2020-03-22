use amethyst::{
    core::transform::Transform,
    input::{get_key, is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::Camera,
    window::ScreenDimensions,
};
use log::info;

use crate::resources::Sprites;
use crate::systems::{*};

pub struct MyState;

impl SimpleState for MyState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let screen = (*world.read_resource::<ScreenDimensions>()).clone();

        init_camera(world, &screen);

        let sprites = Sprites::initialize(world);
        init_shop(world, &sprites, &screen);
        init_board(world, &sprites, &screen);
        init_reserve(world, &sprites, &screen);
    }

    fn handle_event(
        &mut self,
        mut _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            // Check if the window should be closed
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }

            // Listen to any key events
            if let Some(event) = get_key(&event) {
                info!("handling key event: {:?}", event);
            }

            // If you're looking for a more sophisticated event handling solution,
            // including key bindings and gamepad support, please have a look at
            // https://book.amethyst.rs/stable/pong-tutorial/pong-tutorial-03.html#capturing-user-input
        }

        // Keep going
        Trans::None
    }
}

fn init_camera(world: &mut World, screen: &ScreenDimensions) {
    // Center the camera in the middle of the screen, and let it cover
    // the entire screen
    let mut transform = Transform::default();
    transform.set_translation_xyz(screen.width() * 0.5, screen.height() * 0.5, 1.);

    world
        .create_entity()
        .with(Camera::standard_2d(screen.width(), screen.height()))
        .with(transform)
        .build();
}

fn init_shop(world: &mut World, sprites: &Sprites, screen: &ScreenDimensions) {
    let row = 8;
    let size = 48.;
    let mut shop = Shop{
        grid: Grid{
            x: (screen.width() * 0.5) + ((0. - (row as f32 * 0.5) + 0.5) * size),
            y: screen.height() - size,
            entity_size: size,
            entities: [[None; 8];1],
        },
    };
    for i in 0..row {
        let x = (screen.width() * 0.5) + ((i as f32 - (row as f32 * 0.5) + 0.5) * size);
        let y = screen.height() - size;
        let mut transform = Transform::default();
        transform.set_translation_xyz(x, y, 0.);
        world
            .create_entity()
            .with(sprites.shop_sprite_render())
            .with(transform)
            .build();
        let mut transform = Transform::default();
        transform.set_translation_xyz(x, y, 0.1);
        let character = world
            .create_entity()
            .with(sprites.character_sprite_render())
            .with(transform)
            .with(Character{cost: 1})
            .build();
        shop.grid.entities[0][i] = Some(character);
    }
    world.insert(shop)
}

fn init_board(world: &mut World, sprites: &Sprites, screen: &ScreenDimensions) {
    let grid = 8;
    let size = 48.;
    let board = Board{
        grid: Grid{
            x: (screen.width() * 0.5) + ((0. - (grid as f32 * 0.5) + 0.5) * size),
            y: (screen.height() * 0.5) + ((0. - (grid as f32 * 0.5) + 0.5) * size),
            entity_size: size,
            entities: [[None; 8];8],
        },
    };
    world.insert(board);
    for i in 0..(grid * grid) {
        let x = (screen.width() * 0.5) + (((i % grid) as f32 - (grid as f32 * 0.5) + 0.5) * size);
        let y = (screen.height() * 0.5) + (((i / grid) as f32 - (grid as f32 * 0.5) + 0.5) * size);
        let mut transform = Transform::default();
        transform.set_translation_xyz(x, y, 0.);
        world
            .create_entity()
            .with(sprites.grid_sprite_render())
            .with(transform)
            .build();
    }
}

fn init_reserve(world: &mut World, sprites: &Sprites, screen: &ScreenDimensions) {
    let row = 8;
    let size = 48.;
    let reserve = Reserve{
        grid: Grid{
            x: (screen.width() * 0.5) + ((0. - (row as f32 * 0.5) + 0.5) * size),
            y: size,
            entity_size: size,
            entities: [[None; 8];1],
        },
    };
    world.insert(reserve);
    for i in 0..row {
        let mut transform = Transform::default();
        let x = (screen.width() * 0.5) + ((i as f32 - (row as f32 * 0.5) + 0.5) * size);
        let y = size;
        transform.set_translation_xyz(x, y, 0.);
        world
            .create_entity()
            .with(transform)
            .with(sprites.reserve_sprite_render())
            .build();
    }
}
