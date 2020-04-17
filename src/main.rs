// ----------
// IMPORTS
// ----------
// other files to include
mod asteroid;
mod baseobject;
mod boxarea;
mod bullet;
mod globals;
mod isactive;
mod ship;

use crate::asteroid::*;
use crate::boxarea::*;
use crate::bullet::*;
use crate::globals::*;
use crate::isactive::*;
use crate::ship::*;

use rand::{thread_rng, Rng};
use sfml::{graphics::*, system::*, window::*};
use std::collections::HashMap;
use std::f32::{consts::PI, INFINITY};

// ----------
// P SYSTEM
// ----------

struct Particle {
    position: Vector2f,
    velocity: Vector2f,
    acceleration: f32,
    angle: f32,
    shape: [Vertex; 2],
    transform_shape: [Vertex; 2],
    life_time: f32,
    max_life_time: f32,
    is_active: bool,
}

impl IsActive for Particle {
    fn is_active(&self) -> bool {
        self.is_active
    }

    fn kill(&mut self) {
        self.is_active = false;
    }
}

impl Particle {
    fn new() -> Self {
        let p_shape = [Vertex::with_pos((-3.0, 0.)), Vertex::with_pos((3.0, 0.))];

        Self {
            position: Vector2f::default(),
            velocity: Vector2f::default(),
            acceleration: 0.,
            angle: 0.,
            shape: p_shape,
            transform_shape: [Vertex::default(); 2],
            life_time: 0.,
            max_life_time: 0.,
            is_active: true,
        }
    }

    fn init(&mut self, x: f32, y: f32, acc: f32, ang: f32, life: f32) {
        let dx = ang.cos() * acc;
        let dy = ang.sin() * acc;

        self.position.x = x;
        self.position.y = y;
        self.velocity.x = dx;
        self.velocity.y = dy;
        self.angle = ang;
        self.acceleration = acc;
        self.max_life_time = life;
    }

    fn draw(&mut self, window: &mut RenderWindow) {
        if self.is_active {
            window.draw_primitives(
                &self.transform_shape,
                PrimitiveType::LineStrip,
                RenderStates::default(),
            );
        }
    }

    fn update_points(&mut self) {
        for (idx, p) in self.shape.iter_mut().enumerate() {
            let x = p.position.x;
            let y = p.position.y;

            let new_x = (x * self.angle.cos()) - (y * self.angle.sin());
            let new_y = (x * self.angle.sin()) + (y * self.angle.cos());

            self.transform_shape[idx].position.x = new_x;
            self.transform_shape[idx].position.y = new_y;
            self.transform_shape[idx].position += self.position;
        }
    }

    fn is_dead(&self) -> bool {
        self.is_active == false
    }

    fn update(&mut self, delta: f32) {
        if self.is_active {
            self.life_time += delta;

            if self.life_time >= self.max_life_time {
                self.kill();
            }

            self.position += self.velocity * delta;

            self.update_points();
        }
    }
}

struct Explosion {
    position: Vector2f,
    radius: f32,
    particles: [Particle; 8],
    tally: usize,
    is_active: bool,
}

impl IsActive for Explosion {
    fn is_active(&self) -> bool {
        self.is_active
    }

    fn kill(&mut self) {
        self.is_active = false;
    }
}

impl Explosion {
    fn new() -> Self {
        let p = [
            Particle::new(),
            Particle::new(),
            Particle::new(),
            Particle::new(),
            Particle::new(),
            Particle::new(),
            Particle::new(),
            Particle::new(),
        ];

        Self {
            position: Vector2f::default(),
            tally: 0,
            radius: 3.,
            particles: p,
            is_active: false,
        }
    }

