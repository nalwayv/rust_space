// USE
use sfml::{graphics::*, system::*};
//
use crate::baseobject::*;
use crate::boxshape::*;

/// bullet
pub struct Bullet {
    base: BaseObject,
    life_timer: f32,
    max_life_time: f32,
    bullet_points: [Vertex; 5],
    transform_bullet_points: [Vertex; 5],
    transform_points: Vec<Vector2f>,
    // for debug
    flip_color:bool,
    is_debug: bool,
    debug_box: BoxShape,
}

impl Bullet {
    pub fn new(x: f32, y:f32, ang:f32) -> Self {
        let bullet_v = [
            Vertex::with_pos((5., -0.2)), 
            Vertex::with_pos((5., 0.2)),
            Vertex::with_pos((-5.0, 0.2)),
            Vertex::with_pos((-5.0, -0.2)),
            Vertex::with_pos((5.0, -0.2)),
        ];
        let draw_bv = [Vertex::default(); 5];

        let d_box = BoxShape::new(x, y, 20., 20.);
        let acc = 400.;

        let dx = ang.cos() * acc;
        let dy = ang.sin() * acc;

        let tp = vec![Vector2f::default(); 5];

        Self {
            base: BaseObject {
                position: Vector2f::new(x, y),
                velocity: Vector2f::new(dx, dy),
                acceleration: acc,
                is_active: true,
                angle: ang,
            },
            life_timer: 0.0,
            max_life_time: 1.5,
            bullet_points: bullet_v,
            transform_bullet_points: draw_bv,
            transform_points:tp,
            flip_color: false,
            is_debug: true,
            debug_box: d_box,
        }
    }

    /// get vec of the current transform points for this ship
    pub fn get_tp(&self) -> &Vec<Vector2f> {
        &self.transform_points
    }

    pub fn get_box(&self) -> &BoxShape {
        &self.debug_box
    }

    // pub fn toggle_debug(&mut self){
    //     self.is_debug = !self.is_debug;
    // }

    pub fn toggle_debug_color(&mut self, value: bool) {
        self.debug_box.toggle_color(value);
    }

    pub fn toggle_color(&mut self, value: bool) {
        self.flip_color = value;
    }

    pub fn is_active(&self)->bool{
        self.base.is_active
    }

    pub fn draw(&mut self, window: &mut RenderWindow) {
        if self.base.is_active {

            if self.is_debug {
                self.debug_box.draw(window);
            }

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


            if self.flip_color {
                self.transform_bullet_points[idx].color = Color::RED;
            } else {
                self.transform_bullet_points[idx].color = Color::WHITE;
            }

            self.transform_points[idx] = self.transform_bullet_points[idx].position;
        }
    }

    pub fn update(&mut self, delta: f32) {
        if self.base.is_active {
            self.base.position += self.base.velocity * delta;

            self.life_timer += delta;
            if self.life_timer > self.max_life_time {
                self.base.is_active = false;
            }
            
            self.update_points();
            
            // debug
            if self.is_debug {
                self.debug_box.set_position(self.base.position);
                self.debug_box.update(delta);
            }
        }
    }
}
