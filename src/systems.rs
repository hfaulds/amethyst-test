use amethyst::{
    core::{
        math::{Point3, Vector2},
        Transform,
        SystemDesc,
    },
    derive::SystemDesc,
    ecs::*,
    input::{InputHandler, StringBindings},
    renderer::{ActiveCamera,Camera},
    ui::UiText,
    window::ScreenDimensions,
    winit::MouseButton,
};
use crate::components::Character;
use crate::resources::{*};

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
        WriteStorage<'s, UiText>,
        WriteExpect<'s, Money>,
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
            mut text,
            mut money,
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
                finish_selection(selection, pos_world, &characters, &mut transforms, &entities, &mut shop, &mut reserve, &mut text, &mut money);
                self.selection = None;
            }
            return
        }

        if let Some(selection) = &self.selection {
            continue_selection(selection, pos_world, &characters, &mut transforms, &entities);
        } else {
            self.selection = start_selection(shop, pos_world, &characters, &money);
        }
    }
}

fn start_selection(
    shop: WriteExpect<Shop>,
    pos: Point3<f32>,
    characters: &ReadStorage<Character>,
    money: &WriteExpect<Money>,
    ) -> Option<SelectionStart> {
    if let Collision::Character(c, p, i) = shop.grid.collide(pos) {
        let character = characters.get(c).unwrap();
        if money.gold >= character.cost {
            return Some(SelectionStart { character: c, start_pos: p, start_index: i })
        }
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
    text: &mut WriteStorage<UiText>,
    money: &mut WriteExpect<Money>,
) {
    let (_, transform) = (characters, transforms).join()
        .get(selection.character, entities)
        .unwrap();

    if let Collision::Empty(p, i) = reserve.grid.collide(pos) {
        shop.grid.remove(selection.start_index);
        reserve.grid.add(i, selection.character);
        transform.set_translation_xyz(p.x, p.y, 0.1);

        let character = characters.get(selection.character).unwrap();
        money.gold -= character.cost;

        let mut text = text.get_mut(money.text).unwrap();
        text.text = money.gold.to_string();
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