    fn init(&mut self, x: f32, y: f32) {
        //let mut angle = 0.0;
        self.position.x = x;
        self.position.y = y;
        self.is_active = true;
        let n = self.particles.len() as f32;
        let slice = (PI * 2.) / n;
        let mut rng = thread_rng();

        for (idx, particle) in self.particles.iter_mut().enumerate() {
            let rng_ang: f32 = rng.gen_range(1., 3.14);

            let ang = (idx as f32) + rng_ang * slice;

            // println!("ang: {}", ang);

            let px = self.position.x + ang.cos() * self.radius;
            let py = self.position.y + ang.sin() * self.radius;

            // rng life and speed
            let life: f32 = rng.gen_range(1.5, 2.5);
            let acc: f32 = rng.gen_range(100., 200.);

            particle.init(px, py, acc, ang, life);
        }
    }

    fn draw(&mut self, window: &mut RenderWindow) {
        if self.is_active {
            for p in self.particles.iter_mut() {
                p.draw(window);
            }
        }
    }

    fn update(&mut self, delta: f32) {
        if self.is_active {
            for p in self.particles.iter_mut() {
                p.update(delta);

                if p.is_dead() {
                    self.tally += 1;
                }
            }

            if self.tally >= self.particles.len() {
                self.kill();
            }
        }
    }
}

// ----------
// FUNCS
// ----------

/// rng true or false
fn random_bool() -> bool {
    let mut rng = thread_rng();
    let result: bool = rng.gen_bool(1.0 / 3.0);
    result
}

/// rng an angle between 0 and 360
fn random_number(min_v: f32, max_v: f32) -> f32 {
    let mut rng = thread_rng();

    let a = min_v.min(max_v);
    let b = min_v.max(max_v);

    let result: f32 = rng.gen_range(a, b);
    result
}

// degreese to radians
fn d_to_r(deg: f32) -> f32 {
    deg * PI / 180.0
}

/// get vector normal between two points
fn get_vector_normal(p1: Vector2f, p2: Vector2f) -> Vector2f {
    // (-y, x)
    Vector2f::new(-(p2.y - p1.y), p2.x - p1.x)
}

/// get vector dot product between two points
fn get_dot_product(v1: Vector2f, v2: Vector2f) -> f32 {
    let x = v1.x * v2.x;
    let y = v1.y * v2.y;
    let result = x + y;
    result
}

// link https://www.youtube.com/watch?v=7Ik2vowGcU0&t=414s
fn sat(points_1: &Vec<Vector2f>, points_2: &Vec<Vector2f>) -> bool {
    // check shadow overlap
    let mut p1 = &points_1;
    let mut p2 = &points_2;

    for i in 0..2 {
        if i == 1 {
            // swap
            p1 = &points_2;
            p2 = &points_1;
        }

        let n1 = p1.len();
        let n2 = p2.len();

        for x in 0..n1 {
            let y = (x + 1) % n1;
            // norm
            let normal = get_vector_normal(p1[x], p1[y]);

            // shape 1
            let mut min_value1 = INFINITY;
            let mut max_value1 = -INFINITY;
            for idx in 0..n1 {
                let dot = get_dot_product(p1[idx], normal);
                // update min/max
                min_value1 = min_value1.min(dot);
                max_value1 = max_value1.max(dot);
            }

            // shape 2
            let mut min_value2 = INFINITY;
            let mut max_value2 = -INFINITY;
            for idx in 0..n2 {
                let dot = get_dot_product(p2[idx], normal);
                // update min/max
                min_value2 = min_value2.min(dot);
                max_value2 = max_value2.max(dot);
            }

            if !(max_value2 >= min_value1 && max_value1 >= min_value2) {
                return false;
            }
        }
    }

    true
}

fn check_overlap(min_a: f32, max_a: f32, min_b: f32, max_b: f32) -> bool {
    min_b <= max_a && min_a <= max_b
}

