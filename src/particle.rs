// USE
use sfml::{graphics::*, system::*};
//
use crate::baseobject::BaseObject;
use crate::isactive::IsActive;

pub struct Particle {
    base: BaseObject,
    points: [Vertex; 2],
    transform_points: [Vertex; 2],
    life_time: f32,
    max_life_time: f32,
}

impl IsActive for Particle {
    fn is_active(&self) -> bool {
        self.base.is_active
    }

    fn kill(&mut self) {
        self.base.is_active = false;
    }
}

impl Particle {
    pub fn new() -> Self {
        let p_shape = [Vertex::with_pos((-3.0, 0.)), Vertex::with_pos((3.0, 0.))];

        Self {
            base: BaseObject{
                position: Vector2f::default(),
                velocity: Vector2f::default(),
                acceleration: 0., 
                angle: 0.,
                is_active: true,
            },
            points: p_shape,
            transform_points: [Vertex::default(); 2],
            life_time: 0.,
            max_life_time: 0.,
        }
    }

    pub fn init(&mut self, x: f32, y: f32, acc: f32, ang: f32, life: f32) {
        let dx = ang.cos() * acc;
        let dy = ang.sin() * acc;

        self.base.position.x = x;
        self.base.position.y = y;
        self.base.velocity.x = dx;
        self.base.velocity.y = dy;
        self.base.angle = ang;
        self.base.acceleration = acc;
        self.max_life_time = life;
    }

    pub fn draw(&mut self, window: &mut RenderWindow) {
        if self.base.is_active {
            window.draw_primitives(
                &self.transform_points,
                PrimitiveType::LineStrip,
                RenderStates::default(),
            );
        }
    }

    fn update_points(&mut self) {
        for (idx, p) in self.points.iter_mut().enumerate() {
            let pos = p.position;

            let new_x = (pos.x * self.base.angle.cos()) - (pos.y * self.base.angle.sin());
            let new_y = (pos.x * self.base.angle.sin()) + (pos.y * self.base.angle.cos());

            self.transform_points[idx].position.x = new_x;
            self.transform_points[idx].position.y = new_y;
            self.transform_points[idx].position += self.base.position;
        }
    }

    pub fn is_dead(&self) -> bool {
        self.base.is_active == false
    }

    pub fn update(&mut self, delta: f32) {
        if self.base.is_active {
            self.life_time += delta;

            if self.life_time >= self.max_life_time {
                self.kill();
            }

            self.base.position += self.base.velocity * delta;

            self.update_points();
        }
    }
}
