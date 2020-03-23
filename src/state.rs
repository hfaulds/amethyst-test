use amethyst::{
    core::transform::Transform,
    input::{is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::Camera,
    ui::{Anchor, UiText, UiTransform},
    window::ScreenDimensions,
};

use crate::resources::{*};
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

        init_money(world)
    }

    fn handle_event(
        &mut self,
        mut _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }
        }

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
        let character = Character{cost: 1};
        let character_entity = world
            .create_entity()
            .with(sprites.character_sprite_render())
            .with(transform)
            .with(character.clone())
            .build();
        shop.grid.entities[0][i] = Some(character_entity);

        let font = Font::square(world);
        let transform = UiTransform::new(
            "P1".to_string(), Anchor::BottomLeft, Anchor::TopLeft,
            x * 2., y * 2., 0.2, 48., 48., // TODO: Don't do this!
        );
        world.create_entity()
            .with(transform)
            .with(UiText::new(
                    font,
                    character.cost.to_string(),
                    [1., 1., 1., 1.],
                    24.,
            )).build();
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

fn init_money(world: &mut World) {
    let font = Font::square(world);
    let transform = UiTransform::new(
        "P1".to_string(), Anchor::TopRight, Anchor::TopRight,
        -48., -72., 0., 48., 48.,
    );
    world.create_entity()
        .with(transform)
        .with(UiText::new(
            font,
            "10".to_string(),
            [1., 1., 1., 1.],
            48.,
        )).build();
}
