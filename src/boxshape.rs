// USE
use sfml::{graphics::*, system::*};

/// simple box shape for visual
pub struct BoxShape<'a> {
    position: Vector2f,
    size: Vector2f,
    offset: Vector2f,
    flip_color: bool,
    is_active: bool,
    is_debug: bool,
    shape: RectangleShape<'a>,
}

impl<'a> BoxShape<'a> {
    /// new box shape
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        let mut r = RectangleShape::default();
        r.set_position((x, y));
        r.set_size((width, height));
        r.set_fill_color(Color::TRANSPARENT);
        r.set_outline_thickness(1.0);
        r.set_outline_color(Color::WHITE);

        Self {
            position: Vector2f::new(x, y),
            size: Vector2f::new(width, height),
            offset: Vector2f::new(width / 2.0, height / 2.0),
            flip_color: false,
            is_active: true,
            is_debug: true,
            shape: r,
        }
    }

    /// toggle debug
    pub fn toggle_debug(&mut self){
        self.is_debug = !self.is_debug;
    }

    /// change color of box for collison indication
    pub fn toggle_color(&mut self, value: bool) {
        self.flip_color = value;
    }
    
    /// check for overlap
    fn check_overlap(&self, min_a: f32, max_a: f32, min_b: f32, max_b: f32) -> bool {
        min_b <= max_a && min_a <= max_b
    }

    /// check for box overlap
    pub fn check_collision(&self, other: &BoxShape) -> bool {
        if !self.is_active && !other.is_active{
            return false;
        }

        // x
        let min_x1 = self.position.x;
        let max_x1 = self.position.x + self.size.x;
        let min_x2 = other.position.x;
        let max_x2 = other.position.x + other.size.x;

        // y
        let min_y1 = self.position.y;
        let max_y1 = self.position.y + self.size.y;
        let min_y2 = other.position.y;
        let max_y2 = other.position.y + other.size.y;

        // results
        let check_a = self.check_overlap(min_x1, max_x1, min_x2, max_x2);
        let check_b = self.check_overlap(min_y1, max_y1, min_y2, max_y2);

        check_a && check_b
    }

    /// draw box to screen
    pub fn draw(&mut self, window: &mut RenderWindow) {
        if self.is_active && self.is_debug {
            window.draw(&self.shape);
        }
    }

    /// set box position
    pub fn set_position(&mut self, new_pos: Vector2f) {
        self.position = new_pos - self.offset;
    }

    /// update box screen position
    pub fn update(&mut self, _delta: f32) {
        if self.is_active {
            if self.flip_color {
                self.shape.set_outline_color(Color::RED);
            } else {
                self.shape.set_outline_color(Color::WHITE);
            }

            self.shape.set_position(self.position);
        }
    }
}
