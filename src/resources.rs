use amethyst::{
    core::math::{Point2, Point3},
    ecs::Entity,
};

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

pub enum Collision {
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
    pub fn collide(&self, point: Point3<f32>) -> Collision {
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

    pub fn remove(&mut self, i: Point2<usize>) {
        self.entities[i.y][i.x] = None
    }

    pub fn add(&mut self, i: Point2<usize>, entity: Entity) {
        self.entities[i.y][i.x] = Some(entity)
    }
}