/// aabb collision
pub fn aabb(box1: &BoxArea, box2: &BoxArea) -> bool {
    // x
    let min_x1 = box1.get_position().x;
    let max_x1 = box1.get_position().x + box1.get_size().x;
    let min_x2 = box2.get_position().x;
    let max_x2 = box2.get_position().x + box2.get_size().x;

    // y
    let min_y1 = box1.get_position().y;
    let max_y1 = box1.get_position().y + box1.get_size().y;
    let min_y2 = box2.get_position().y;
    let max_y2 = box2.get_position().y + box2.get_size().y;

    // results
    let check_a = check_overlap(min_x1, max_x1, min_x2, max_x2);
    let check_b = check_overlap(min_y1, max_y1, min_y2, max_y2);

    check_a && check_b
}

fn filter_out_inactive<T>(value: &mut Vec<T>)
where
    T: IsActive,
{
    if !value.is_empty() {
        value.retain(|x| x.is_active());
    }
}

fn populate_key_map(key_map: &mut HashMap<&Key, bool>) {
    key_map.insert(&Key::W, false);
    key_map.insert(&Key::A, false);
    key_map.insert(&Key::D, false);
    key_map.insert(&Key::Space, false);
}

fn polulate_asteroids(x: f32, y: f32, asteroid_size: AsteroidSize, asteroids: &mut Vec<Asteroid>) {
    let new_a = Asteroid::new(
        x,
        y,
        d_to_r(random_number(1., 360.)),
        8.,
        3.,
        random_bool(),
        asteroid_size,
    );
    asteroids.push(new_a);
}

