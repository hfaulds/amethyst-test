use amethyst::{
    core::{
        math::{Point3, Vector2, Vector3},
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
pub struct PlacementSystem;

impl<'s> System<'s> for PlacementSystem {
    type SystemData = (
        Read<'s, InputHandler<StringBindings>>,
        Entities<'s>,
        Read<'s, ActiveCamera>,
        ReadExpect<'s, ScreenDimensions>,
        ReadStorage<'s, Camera>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, Tile>,
        Read<'s, LazyUpdate>,
        ReadExpect<'s, Sprites>,
    );

    fn run(
        &mut self, (
            input,
            entities,
            active_camera,
            screen,
            cameras,
            transforms,
            mut tiles,
            updater,
            sprites,
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

        for (tile, transform) in (&mut tiles, &transforms).join() {
            let bounding_box = tile.bounding_box(transform.translation());
            if bounding_box.collide(pos_world) {
                if tile.occupied {
                    println!("no");
                } else {
                    println!("yes");
                    let mut transform = Transform::default();
                    transform.set_translation_xyz(bounding_box.left + (tile.size * 0.5), bounding_box.bottom + (tile.size * 0.5), 0.1);
                    let character = entities.create();
                    updater.insert(character, sprites.character_sprite_render());
                    updater.insert(character, transform.clone());
                    tile.occupied = true;
                }
            }
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

pub struct Tile {
    pub size: f32,
    pub occupied: bool,
}

impl Tile {
    pub fn bounding_box(&self, pos: &Vector3<f32>) -> BoundingBox {
        let left = pos.x - (self.size * 0.5);
        let bottom = pos.y - (self.size * 0.5);
        let right = pos.x + (self.size * 0.5);
        let top = pos.y + (self.size * 0.5);
        return BoundingBox{left, bottom, right, top}
    }
}

impl Component for Tile {
    type Storage = DenseVecStorage<Self>;
}

pub struct Character {
}

impl Component for Character {
    type Storage = DenseVecStorage<Self>;
}

pub struct BoundingBox {
    pub left: f32,
    pub bottom: f32,
    pub right: f32,
    pub top: f32,
}

impl BoundingBox {
    fn collide(&self, point: Point3<f32>) -> bool {
        point.x >= self.left &&
            point.x <= self.right &&
            point.y >= self.bottom &&
            point.y <= self.top
    }
}
