// USE
use sfml::{graphics::*, system::*};
//
use std::f32::consts::PI;
//
use crate::baseobject::BaseObject;
use crate::globals::{SCREEN_WIDTH, SCREEN_HEIGHT};
use crate::boxarea::BoxArea;
use crate::isactive::IsActive;

/// asteroid types
#[derive(Copy, Clone)]
#[allow(dead_code)]
pub enum AsteroidSize {
    SMALL,
    MEDIUM,
    LARGE,
    NONE,
}

/// asteroid
#[allow(dead_code)]
pub struct Asteroid {
    base: BaseObject,
    asteroid_size: AsteroidSize,
    rotate_speed: f32,
    points: [Vertex; 9],
    transform_points: [Vertex; 9],
    tp: Vec<Vector2f>,
    flip_color: bool,
    is_debug:bool,
    box_area: BoxArea,
}

impl IsActive for Asteroid{
    fn is_active(&self)->bool{
        self.base.is_active
    }

    fn kill(&mut self){
        self.base.is_active = false;
    }
}

// #[allow(dead_code)]
impl Asteroid {
    /// new asteroid
    pub fn new(
        x: f32,
        y: f32,
        ang: f32,
        acc: f32,
        rotate_speed: f32,
        rotate_right: bool,
        size_type: AsteroidSize,
    ) -> Self {
        let points = match size_type {
            AsteroidSize::SMALL => [
                Vertex::with_pos((0.0, -10.20)),
                Vertex::with_pos((5.50, -4.50)),
                Vertex::with_pos((8.0, 0.0)),
                Vertex::with_pos((5.0, 3.50)),
                Vertex::with_pos((0.0, 9.0)),
                Vertex::with_pos((-3.80, 4.0)),
                Vertex::with_pos((-4.45, 0.0)),
                Vertex::with_pos((-2.80, -3.0)),
                Vertex::with_pos((0.0, -10.27)),
            ],
            AsteroidSize::MEDIUM => [
                Vertex::with_pos((0.0, -20.0)),
                Vertex::with_pos((4.0, -6.0)),
                Vertex::with_pos((4.5, 0.0)),
                Vertex::with_pos((4.3, 3.9)),
                Vertex::with_pos((0.0, 10.0)),
                Vertex::with_pos((-8.2, 8.67)),
                Vertex::with_pos((-10.0, 0.0)),
                Vertex::with_pos((-6.4, -14.3)),
                Vertex::with_pos((0.0, -20.0)),
            ],
            AsteroidSize::LARGE => [
                Vertex::with_pos((0.0, -50.0)),
                Vertex::with_pos((40.0, -40.0)),
                Vertex::with_pos((60.0, 0.0)),
                Vertex::with_pos((30.0, 35.0)),
                Vertex::with_pos((0.0, 50.0)),
                Vertex::with_pos((-40.5, 40.0)),
                Vertex::with_pos((-55.0, 0.0)),
                Vertex::with_pos((-35.0, -30.0)),
                Vertex::with_pos((0.0, -50.0)),
            ],
            AsteroidSize::NONE => [Vertex::default(); 9],
        };

        let dx = ang.cos() * acc;
        let dy = ang.sin() * acc;

        let speed = match rotate_right {
            true => rotate_speed,
            false => -rotate_speed,
        };

        let ba = BoxArea::new(x, y, 110., 110.);

        Self {
            base: BaseObject {
                position: Vector2f::new(x, y),
                velocity: Vector2f::new(dx, dy),
                acceleration: acc,
                angle: ang,
                is_active: true,
            },
            asteroid_size: size_type,
            rotate_speed: speed,
            points: points,
            transform_points: [Vertex::default(); 9],
            tp: vec![Vector2f::default(); 9],
            flip_color: false,
            is_debug: false,
            box_area: ba,
        }
    }

    pub fn get_box_area(&self)->&BoxArea{
        &self.box_area
    }

    /// return current screen position
    pub fn get_position(&self) -> Vector2f {
        self.base.position
    }

    pub fn get_asteroid_type(&self)->AsteroidSize{
        self.asteroid_size
    }

    /// toggle color
    pub fn toggle_color(&mut self, value: bool) {
        self.flip_color = value;
    }

    /// get vec of the current transform points for this asteroid
    pub fn get_tp(&self) -> &Vec<Vector2f> {
        &self.tp
    }

    /// update transform points
    fn update_points(&mut self) {
        for (idx, p) in self.points.iter_mut().enumerate() {
            let x = p.position.x;
            let y = p.position.y;

            let new_x = (x * self.base.angle.cos()) - (y * self.base.angle.sin());
            let new_y = (x * self.base.angle.sin()) + (y * self.base.angle.cos());

            self.transform_points[idx].position.x = new_x;
            self.transform_points[idx].position.y = new_y;
            self.transform_points[idx].position += self.base.position;

            self.tp[idx] = self.transform_points[idx].position;

            if self.flip_color {
                self.transform_points[idx].color = Color::RED;
            } else {
                self.transform_points[idx].color = Color::WHITE;
            }
        }
    }

    /// draw asteroids transform points to screen in LineStrip format
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

    pub fn update(&mut self, delta: f32) {
        if self.is_active() {
            self.base.angle += self.rotate_speed * delta;

            if self.base.angle < 0.0 {
                self.base.angle += PI * 2.0;
            }
            if self.base.angle > PI * 2.0 {
                self.base.angle -= PI * 2.0;
            }

            self.base.position += self.base.velocity * self.base.acceleration * delta;
            
            self.box_area.set_position(self.get_position());
            self.box_area.update();
            
            self.screen_wrap(SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32, 50.);
            self.update_points();
        }
    }
}
