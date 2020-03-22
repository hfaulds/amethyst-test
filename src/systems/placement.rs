use amethyst::{
    core::{
        math::{Point3, Vector2},
        Transform,
        SystemDesc,
    },
    derive::SystemDesc,
    ecs::*,
    input::{InputHandler, StringBindings},
    winit::MouseButton,
    renderer::{ActiveCamera,Camera},
    window::ScreenDimensions,
};
use crate::resources::Sprites;

/// This system is responsible for placing characters.
#[derive(SystemDesc)]
pub struct PlacementSystem {
    pub selected_character: Option<Transform>,
}

impl<'s> System<'s> for PlacementSystem {
    type SystemData = (
        Read<'s, InputHandler<StringBindings>>,
        Entities<'s>,
        Read<'s, ActiveCamera>,
        ReadExpect<'s, ScreenDimensions>,
        ReadStorage<'s, Camera>,
        ReadStorage<'s, Transform>,
        Read<'s, LazyUpdate>,
        ReadExpect<'s, Sprites>,
        WriteStorage<'s, Character>,
        ReadExpect<'s, Shop>,
    );

    fn run(
        &mut self, (
            input,
            entities,
            active_camera,
            screen,
            cameras,
            transforms,
            updater,
            sprites,
            characters,
            shop,
        ): Self::SystemData
    ) {
        if !input.mouse_button_is_down(MouseButton::Left) {
            return
        }
        let mouse_position = match input.mouse_position() {
            Some(p) => p,
            None => return,
        };
        let pos_world = match get_world_pos_for_cursor(mouse_position, &entities, active_camera, screen, cameras, &transforms) {
            Some(p) => p,
            None => return,
        };

        if let Some(character) = shop.grid.collide(pos_world) {
            println!("yes");
/*
                let mut transform = Transform::default();
                transform.set_translation_xyz(bounding_box.left + (tile.size * 0.5), bounding_box.bottom + (tile.size * 0.5), 0.1);
                let character = entities.create();
                updater.insert(character, sprites.character_sprite_render());
                updater.insert(character, transform.clone());
                tile.occupied = true;
*/
        }
    }
}

fn get_world_pos_for_cursor(
    mouse_position: (f32, f32),
    entities: &Entities,
    active_camera: Read<ActiveCamera>,
    screen: ReadExpect<ScreenDimensions>,
    cameras: ReadStorage<Camera>,
    transforms: &ReadStorage<Transform>,
) -> Option<Point3<f32>> {
    let mut camera_join = (&cameras, transforms).join();
    if let Some((camera, camera_transform)) = active_camera
        .entity
            .and_then(|a| camera_join.get(a, &entities))
            .or_else(|| camera_join.next())
    {
        let screen_dimensions = Vector2::new(screen.width(), screen.height());
        let pos_screen = Point3::new(
            mouse_position.0,
            mouse_position.1,
            0.1,
        );
        let pos_world = camera.projection().screen_to_world_point(
            pos_screen,
            screen_dimensions,
            camera_transform,
        );
        return Some(pos_world)
    }
    None
}

pub struct Character {
    pub cost: u8,
}

impl Component for Character {
    type Storage = DenseVecStorage<Self>;
}

pub struct Shop {
    pub grid: Grid<8,1>,
}

pub struct Board {
    pub grid: Grid<8,8>,
}

pub struct Reserve {
    pub grid: Grid<8,1>,
}

pub struct Grid<const X: usize, const Y: usize> {
    pub x: f32,
    pub y: f32,
    pub entity_size: f32,
    pub entities: [[Option<Entity>;X];Y],
}

impl<const X: usize, const Y: usize> Grid<X,Y> {
    fn collide(&self, point: Point3<f32>) -> Option<Entity> {
        let x = ((point.x + (self.entity_size/2.) - self.x) / self.entity_size) as i8;
        if x < 0 {
            return None
        }
        let x = x as usize;
        if x >= X {
            return None
        }
        let y = ((point.y + (self.entity_size/2.) - self.y) / self.entity_size) as i8;
        if y < 0 {
            return None
        }
        let y = y as usize;
        if y >= Y {
            return None
        }
        return self.entities[y as usize][x as usize]
    }
}
