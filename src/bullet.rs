// USE
use sfml::{graphics::*, system::*};
//
use crate::baseobject::*;

/// bullet
pub struct Bullet {
    base: BaseObject,
    life_timer: f32,
    max_life_time: f32,
    bullet_points: [Vertex; 2],
    transform_bullet_points: [Vertex; 2],
}

impl Bullet {
    pub fn new() -> Self {
        let bullet_v = [Vertex::with_pos((-5.0, 0.0)), Vertex::with_pos((5.0, 0.0))];
        let draw_bv = [Vertex::default(); 2];

        Self {
            base: BaseObject {
                position: Vector2f::default(),
                velocity: Vector2f::default(),
                acceleration: 350.0,
                is_active: false,
                angle: 0.0,
            },
            life_timer: 0.0,
            max_life_time: 1.0,
            bullet_points: bullet_v,
            transform_bullet_points: draw_bv,
        }
    }

    pub fn init(&mut self, position: (f32, f32), ang: f32) {
        self.base.angle = ang;
        self.base.position.x = position.0;
        self.base.position.y = position.1;

        self.base.velocity.x = self.base.angle.cos() * self.base.acceleration;
        self.base.velocity.y = self.base.angle.sin() * self.base.acceleration;

        self.base.is_active = true;
    }

    pub fn is_active(&self)->bool{
        self.base.is_active
    }

    pub fn draw(&mut self, window: &mut RenderWindow) {
        if self.base.is_active {
            window.draw_primitives(
                &self.transform_bullet_points,
                PrimitiveType::LineStrip,
                RenderStates::default(),
            );
        }
    }

    fn update_points(&mut self) {
        for (idx, p) in self.bullet_points.iter_mut().enumerate() {
            let x = p.position.x;
            let y = p.position.y;

            let new_x = (x * self.base.angle.cos()) - (y * self.base.angle.sin());
            let new_y = (x * self.base.angle.sin()) + (y * self.base.angle.cos());

            self.transform_bullet_points[idx].position.x = new_x;
            self.transform_bullet_points[idx].position.y = new_y;
            self.transform_bullet_points[idx].position += self.base.position;
        }
    }

    pub fn update(&mut self, delta: f32) {
        if self.base.is_active {
            // self.shape.move_(self.base.velocity * delta);
            self.base.position += self.base.velocity * delta;

            self.update_points();

            self.life_timer += delta;
            if self.life_timer > self.max_life_time {
                self.base.is_active = false;
            }
        }
    }
}
