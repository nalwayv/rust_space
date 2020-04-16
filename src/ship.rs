// USE
use sfml::{graphics::*, system::*, window::*};
//
use std::collections::HashMap;
use std::f32::consts::PI;
//
use crate::baseobject::*;
use crate::boxshape::*;

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
    flip_color: bool,
    ship_points: [Vertex; 4],
    transform_ship_points: [Vertex; 4],
    transform_points: Vec<Vector2f>,
    thruster_points: [Vertex; 4],
    transform_thruster_points: [Vertex; 4],
    // for debug
    is_debug: bool,
    debug_box : BoxShape,
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

        let tp = vec![Vector2f::default(); 4];

        let d_box = BoxShape::new(x, y, 70., 70.);

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
            flip_color: false,
            ship_points: ship_v,
            transform_ship_points: draw_sv,
            transform_points: tp,
            thruster_points: thruster_v,
            transform_thruster_points: draw_tv,
            is_debug: true,
            debug_box: d_box,
        }
    }

    pub fn get_box(&self) -> &BoxShape{
        &self.debug_box
    }
    
    pub fn get_position(&self) -> Vector2f {
        self.base.position
    }

    pub fn get_angle(&self) -> f32 {
        self.base.angle
    }

    pub fn toggle_color(&mut self, value: bool) {
        self.flip_color = value;
    }

    pub fn toggle_debug(&mut self) {
        self.debug_box.toggle_active();
    }

    pub fn toggle_debug_color(&mut self, value: bool) {
        self.debug_box.toggle_color(value);
    }

    /// get vec of the current transform points for this ship
    pub fn get_tp(&self) -> &Vec<Vector2f> {
        &self.transform_points
    }

    pub fn is_fireing(&self)->bool{
        self.is_shooting
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
        if self. base.is_active{
            if self.is_debug{
                self.debug_box.draw(window);
            }

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
    }

    fn screen_wrap(&mut self, width: f32, height: f32, padding: f32) {
        // screen wrap
        let screen_edge = 0.;

        if self.base.position.x > width  + padding {
            self.base.position.x = screen_edge - padding;
        }
        if self.base.position.x < screen_edge - padding {
            self.base.position.x = width  + padding;
        }
        if self.base.position.y > height + padding {
            self.base.position.y = screen_edge - padding;
        }
        if self.base.position.y < screen_edge - padding {
            self.base.position.y = height + padding;
        }
    }

    fn update_verts(&mut self) {
        // rotation matrix
        // [ cos + -sin ]
        // [ sin +  cos ]
        // ship
        for (idx, p) in self.ship_points.iter_mut().enumerate() {
            let x = p.position.x;
            let y = p.position.y;

            self.transform_ship_points[idx].position.x =
                (x * self.base.angle.cos()) - (y * self.base.angle.sin());
            self.transform_ship_points[idx].position.y =
                (x * self.base.angle.sin()) + (y * self.base.angle.cos());
            self.transform_ship_points[idx].position += self.base.position;

            if self.flip_color {
                self.transform_ship_points[idx].color = Color::RED;
            } else {
                self.transform_ship_points[idx].color = Color::WHITE;
            }

            self.transform_points[idx] = self.transform_ship_points[idx].position;
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

                if self.flip_color {
                    self.transform_thruster_points[idx].color = Color::RED;
                } else {
                    self.transform_thruster_points[idx].color = Color::WHITE;
                }
            }
        }
    }

    pub fn update(&mut self, delta: f32) {
        // angle
        if self.base.is_active{
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

            if self.is_debug{
                self.debug_box.set_position(self.base.position);
                self.debug_box.update(delta);
            }
        }
    }
}
