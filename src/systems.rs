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
        WriteExpect<'s, Board>,
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
            mut board,
        ): Self::SystemData
    ) {
        let pos_world = match get_world_pos_for_cursor(&input, &entities, active_camera, screen, cameras, &transforms) {
            Some(p) => p,
            None => return,
        };

        if input.mouse_button_is_down(MouseButton::Left) {
            match &self.selection {
                Some(selection) => {
                    let (_, transform) = (&characters, &mut transforms).join()
                        .get(selection.character, &entities)
                        .unwrap();
                    transform.set_translation_xyz(pos_world.x, pos_world.y, 0.1);
                },
                None => {
                    let collision = shop.grid.collide(pos_world)
                        .or( || board.grid.collide(pos_world))
                        .or( || reserve.grid.collide(pos_world));
                    if let Collision::Character(collision_source, character, start_pos, start_index) = collision {
                        match collision_source {
                            CollisionSource::Shop  => {
                                let c = characters.get(character).unwrap();
                                if money.gold >= c.cost {
                                    self.selection = Some(SelectionStart { collision_source, character, start_pos, start_index });
                                }
                            }
                            _ => {
                                self.selection = Some(SelectionStart { collision_source, character, start_pos, start_index });
                            }
                        }
                    }
                }
            }
            return
        }

        if let Some(selection) = &self.selection {
            let (_, transform) = (&characters, &mut transforms).join()
                .get(selection.character, &entities)
                .unwrap();

            let collision = shop.grid.collide(pos_world)
                .or( ||  board.grid.collide(pos_world))
                .or( || reserve.grid.collide(pos_world));
            let movement = match collision {
                Collision::Empty(cs, new_pos, i) => {
                    match (selection.collision_source, cs) {
                        (CollisionSource::Shop, CollisionSource::Board) => {
                            let character = characters.get(selection.character).unwrap();
                            money.gold -= character.cost;

                            let mut text = text.get_mut(money.text).unwrap();
                            text.text = money.gold.to_string();
                            Movement::Move(&mut shop.grid, &mut board.grid, i, new_pos)
                        },
                        (CollisionSource::Shop, CollisionSource::Reserve) => {
                            let character = characters.get(selection.character).unwrap();
                            money.gold -= character.cost;

                            let mut text = text.get_mut(money.text).unwrap();
                            text.text = money.gold.to_string();
                            Movement::Move(&mut shop.grid, &mut reserve.grid, i, new_pos)
                        },
                        (CollisionSource::Board, CollisionSource::Board) => {
                            Movement::InternalMove(&mut board.grid, i, new_pos)
                        },
                        (CollisionSource::Board, CollisionSource::Reserve) => {
                            Movement::Move(&mut board.grid, &mut reserve.grid, i, new_pos)
                        },
                        (CollisionSource::Reserve, CollisionSource::Reserve) => {
                            Movement::InternalMove(&mut reserve.grid, i, new_pos)
                        },
                        (CollisionSource::Reserve, CollisionSource::Board) => {
                            Movement::Move(&mut reserve.grid, &mut board.grid, i, new_pos)
                        },
                        (_, CollisionSource::Shop) => {
                            Movement::None
                        },
                    }
                },
                _ => Movement::None,
            };
            match movement {
                Movement::Move(a, b, i , pos) => {
                    a.remove(selection.start_index);
                    b.add(i, selection.character);
                    transform.set_translation_xyz(pos.x, pos.y, 0.1);
                },
                Movement::InternalMove(a, i , pos) => {
                    a.remove(selection.start_index);
                    a.add(i, selection.character);
                    transform.set_translation_xyz(pos.x, pos.y, 0.1);
                },
                _ => {
                    transform.set_translation_xyz(selection.start_pos.x, selection.start_pos.y, 0.1);
                },
            }
            self.selection = None;
        }
    }
}

fn get_world_pos_for_cursor(
    input: &Read<InputHandler<StringBindings>>,
    entities: &Entities,
    active_camera: Read<ActiveCamera>,
    screen: ReadExpect<ScreenDimensions>,
    cameras: ReadStorage<Camera>,
    transforms: &WriteStorage<Transform>,
) -> Option<Point3<f32>> {
    let mouse_position = input.mouse_position()?;
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

