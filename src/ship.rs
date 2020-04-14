// USE
use sfml::{graphics::*, system::*, window::*};
//
use std::collections::HashMap;
use std::f32::consts::PI;
//
use crate::baseobject::*;

#[allow(dead_code)]
pub struct Ship {
    base: BaseObject,

    deceleration: f32,
    top_speed: f32,
    rotation_speed: f32,
    is_turning_left: bool,
    is_turning_right: bool,
    is_thrusting: bool,
    is_shooting: bool,

    ship_points: [Vertex; 4],
    transform_ship_points: [Vertex; 4],
    thruster_points: [Vertex; 4],
    transform_thruster_points: [Vertex; 4],
}

impl Ship {
    pub fn new(x: f32, y: f32, angle: f32) -> Self {
        let ship_v = [
            Vertex::with_pos((10., 0.)),
            Vertex::with_pos((-10., -7.)),
            Vertex::with_pos((-10., 7.)),
            Vertex::with_pos((10., 0.)),
        ];

        let thruster_v = [
            Vertex::with_pos((-8., 0.)),
            Vertex::with_pos((0., -3.)),
            Vertex::with_pos((0., 3.)),
            Vertex::with_pos((-8., 0.)),
        ];

        let draw_sv = [Vertex::default(); 4];

        let draw_tv = [Vertex::default(); 4];

        Self {
            base: BaseObject {
                position: Vector2f::new(x, y),
                velocity: Vector2f::default(),
                acceleration: 100.,
                angle: angle,
                is_active: true,
            },
            deceleration: 20.,
            top_speed: 250.,
            rotation_speed: 5.,
            is_turning_left: false,
            is_turning_right: false,
            is_thrusting: false,
            is_shooting: false,
            ship_points: ship_v,
            transform_ship_points: draw_sv,
            thruster_points: thruster_v,
            transform_thruster_points: draw_tv,
        }
    }

    pub fn get_position_x(&self) -> f32 {
        self.base.position.x
    }

    pub fn get_position_y(&self) -> f32 {
        self.base.position.y
    }

    pub fn get_position(&self) -> Vector2f {
        self.base.position
    }

    pub fn get_angle(&self) -> f32 {
        self.base.angle
    }

    pub fn get_transform_points(&self) -> Vec<(f32, f32)> {
        // leave out last as its a copy of the first.
        let n = self.transform_ship_points.len();
        let mut result = vec![(0., 0.); 3];

        for (idx, p) in self.transform_ship_points[0..n - 1].iter().enumerate() {
            result[idx] = (p.position.x, p.position.y);
        }

        result
    }

    pub fn inputs(&mut self, key_map: &HashMap<&Key, bool>) {
        // left
        if let Some(x) = key_map.get(&Key::D) {
            self.is_turning_left = *x
        }
        // right
        if let Some(x) = key_map.get(&Key::A) {
            self.is_turning_right = *x;
        }
        // up
        if let Some(x) = key_map.get(&Key::W) {
            self.is_thrusting = *x;
        }
        if let Some(x) = key_map.get(&Key::Space) {
            self.is_shooting = *x;
        }
    }

    pub fn draw(&mut self, window: &mut RenderWindow) {
        // ship
        window.draw_primitives(
            &self.transform_ship_points,
            PrimitiveType::LineStrip,
            RenderStates::default(),
        );
        // thruster
        if self.is_thrusting {
            window.draw_primitives(
                &self.transform_thruster_points,
                PrimitiveType::LineStrip,
                RenderStates::default(),
            );
        }
    }

    fn update_verts(&mut self) {
        // rotation matrix
        // [ cos + -sin ]
        // [ sin +  cos ]
        // ship
        for (idx, vert) in self.ship_points.iter_mut().enumerate() {
            let x = vert.position.x;
            let y = vert.position.y;

            self.transform_ship_points[idx].position.x =
                (x * self.base.angle.cos()) - (y * self.base.angle.sin());
            self.transform_ship_points[idx].position.y =
                (x * self.base.angle.sin()) + (y * self.base.angle.cos());
            self.transform_ship_points[idx].position += self.base.position;
        }

        if self.is_thrusting {
            // thruster
            for (idx, vert) in self.thruster_points.iter_mut().enumerate() {
                let offset = -18.;
                let x = vert.position.x + offset;
                let y = vert.position.y;

                self.transform_thruster_points[idx].position.x =
                    (x * self.base.angle.cos()) - (y * self.base.angle.sin());
                self.transform_thruster_points[idx].position.y =
                    (x * self.base.angle.sin()) + (y * self.base.angle.cos());

                self.transform_thruster_points[idx].position += self.base.position;
            }
        }
    }

    fn screen_wrap(&mut self, width: f32, height: f32, padding: f32) {
        // screen wrap
        let screen_edge = 0.;

        if self.base.position.x > width as f32 + padding {
            self.base.position.x = screen_edge - padding;
        }
        if self.base.position.x < screen_edge - padding {
            self.base.position.x = width as f32 + padding;
        }
        if self.base.position.y > height as f32 + padding {
            self.base.position.y = screen_edge - padding;
        }
        if self.base.position.y < screen_edge - padding {
            self.base.position.y = height as f32 + padding;
        }
    }

    pub fn update(&mut self, delta: f32) {
        // angle
        if self.base.angle < 0. {
            self.base.angle += PI * 2.;
        }
        if self.base.angle > PI * 2. {
            self.base.angle -= PI * 2.;
        }

        // direction
        if self.is_turning_left {
            self.base.angle += self.rotation_speed * delta;
        }

        if self.is_turning_right {
            self.base.angle -= self.rotation_speed * delta;
        }

        if self.is_thrusting {
            self.base.velocity.x += self.base.angle.cos() * self.base.acceleration * delta;
            self.base.velocity.y += self.base.angle.sin() * self.base.acceleration * delta;
        }

        // TODO: shoot
        if self.is_shooting {
            // println!("bang!");
        }

        // slow down/top speed
        let length = (self.base.velocity.x * self.base.velocity.x
            + self.base.velocity.y * self.base.velocity.y)
            .sqrt();

        if length > 0. {
            self.base.velocity -= self.base.velocity / length * self.deceleration * delta;
        }

        if length > self.top_speed {
            self.base.velocity = (self.base.velocity / length) * self.top_speed
        }

        self.base.position += self.base.velocity * delta;
        self.screen_wrap(800., 600., 20.);
        self.update_verts();
    }
}
