use amethyst::{
    core::{
        math::{Point2, Point3, Vector2},
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

/// This system is responsible for placing characters.
#[derive(SystemDesc)]
pub struct PlacementSystem {
    pub selection: Option<SelectionStart>,
}

impl<'s> System<'s> for PlacementSystem {
    type SystemData = (
        Read<'s, InputHandler<StringBindings>>,
        Entities<'s>,
        Read<'s, ActiveCamera>,
        ReadExpect<'s, ScreenDimensions>,
        ReadStorage<'s, Camera>,
        WriteStorage<'s, Transform>,
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
            mut transforms,
            characters,
            shop,
        ): Self::SystemData
    ) {
        if !input.mouse_button_is_down(MouseButton::Left) {
            if let Some(selection) = &self.selection {
                let (_, transform) = (&characters, &mut transforms).join()
                    .get(selection.character, &entities)
                    .unwrap();
                transform.set_translation_xyz(selection.start_pos.x, selection.start_pos.y, 0.1);
                self.selection = None;
            }
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

        if let Some(selection) = &self.selection {
            let (_, transform) = (&characters, &mut transforms).join()
                .get(selection.character, &entities)
                .unwrap();
            transform.set_translation_xyz(pos_world.x, pos_world.y, 0.1);
        } else {
            self.selection = shop.grid.select(pos_world);
        }
    }
}

fn get_world_pos_for_cursor(
    mouse_position: (f32, f32),
    entities: &Entities,
    active_camera: Read<ActiveCamera>,
    screen: ReadExpect<ScreenDimensions>,
    cameras: ReadStorage<Camera>,
    transforms: &WriteStorage<Transform>,
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

#[derive(Clone)]
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

pub struct SelectionStart {
   pub character: Entity,
   pub start_pos: Point2<f32>,
}

pub struct Grid<const X: usize, const Y: usize> {
    pub x: f32,
    pub y: f32,
    pub entity_size: f32,
    pub entities: [[Option<Entity>;X];Y],
}

impl<const X: usize, const Y: usize> Grid<X,Y> {
    fn select(&self, point: Point3<f32>) -> Option<SelectionStart> {
        let x = (point.x + (self.entity_size/2.) - self.x) / self.entity_size;
        if x < 0. {
            return None
        }
        let x = x as usize;
        if x >= X {
            return None
        }
        let y = (point.y + (self.entity_size/2.) - self.y) / self.entity_size;
        if y < 0. {
            return None
        }
        let y = y as usize;
        if y >= Y {
            return None
        }
        if let Some(entity) = self.entities[y as usize][x as usize] {
            return Some(SelectionStart{
                character: entity,
                start_pos: Point2::new(
                    self.x + (x as f32 * self.entity_size),
                    self.y + (y as f32 * self.entity_size),
                ),
            });
        }
        None
    }
}
