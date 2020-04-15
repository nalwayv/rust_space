// USE
use sfml::{graphics::*, system::*};
//
use std::f32::consts::PI;
//
use crate::baseobject::*;

/// asteroid types
pub enum AsteroidSize {
    SMALL,
    MEDIUM,
    LARGE,
}

/// asteroid
pub struct Asteroid {
    base: BaseObject,
    size: AsteroidSize,
    rotate_speed: f32,
    screen_padding: f32,
    asteroid_points: [Vertex; 9],
    transform_asteroid_points: [Vertex; 9],
    transform_points: Vec<Vector2f>,
    flip_color: bool,
}

#[allow(dead_code)]
impl Asteroid {
    /// new asteroid
    pub fn new() -> Self {
        Self {
            base: BaseObject {
                position: Vector2f::default(),
                velocity: Vector2f::default(),
                acceleration: 0.0,
                angle: 0.0,
                is_active: false,
            },
            size: AsteroidSize::SMALL,
            rotate_speed: 0.0,
            screen_padding: 0.0,
            asteroid_points: [Vertex::default(); 9],
            transform_asteroid_points: [Vertex::default(); 9],

            transform_points: vec![Vector2f::default(); 9],

            flip_color:false,
        }
    }

    /// init a new asteroid to a small type
    pub fn init_small(&mut self, x: f32, y: f32, ang: f32, rotate_right: bool) {
        self.asteroid_points = [
            Vertex::with_pos((0.0, -10.20)),
            Vertex::with_pos((5.50, -4.50)),
            Vertex::with_pos((8.0, 0.0)),
            Vertex::with_pos((5.0, 3.50)),
            Vertex::with_pos((0.0, 9.0)),
            Vertex::with_pos((-3.80, 4.0)),
            Vertex::with_pos((-4.45, 0.0)),
            Vertex::with_pos((-2.80, -3.0)),
            Vertex::with_pos((0.0, -10.27)),
        ];
        self.size = AsteroidSize::SMALL;

        self.base.position.x = x;
        self.base.position.y = y;

        self.base.angle = ang;
        self.base.acceleration = 10.0;

        self.rotate_speed = match rotate_right {
            true => 8.0,
            false => -8.0,
        };

        self.screen_padding = 20.0;

        self.base.velocity.x = self.base.angle.cos() * self.base.acceleration;
        self.base.velocity.y = self.base.angle.sin() * self.base.acceleration;

        self.base.is_active = true;
    }

    /// init a new asteroid to a medium type
    pub fn init_medium(&mut self, x: f32, y: f32, ang: f32, rotate_right: bool) {
        self.asteroid_points = [
            Vertex::with_pos((0.0, -20.0)),
            Vertex::with_pos((4.0, -6.0)),
            Vertex::with_pos((4.5, 0.0)),
            Vertex::with_pos((4.3, 3.9)),
            Vertex::with_pos((0.0, 10.0)),
            Vertex::with_pos((-8.2, 8.67)),
            Vertex::with_pos((-10.0, 0.0)),
            Vertex::with_pos((-6.4, -14.3)),
            Vertex::with_pos((0.0, -20.0)),
        ];
        self.size = AsteroidSize::MEDIUM;

        self.base.position.x = x;
        self.base.position.y = y;

        self.base.angle = ang;
        self.base.acceleration = 8.0;
        self.rotate_speed = match rotate_right {
            true => 4.0,
            false => -4.0,
        };

        self.screen_padding = 30.0;

        self.base.velocity.x = self.base.angle.cos() * self.base.acceleration;
        self.base.velocity.y = self.base.angle.sin() * self.base.acceleration;

        self.base.is_active = true;
    }

    /// init a new asteroid to a large type
    pub fn init_large(&mut self, x: f32, y: f32, ang: f32, rotate_right: bool) {
        self.asteroid_points = [
            Vertex::with_pos((0.0, -50.0)),
            Vertex::with_pos((40.0, -40.0)),
            Vertex::with_pos((60.0, 0.0)),
            Vertex::with_pos((30.0, 35.0)),
            Vertex::with_pos((0.0, 50.0)),
            Vertex::with_pos((-40.5, 40.0)),
            Vertex::with_pos((-55.0, 0.0)),
            Vertex::with_pos((-35.0, -30.0)),
            Vertex::with_pos((0.0, -50.0)),
        ];
        self.size = AsteroidSize::LARGE;

        self.base.position.x = x;
        self.base.position.y = y;

        self.base.angle = ang;
        self.base.acceleration = 4.0;
        self.rotate_speed = match rotate_right {
            true => 1.0,
            false => -1.0,
        };
        self.screen_padding = 50.0;

        self.base.velocity.x = self.base.angle.cos() * self.base.acceleration;
        self.base.velocity.y = self.base.angle.sin() * self.base.acceleration;

        self.base.is_active = true;
    }

    /// return current screen position
    pub fn get_position(&self) -> Vector2f {
        self.base.position
    }

    pub fn toggle_color(&mut self, value: bool){
        self.flip_color = value;
    }

    /// get vec of the current transform points for this asteroid
    pub fn get_tp(&self)->&Vec<Vector2f>{
        &self.transform_points
    }

    /// update transform points
    fn update_points(&mut self) {
        for (idx, p) in self.asteroid_points.iter_mut().enumerate() {
            let x = p.position.x;
            let y = p.position.y;

            let new_x = (x * self.base.angle.cos()) - (y * self.base.angle.sin());
            let new_y = (x * self.base.angle.sin()) + (y * self.base.angle.cos());

            self.transform_asteroid_points[idx].position.x = new_x;
            self.transform_asteroid_points[idx].position.y = new_y;
            self.transform_asteroid_points[idx].position += self.base.position;

            self.transform_points[idx] = self.transform_asteroid_points[idx].position;

            if self.flip_color{
                self.transform_asteroid_points[idx].color = Color::RED;
            }else{
                self.transform_asteroid_points[idx].color = Color::WHITE;
            }
        }
    }

    /// draw asteroids transform points to screen in LineStrip format
    pub fn draw(&mut self, window: &mut RenderWindow) {
        if self.base.is_active {
            window.draw_primitives(
                &self.transform_asteroid_points,
                PrimitiveType::LineStrip,
                RenderStates::default(),
            );
        }
    }

    pub fn update(&mut self, delta: f32) {
        if self.base.is_active {
            self.base.angle += self.rotate_speed * delta;

            if self.base.angle < 0.0 {
                self.base.angle += PI * 2.0;
            }
            if self.base.angle > PI * 2.0 {
                self.base.angle -= PI * 2.0;
            }

            let screen_width = 800.0; 
            let screen_height = 600.0;
            let screen_edge = 0.0;

            //wrap
            if self.base.position.x > screen_width + self.screen_padding {
                self.base.position.x = screen_edge - self.screen_padding;
            }

            if self.base.position.x < screen_edge - self.screen_padding {
                self.base.position.x = screen_width + self.screen_padding;
            }

            if self.base.position.y > screen_height + self.screen_padding {
                self.base.position.y = screen_edge - self.screen_padding;
            }

            if self.base.position.y < screen_edge - self.screen_padding {
                self.base.position.y = screen_height + self.screen_padding;
            }

            self.base.position += self.base.velocity * self.base.acceleration * delta;
            
            self.update_points();
        }
    }
}