/// main run for sfml window
fn run(width: u32, height: u32) {
    let mut window = RenderWindow::new((width, height), "space", Style::CLOSE, &Default::default());
    window.set_mouse_cursor_visible(false);
    window.set_framerate_limit(30);
    let mut is_paused = false;
    let mut clock = Clock::start();

    // key maps
    let mut key_map: HashMap<&Key, bool> = HashMap::new();
    populate_key_map(&mut key_map);

    // ship
    let center_x = width as f32 * 0.5;
    let center_y = height as f32 * 0.5;
    let mut ship = Ship::new(center_x, center_y, 0.);

    // bullets
    let mut shoot_time = 0.0;
    let max_shoot_time = 0.5;
    let mut bullets: Vec<Bullet> = Vec::new();

    // asteroids
    let asteroid1 = Asteroid::new(
        50.,
        350.,
        d_to_r(random_number(1., 360.)),
        8.,
        2.,
        random_bool(),
        AsteroidSize::LARGE,
    );
    let asteroid2 = Asteroid::new(
        150.,
        150.,
        d_to_r(random_number(1., 360.)),
        9.,
        2.,
        random_bool(),
        AsteroidSize::LARGE,
    );
    let asteroid3 = Asteroid::new(
        10.,
        10.,
        d_to_r(random_number(1., 360.)),
        8.,
        2.,
        random_bool(),
        AsteroidSize::LARGE,
    );
    let mut gen_medium = false;
    let mut gen_small = false;
    let mut ax = 0.0;
    let mut ay = 0.0;
    let mut asteroids = vec![asteroid1, asteroid2, asteroid3];

    // explosions
    let mut explosions: Vec<Explosion> = vec![];

    while window.is_open() {
        // INPUTS ---
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed => window.close(),
                Event::KeyPressed { code, .. } => match code {
                    Key::Escape => window.close(),
                    Key::P => is_paused = !is_paused,

                    Key::W => {
                        if let Some(x) = key_map.get_mut(&Key::W) {
                            *x = true;
                        }
                    }
                    Key::D => {
                        if let Some(x) = key_map.get_mut(&Key::D) {
                            *x = true;
                        }
                    }
                    Key::A => {
                        if let Some(x) = key_map.get_mut(&Key::A) {
                            *x = true;
                        }
                    }
                    Key::Space => {
                        if let Some(x) = key_map.get_mut(&Key::Space) {
                            *x = true;
                        }
                    }
                    _ => {}
                },
                Event::KeyReleased { code, .. } => match code {
                    Key::W => {
                        if let Some(x) = key_map.get_mut(&Key::W) {
                            *x = false;
                        }
                    }
                    Key::D => {
                        if let Some(x) = key_map.get_mut(&Key::D) {
                            *x = false;
                        }
                    }
                    Key::A => {
                        if let Some(x) = key_map.get_mut(&Key::A) {
                            *x = false;
                        }
                    }
                    Key::Space => {
                        if let Some(x) = key_map.get_mut(&Key::Space) {
                            *x = false;
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        if !is_paused {
            let delta = clock.restart().as_seconds();

            // INPUTS ---
            ship.inputs(&key_map);

            // COLLISION ---

            // collision
            // ship asteroid
            for a in asteroids.iter_mut() {
                if aabb(ship.get_box_area(), a.get_box_area()) {
                    if sat(ship.get_tp(), a.get_tp()) {
                        ship.toggle_color(true);
                        a.toggle_color(true);
                    }
                } else {
                    ship.toggle_color(false);
                    a.toggle_color(false);
                }
            }

            // asteroid bullet
            if !bullets.is_empty() {
                for b in bullets.iter_mut() {
                    for a in asteroids.iter_mut() {
                        if a.is_active() {
                            if aabb(a.get_box_area(), b.get_box_area()) {
                                if sat(a.get_tp(), b.get_tp()) {
                                    a.toggle_color(true);

                                    // init new explosions
                                    let x = b.get_position().x;
                                    let y = b.get_position().y;

                                    let mut ex = Explosion::new();
                                    ex.init(x, y);
                                    explosions.push(ex);

                                    match a.get_asteroid_type() {
                                        AsteroidSize::LARGE => {
                                            gen_medium = true;
                                            ax = a.get_position().x;
                                            ay = a.get_position().y;
                                        }
                                        AsteroidSize::MEDIUM => {
                                            gen_small = true;
                                            ax = a.get_position().x;
                                            ay = a.get_position().y;
                                        }
                                        _ => {}
                                    }
                                    // remove
                                    a.kill();
                                    b.kill();
                                }
                            } else {
                                a.toggle_color(false);
                            }
                        }
                    }
                }
            }

            if gen_medium {
                polulate_asteroids(ax, ay, AsteroidSize::MEDIUM, &mut asteroids);
                gen_medium = false;
            }
            if gen_small {
                polulate_asteroids(ax, ay, AsteroidSize::SMALL, &mut asteroids);
                gen_small = false;
            }

            // UPDATE ---
            // explosion.update(delta);
            if !explosions.is_empty() {
                for e in explosions.iter_mut() {
                    e.update(delta);
                }
            }
            filter_out_inactive(&mut explosions);

            ship.update(delta);

            shoot_time += delta;
            if ship.is_fireing() && shoot_time > max_shoot_time {
                let new_b = Bullet::new(
                    ship.get_position().x,
                    ship.get_position().y,
                    ship.get_angle(),
                );

                bullets.push(new_b);
                shoot_time = 0.;
            }

            if !bullets.is_empty() {
                for bullet in bullets.iter_mut() {
                    bullet.update(delta);
                }
            }

            filter_out_inactive(&mut bullets);

            if !asteroids.is_empty() {
                for a in asteroids.iter_mut() {
                    a.update(delta);
                }
            }

            // RENDER ---

            window.clear(Color::BLACK);
            // ->
            // explosions
            if !explosions.is_empty() {
                for e in explosions.iter_mut() {
                    e.draw(&mut window);
                }
            }

            //ship
            ship.draw(&mut window);

            // asteroid
            if !asteroids.is_empty() {
                for a in asteroids.iter_mut() {
                    a.draw(&mut window);
                }
            }
            // bullets
            if !bullets.is_empty() {
                for bullet in bullets.iter_mut() {
                    bullet.draw(&mut window);
                }
            }

            // <-
            window.display();
        } else {
            // so when un pausing objects
            // dont just jump across the screen.
            clock.restart();
        }
    }
}

fn main() {
    run(SCREEN_WIDTH, SCREEN_HEIGHT);
}
