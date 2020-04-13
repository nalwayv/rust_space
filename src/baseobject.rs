use sfml::{system::*};

/// Simple base Object
pub struct BaseObject {
    pub position: Vector2f,
    pub velocity: Vector2f,
    pub acceleration: f32,
    pub angle: f32,
    pub is_active: bool,
}
