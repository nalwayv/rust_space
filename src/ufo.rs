use sfml::{graphics::*, system::*};
use crate::baseobject::BaseObject;
use crate::boxarea::BoxArea; 
use crate::isactive::IsActive;

pub struct Ufo {
    base: BaseObject,
    shoot_time: f32,
    max_shoot_time: f32,
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
    pub fn new() -> Self {

        let p = [
            Vertex::with_pos((5., -5.)),
            Vertex::with_pos((16., 0.)),
            Vertex::with_pos((5., 5.)),
            Vertex::with_pos((-5., 5.)),
            Vertex::with_pos((-16., 0.)),
            Vertex::with_pos((-5., -5.)),
            Vertex::with_pos((5., -5.)),
        ];

        let px = 50.0;
        let py = 50.0;
        let ang: f32 = 0.0;
        let acc = 100.0;
        let dx = ang.cos() * acc;
        let dy = ang.sin() * acc;

        let ba = BoxArea::new(px, py, 70., 70.);

        Self {
            base:BaseObject{
                position:  Vector2f::new(px, py),
                velocity: Vector2f::new(dx, dy),
                acceleration: 0.,
                angle: ang,
                is_active: true,
            },
            points: p,
            shoot_time: 0.0,
            max_shoot_time: 1.5,
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

    fn update_points(&mut self) {
        for (idx, p) in self.points.iter_mut().enumerate() {
            let x = p.position.x;
            let y = p.position.y;

            let new_x = (x * self.base.angle.cos()) - (y * self.base.angle.sin());
            let new_y = (x * self.base.angle.sin()) + (y * self.base.angle.cos());

            self.transform_points[idx].position.x = new_x;
            self.transform_points[idx].position.y = new_y;
            self.transform_points[idx].position += self.base.position;

            self.tp[idx] =  self.transform_points[idx].position;
        }
    }

    pub fn update(&mut self, delta: f32) {
        if self.is_active() {
            self.shoot_time += delta;
            if self.shoot_time >= self.max_shoot_time {
                self.is_shooting = true;
                self.shoot_time = 0.;
            } else {
                self.is_shooting = false;
            }

            self.box_area.set_position(self.get_position());
            self.box_area.update();

            self.update_points();
        }
    }
}
