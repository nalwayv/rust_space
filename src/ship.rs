// USE
use sfml::{graphics::*, system::*, window::*};
//
use std::collections::HashMap;
use std::f32::consts::PI;
//
use crate::baseobject::BaseObject;
use crate::boxarea::BoxArea;
use crate::globals::{v2_length, v2_unit, SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::isactive::IsActive;

#[allow(dead_code)]
pub struct Ship {
    base: BaseObject,

    friction: f32,
    top_speed: f32,
    rotation_speed: f32,
    is_turning_left: bool,
    is_turning_right: bool,
    is_thrusting: bool,
    is_shooting: bool,
    is_debug: bool,
    points: [Vertex; 4],
    transform_points: [Vertex; 4],
    tp: Vec<Vector2f>,
    thruster_points: [Vertex; 4],
    transform_thruster_points: [Vertex; 4],
    box_area: BoxArea,
}

impl IsActive for Ship {
    fn is_active(&self) -> bool {
        self.base.is_active
    }

    fn kill(&mut self) {
        self.base.is_active = false;
    }
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

        let ba = BoxArea::new(x, y, 70., 70.);

        Self {
            base: BaseObject {
                position: Vector2f::new(x, y),
                velocity: Vector2f::default(),
                acceleration: 100.,
                angle: angle,
                is_active: true,
            },
            friction: 20.,
            top_speed: 250.,
            rotation_speed: 5.,
            is_turning_left: false,
            is_turning_right: false,
            is_thrusting: false,
            is_shooting: false,
            is_debug: false,
            points: ship_v,
            transform_points: draw_sv,
            tp: tp,
            thruster_points: thruster_v,
            transform_thruster_points: draw_tv,
            box_area: ba,
        }
    }

    pub fn get_box_area(&self) -> &BoxArea {
        &self.box_area
    }

    pub fn get_position(&self) -> Vector2f {
        self.base.position
    }

    pub fn get_angle(&self) -> f32 {
        self.base.angle
    }

    pub fn alive(&mut self){
        self.base.is_active = true;
    }

    /// get vec of the current transform points for this ship
    pub fn get_tp(&self) -> &Vec<Vector2f> {
        &self.tp
    }


    pub fn is_fireing(&self) -> bool {
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
        if self.is_active() {
            if self.is_debug {
                self.box_area.draw(window);
            }

            // ship
            window.draw_primitives(
                &self.transform_points,
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

        if self.base.position.x > width + padding {
            self.base.position.x = screen_edge - padding;
        }
        if self.base.position.x < screen_edge - padding {
            self.base.position.x = width + padding;
        }
        if self.base.position.y > height + padding {
            self.base.position.y = screen_edge - padding;
        }
        if self.base.position.y < screen_edge - padding {
            self.base.position.y = height + padding;
        }
    }

    fn update_points(&mut self) {
        // rotation matrix
        // [ cos + -sin ]
        // [ sin +  cos ]
        // ship
        for (idx, p) in self.points.iter_mut().enumerate() {
            let pos = p.position;

            let new_x = (pos.x * self.base.angle.cos()) - (pos.y * self.base.angle.sin());
            let new_y = (pos.x * self.base.angle.sin()) + (pos.y * self.base.angle.cos());

            self.transform_points[idx].position.x = new_x;
            self.transform_points[idx].position.y = new_y;
            self.transform_points[idx].position += self.base.position;

            // for sat collision
            self.tp[idx] = self.transform_points[idx].position;
        }

        if self.is_thrusting {
            // thruster
            for (idx, vert) in self.thruster_points.iter_mut().enumerate() {
                let offset = -18.;
                let x = vert.position.x + offset;
                let y = vert.position.y;


                let new_x = (x * self.base.angle.cos()) - (y * self.base.angle.sin());
                let new_y =  (x * self.base.angle.sin()) + (y * self.base.angle.cos());

                self.transform_thruster_points[idx].position.x = new_x;
                self.transform_thruster_points[idx].position.y = new_y;
                self.transform_thruster_points[idx].position += self.base.position;

            }
        }
    }

    pub fn update(&mut self, delta: f32) {
        // angle
        if self.is_active() {
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
            let len = v2_length(self.base.velocity);

            if len > 0. {
                let unit = v2_unit(self.base.velocity);
                self.base.velocity -= unit * self.friction * delta;
            }

            if len > self.top_speed {
                let unit = v2_unit(self.base.velocity);
                self.base.velocity = unit * self.top_speed;
            }

            // p=v*t
            self.base.position += self.base.velocity * delta;

            // box collider
            self.box_area.set_position(self.get_position());
            self.box_area.update();

            self.screen_wrap(SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32, 20.);
            self.update_points();
        }
    }
}
