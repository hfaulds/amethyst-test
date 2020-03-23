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
pub struct PurchaseSystem {
    pub selection: Option<SelectionStart>,
}

impl<'s> System<'s> for PurchaseSystem {
    type SystemData = (
        Read<'s, InputHandler<StringBindings>>,
        Entities<'s>,
        Read<'s, ActiveCamera>,
        ReadExpect<'s, ScreenDimensions>,
        ReadStorage<'s, Camera>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Character>,
        WriteExpect<'s, Shop>,
        WriteExpect<'s, Reserve>,
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
            mut shop,
            mut reserve,
        ): Self::SystemData
    ) {
        let mouse_position = match input.mouse_position() {
            Some(p) => p,
            None => return,
        };
        let pos_world = match get_world_pos_for_cursor(mouse_position, &entities, active_camera, screen, cameras, &transforms) {
            Some(p) => p,
            None => return,
        };

        if !input.mouse_button_is_down(MouseButton::Left) {
            if let Some(selection) = &self.selection {
                finish_selection(selection, pos_world, &characters, &mut transforms, &entities, &mut shop, &mut reserve);
                self.selection = None;
            }
            return
        }

        if let Some(selection) = &self.selection {
            continue_selection(selection, pos_world, &characters, &mut transforms, &entities);
        } else {
            self.selection = start_selection(shop, pos_world);
        }
    }
}

fn start_selection(shop: WriteExpect<Shop>, pos: Point3<f32>) -> Option<SelectionStart> {
    if let Collision::Character(c, p, i) = shop.grid.collide(pos) {
        return Some(SelectionStart { character: c, start_pos: p, start_index: i })
    }
    None
}

fn continue_selection(
    selection: &SelectionStart,
    pos: Point3<f32>,
    characters: &ReadStorage<Character>,
    transforms: &mut WriteStorage<Transform>,
    entities: &Entities,
) {
    let (_, transform) = (characters, transforms).join()
        .get(selection.character, entities)
        .unwrap();
    transform.set_translation_xyz(pos.x, pos.y, 0.1);
}

fn finish_selection(
    selection: &SelectionStart,
    pos: Point3<f32>,
    characters: &ReadStorage<Character>,
    transforms: &mut WriteStorage<Transform>,
    entities: &Entities,
    shop: &mut WriteExpect<Shop>,
    reserve: &mut WriteExpect<Reserve>,
) {
    let (_, transform) = (characters, transforms).join()
        .get(selection.character, entities)
        .unwrap();

    if let Collision::Empty(p, i) = reserve.grid.collide(pos) {
        shop.grid.remove(i);
        reserve.grid.add(i, selection.character);
        transform.set_translation_xyz(p.x, p.y, 0.1);
    } else {
        transform.set_translation_xyz(selection.start_pos.x, selection.start_pos.y, 0.1);
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
   pub start_index: Point2<usize>,
}

enum Collision {
    None,
    Empty(Point2<f32>, Point2<usize>),
    Character(Entity, Point2<f32>, Point2<usize>),
}

pub struct Grid<const X: usize, const Y: usize> {
    pub x: f32,
    pub y: f32,
    pub entity_size: f32,
    pub entities: [[Option<Entity>;X];Y],
}

impl<const X: usize, const Y: usize> Grid<X,Y> {
    fn collide(&self, point: Point3<f32>) -> Collision {
        let x = (point.x + (self.entity_size/2.) - self.x) / self.entity_size;
        if x < 0. {
            return Collision::None
        }
        let x = x as usize;
        if x >= X {
            return Collision::None
        }
        let y = (point.y + (self.entity_size/2.) - self.y) / self.entity_size;
        if y < 0. {
            return Collision::None
        }
        let y = y as usize;
        if y >= Y {
            return Collision::None
        }
        let pos = Point2::new(
            self.x + (x as f32 * self.entity_size),
            self.y + (y as f32 * self.entity_size),
        );
        if let Some(entity) = self.entities[y as usize][x as usize] {
            return Collision::Character(entity, pos, Point2::new(x, y));
        }
        Collision::Empty(pos, Point2::new(x, y))
    }

    fn remove(&mut self, i: Point2<usize>) {
        self.entities[i.y][i.x] = None
    }

    fn add(&mut self, i: Point2<usize>, entity: Entity) {
        self.entities[i.y][i.x] = Some(entity)
    }
}
