use crate::baseobject::BaseObject;
use crate::boxarea::BoxArea;
use crate::globals::{random_number, SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::isactive::IsActive;
use sfml::{graphics::*, system::*};
use std::f32::consts::PI;

pub struct Ufo {
    base: BaseObject,
    shoot_time: f32,
    max_shoot_time: f32,
    turn_time: f32,
    max_turn_time: f32,
    is_shooting: bool,
    points: [Vertex; 7],
    transform_points: [Vertex; 7],
    tp: Vec<Vector2f>,
    box_area: BoxArea,
    is_debug: bool,
}

impl IsActive for Ufo {
    fn is_active(&self) -> bool {
        self.base.is_active
    }

    fn kill(&mut self) {
        self.base.is_active = false;
    }
}

impl Ufo {
    pub fn new(x: f32, y: f32, acc: f32) -> Self {
        let p = [
            Vertex::with_pos((5., -5.)),
            Vertex::with_pos((16., 0.)),
            Vertex::with_pos((5., 5.)),
            Vertex::with_pos((-5., 5.)),
            Vertex::with_pos((-16., 0.)),
            Vertex::with_pos((-5., -5.)),
            Vertex::with_pos((5., -5.)),
        ];

        let ang: f32 = 0.0;
        let dx = ang.cos() * acc;
        let dy = ang.sin() * acc;

        let ba = BoxArea::new(x, y, 70., 70.);

        Self {
            base: BaseObject {
                position: Vector2f::new(x, y),
                velocity: Vector2f::new(dx, dy),
                acceleration: acc,
                angle: ang,
                is_active: true,
            },
            points: p,
            shoot_time: 0.0,
            max_shoot_time: 1.5,
            turn_time: 0.0,
            max_turn_time: 2.5,
            is_shooting: false,
            transform_points: [Vertex::default(); 7],
            tp: vec![Vector2f::default(); 7],
            box_area: ba,
            is_debug: false,
        }
    }

    pub fn get_position(&self) -> Vector2f {
        self.base.position
    }

    /// get vec of the current transform points for this ship
    pub fn get_tp(&self) -> &Vec<Vector2f> {
        &self.tp
    }

    pub fn get_box_area(&self) -> &BoxArea {
        &self.box_area
    }
    pub fn is_shooting(&self) -> bool {
        self.is_shooting
    }

    pub fn draw(&mut self, window: &mut RenderWindow) {
        if self.is_active() {
            if self.is_debug {
                self.box_area.draw(window);
            }

            window.draw_primitives(
                &self.transform_points,
                PrimitiveType::LineStrip,
                RenderStates::default(),
            );
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
        for (idx, p) in self.points.iter_mut().enumerate() {
            let x = p.position.x;
            let y = p.position.y;

            let new_x = (x * self.base.angle.cos()) - (y * self.base.angle.sin());
            let new_y = (x * self.base.angle.sin()) + (y * self.base.angle.cos());

            self.transform_points[idx].position.x = new_x;
            self.transform_points[idx].position.y = new_y;

            self.transform_points[idx].color = Color::RED;

            self.transform_points[idx].position += self.base.position;

            self.tp[idx] = self.transform_points[idx].position;
        }
    }

    pub fn update(&mut self, delta: f32) {
        if self.is_active() {
            // try and kill player!
            self.shoot_time += delta;
            if self.shoot_time >= self.max_shoot_time {
                self.is_shooting = true;
                self.shoot_time = 0.;
            } else {
                self.is_shooting = false;
            }

            // random new direction
            self.turn_time += delta;
            if self.turn_time >= self.max_turn_time {
                // 0.0 - 6.2831
                let new_ang = random_number(0.0, PI * 2.);

                let dx = new_ang.cos() * self.base.acceleration;
                let dy = new_ang.sin() * self.base.acceleration;

                self.base.velocity = Vector2f::new(dx, dy);

                self.turn_time = 0.0;
            }

            self.base.position += self.base.velocity * delta;
            self.box_area.set_position(self.get_position());
            self.box_area.update();

            self.screen_wrap(SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32, 20.);
            self.update_points();
        }
    }
}
