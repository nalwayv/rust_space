// USE
use sfml::{graphics::*, system::*};

/// simple box shape for visual
pub struct BoxArea {
    position: Vector2f,
    size: Vector2f,
    offset: Vector2f,
    flip_color: bool,
    is_active: bool,
    points: [Vertex; 5],
    transform_points: [Vertex; 5],
}

impl BoxArea {
    /// new box shape
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {

        let w = width / 2.;
        let h = height / 2.;
        let p = [
            Vertex::with_pos((w, -h)),
            Vertex::with_pos((w, h)),
            Vertex::with_pos((-w, h)),
            Vertex::with_pos((-w, -h)),
            Vertex::with_pos((w, -h)),
        ];

        Self {
            position: Vector2f::new(x, y),
            size: Vector2f::new(width, height),
            offset: Vector2f::new(w, h),
            flip_color: false,
            is_active: true,
            points: p,
            transform_points: [Vertex::default(); 5],
        }
    }

    pub fn get_position(&self) -> Vector2f {
        self.position
    }

    pub fn get_size(&self) -> Vector2f {
        self.size
    }

    pub fn toggle_active(&mut self) {
        self.is_active = !self.is_active;
    }

    pub fn set_is_actrive_to(&mut self, value:bool){
        self.is_active = value;
    }

    /// change color of box for collison indication
    pub fn toggle_color(&mut self, value: bool) {
        self.flip_color = value;
    }

    /// draw box to screen
    pub fn draw(&mut self, window: &mut RenderWindow) {
        if self.is_active {
            window.draw_primitives(
                &self.transform_points,
                PrimitiveType::LineStrip,
                RenderStates::default(),
            );
        }
    }

    /// set box position
    pub fn set_position(&mut self, new_pos: Vector2f) {
        self.position = new_pos - self.offset;
    }

    fn update_verts(&mut self) {
        let angle: f32 = 0.0;

        for (idx, p) in self.points.iter_mut().enumerate() {
            let x = p.position.x;
            let y = p.position.y;

            self.transform_points[idx].position.x = (x * angle.cos()) - (y * angle.sin());
            self.transform_points[idx].position.y = (x * angle.sin()) + (y * angle.cos());
            self.transform_points[idx].position += self.position + self.offset;

            if self.flip_color {
                self.transform_points[idx].color = Color::RED;
            } else {
                self.transform_points[idx].color = Color::WHITE;
            }
        }
    }

    /// update box screen position
    pub fn update(&mut self) {
        if self.is_active {
            self.update_verts();
        }
    }
}
