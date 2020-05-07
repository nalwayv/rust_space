// USE
use sfml::{graphics::*, system::*};
//
use crate::baseobject::BaseObject;
use crate::boxarea::BoxArea;
use crate::isactive::IsActive;

// so bullets from alien dont destroy asteroids
#[derive(PartialEq)]
pub enum ShooterType {
    PLAYER,
    ALIEN,
}

/// bullet
pub struct Bullet {
    base: BaseObject,
    life_timer: f32,
    max_life_time: f32,
    bullet_points: [Vertex; 5],
    transform_bullet_points: [Vertex; 5],
    transform_points: Vec<Vector2f>,
    is_debug: bool,
    mask: ShooterType,
    box_area: BoxArea,
}

impl IsActive for Bullet {
    fn is_active(&self) -> bool {
        self.base.is_active
    }

    fn kill(&mut self) {
        self.base.is_active = false;
    }
}

impl Bullet {
    pub fn new(x: f32, y: f32, ang: f32, mask: ShooterType) -> Self {
        let bullet_v = [
            Vertex::with_pos((5., -0.2)),
            Vertex::with_pos((5., 0.2)),
            Vertex::with_pos((-5.0, 0.2)),
            Vertex::with_pos((-5.0, -0.2)),
            Vertex::with_pos((5.0, -0.2)),
        ];
        let draw_bv = [Vertex::default(); 5];

        let acc = 400.;

        let dx = ang.cos() * acc;
        let dy = ang.sin() * acc;

        let tp = vec![Vector2f::default(); 5];

        let ba = BoxArea::new(x, y, 10., 10.);

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
            transform_points: tp,
            is_debug: false,
            box_area: ba,
            mask: mask,
        }
    }

    /// get owner of this current projectile
    pub fn get_shooter_type(&self) -> &ShooterType {
        &self.mask
    }

    /// get vec of the current transform points for this ship
    pub fn get_tp(&self) -> &Vec<Vector2f> {
        &self.transform_points
    }

    pub fn get_box_area(&self) -> &BoxArea {
        &self.box_area
    }

    /// return current screen position
    pub fn get_position(&self) -> Vector2f {
        self.base.position
    }

    pub fn draw(&mut self, window: &mut RenderWindow) {
        if self.is_active() {
            if self.is_debug {
                self.box_area.draw(window);
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

            match self.mask {
                ShooterType::ALIEN => self.transform_bullet_points[idx].color = Color::RED,
                ShooterType::PLAYER => self.transform_bullet_points[idx].color = Color::WHITE,
            }

            self.transform_points[idx] = self.transform_bullet_points[idx].position;
        }
    }

    pub fn update(&mut self, delta: f32) {
        if self.is_active() {
            self.base.position += self.base.velocity * delta;

            self.life_timer += delta;
            if self.life_timer > self.max_life_time {
                self.kill();
            }

            self.box_area.set_position(self.get_position());
            self.box_area.update();
            self.update_points();
        }
    }
}
