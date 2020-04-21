use sfml::{graphics::*, system::*};

struct LifePoint {
    angle: f32,
    is_active: bool,
    position: Vector2f,
    points: [Vertex; 4],
    transform_points: [Vertex; 4],
}

impl LifePoint {
    fn new(x: f32, y: f32) -> Self {
        let p = [
            Vertex::with_pos((10., 0.)),
            Vertex::with_pos((-10., -7.)),
            Vertex::with_pos((-10., 7.)),
            Vertex::with_pos((10., 0.)),
        ];
        let t = [Vertex::default(); 4];

        Self {
            is_active: true,
            position: Vector2f::new(x, y),
            angle: 4.78,
            points: p,
            transform_points: t,
        }
    }

    fn kill(&mut self) {
        self.is_active = false;
    }

    fn update_points(&mut self) {
        for (idx, p) in self.points.iter_mut().enumerate() {
            let pos = p.position;

            let new_x = (pos.x * self.angle.cos()) - (pos.y * self.angle.sin());
            let new_y = (pos.x * self.angle.sin()) + (pos.y * self.angle.cos());

            self.transform_points[idx].position.x = new_x;
            self.transform_points[idx].position.y = new_y;
            self.transform_points[idx].position += self.position;
        }
    }

    fn update(&mut self) {
        if self.is_active {
            self.update_points();
        }
    }

    fn draw(&mut self, window: &mut RenderWindow) {
        if self.is_active {
            window.draw_primitives(
                &self.transform_points,
                PrimitiveType::LineStrip,
                RenderStates::default(),
            );
        }
    }
}

pub struct Lives {
    amount: usize,
    total: Vec<LifePoint>,
}

impl Lives {
    pub fn new(x: f32, y: f32) -> Self {
        let t = vec![
            LifePoint::new(x, y),
            LifePoint::new(x + 30., y),
            LifePoint::new(x + 60., y),
        ];

        let a = t.len();

        Self {
            total: t,
            amount: a,
        }
    }

    pub fn draw(&mut self, window: &mut RenderWindow) {
        for t in self.total.iter_mut() {
            t.draw(window);
        }
    }

    pub fn remove_life(&mut self) {
        if self.amount > 0 {
            if let Some(x) = self.total.get_mut(self.amount - 1) {
                x.kill();
                self.amount -= 1;
            }
        }
    }

    pub fn update(&mut self) {
        for t in self.total.iter_mut() {
            t.update();
        }
    }
}
