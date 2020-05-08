// ----------
// IMPORTS
// ----------
// other files to include
mod asteroid;
mod baseobject;
mod boxarea;
mod bullet;
mod explosion;
mod globals;
mod isactive;
mod lives;
mod particle;
mod ship;
mod ufo;

use crate::asteroid::*;
use crate::boxarea::*;
use crate::bullet::*;
use crate::explosion::*;
use crate::globals::*;
use crate::isactive::*;
use crate::lives::*;
use crate::ship::*;
use crate::ufo::*;

use sfml::{graphics::*, system::*, window::*, audio::*};
use std::collections::HashMap;
use std::f32::INFINITY;

// ----------
// SOUND MANAGER
// ----------
struct SoundManager{
    sound_map: HashMap<String, SfBox<SoundBuffer>>,
}

impl SoundManager{

    fn new()->Self{
        Self{
            sound_map: HashMap::default(),
        }
    }

    fn load(&mut self, id:&str, file_path:&str){
        if !self.sound_map.contains_key(id){
            let new_sound = SoundBuffer::from_file(file_path);
            
            if let Some(x) = new_sound{
                self.sound_map.insert(String::from(id), x);
            }
        }
    }


    pub fn get(&self, id: &str)->Option<&SoundBuffer>{

        if let Some(x) = self.sound_map.get(id){
            return Some(x);
        }
        None
    }
}

// ----------
// FUNCS
// ----------
enum GenAsteroid {
    MEDIUM(f32, f32),
    SMALL(f32, f32),
    NONE,
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
            let normal = v2_normal(p1[x], p1[y]);

            // shape 1
            let mut min_value1 = INFINITY;
            let mut max_value1 = -INFINITY;
            for idx in 0..n1 {
                let dot = v2_dot(p1[idx], normal);
                // update min/max
                min_value1 = min_value1.min(dot);
                max_value1 = max_value1.max(dot);
            }

            // shape 2
            let mut min_value2 = INFINITY;
            let mut max_value2 = -INFINITY;
            for idx in 0..n2 {
                let dot = v2_dot(p2[idx], normal);
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

    let is_overlapping = |min_a: f32, max_a: f32, min_b: f32, max_b: f32| -> bool {
        min_b <= max_a && min_a <= max_b
    };

    // results
    let check_a = is_overlapping(min_x1, max_x1, min_x2, max_x2);
    let check_b = is_overlapping(min_y1, max_y1, min_y2, max_y2);

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
        random_number(10., 15.),
        3.,
        random_bool(),
        asteroid_size,
    );

    let new_b = Asteroid::new(
        x,
        y,
        d_to_r(random_number(1., 360.)),
        random_number(10., 15.),
        3.,
        random_bool(),
        asteroid_size,
    );
    asteroids.push(new_a);
    asteroids.push(new_b);
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

    // SM
    let mut sm = SoundManager::new();
    sm.load("fire", "assets/sound/fire.wav");
    sm.load("explosion", "assets/sound/explosion.wav");
    
    let mut fire_sound = Sound::default();
    let mut explosion_sound = Sound::default();

    if let Some(f) = sm.get("fire"){
        fire_sound.set_buffer(f);
    }
    if let Some(e) = sm.get("explosion"){
        explosion_sound.set_buffer(e);
    }

    // Ship
    let center_x = width as f32 * 0.5;
    let center_y = height as f32 * 0.5;
    let mut ship = Ship::new(center_x, center_y, 0.);
    let mut is_damaged = false;

    // Ufo
    let mut ufo = Ufo::new(50. ,50., 100.);

    // Bullets
    let mut shoot_time = 0.0;
    let max_shoot_time = 0.5;
    let mut bullets: Vec<Bullet> = Vec::new();

    // Asteroids
    let mut gen_new_asteroids = false;
    let mut gen_type = GenAsteroid::NONE;
    let mut asteroids = vec![
        Asteroid::new(
            50.,
            350.,
            d_to_r(random_number(1., 360.)),
            random_number(5., 15.),
            2.,
            random_bool(),
            AsteroidSize::LARGE,
        ),
        Asteroid::new(
            150.,
            150.,
            d_to_r(random_number(1., 360.)),
            random_number(5., 15.),
            2.,
            random_bool(),
            AsteroidSize::LARGE,
        ),
        Asteroid::new(
            10.,
            10.,
            d_to_r(random_number(1., 360.)),
            random_number(5., 15.),
            2.,
            random_bool(),
            AsteroidSize::LARGE,
        ),
    ];

    // Explosions
    let mut explosions: Vec<Explosion> = vec![];

    // Lives
    let mut lives = Lives::new(50., 25.);

    while window.is_open() {
        // INPUTS ---
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed => window.close(),
                Event::KeyPressed { code, .. } => match code {
                    Key::Escape => window.close(),
                    Key::P => is_paused = !is_paused,

                    Key::L => ship.alive(),
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

            // buller / ufo / ship
            if ship.is_active() && ufo.is_active() && !bullets.is_empty() {
                // player bullet to alien or alien bullet to ship
                for b in bullets.iter_mut() {
                    if *b.get_shooter_type() == ShooterType::ALIEN {
                        if aabb(b.get_box_area(), ship.get_box_area()) {
                            if sat(b.get_tp(), ship.get_tp()) {
                                let x = b.get_position().x;
                                let y = b.get_position().y;

                                explosions.push(Explosion::new(x, y));
                                is_damaged = true;

                                ship.kill();
                                b.kill();
                                
                                explosion_sound.play();

                                break;
                            }
                        }
                    } 

                    if *b.get_shooter_type() == ShooterType::PLAYER {
                        if aabb(b.get_box_area(), ufo.get_box_area()) {
                            if sat(b.get_tp(), ufo.get_tp()) {
                                let x = b.get_position().x;
                                let y = b.get_position().y;

                                explosions.push(Explosion::new(x, y));

                                ufo.kill();
                                b.kill();

                                explosion_sound.play();

                                break;
                            }
                        }
                    }
                }
            }

            // ship / asteroids
            if ship.is_active() && !asteroids.is_empty() {
                for a in asteroids.iter_mut() {
                    if aabb(ship.get_box_area(), a.get_box_area()) {
                        if sat(ship.get_tp(), a.get_tp()) {
                            is_damaged = true;

                            let ax = a.get_position().x;
                            let ay = a.get_position().y;
                            explosions.push(Explosion::new(ax, ay));

                            let sx = ship.get_position().x;
                            let sy = ship.get_position().y;
                            explosions.push(Explosion::new(sx, sy));

                            match a.get_asteroid_type() {
                                AsteroidSize::LARGE => {
                                    gen_type =
                                        GenAsteroid::MEDIUM(a.get_position().x, a.get_position().y);
                                    gen_new_asteroids = true;
                                }
                                AsteroidSize::MEDIUM => {
                                    gen_type =
                                        GenAsteroid::SMALL(a.get_position().x, a.get_position().y);
                                    gen_new_asteroids = true;
                                }
                                _ => {}
                            }

                            ship.kill();
                            a.kill();

                            explosion_sound.play();

                            break;
                        }
                    }
                }
            }

            // asteroid / bullet
            if !bullets.is_empty() && !asteroids.is_empty() {
                for b in bullets.iter_mut() {
                    // skip bullets from ufo
                    if *b.get_shooter_type() == ShooterType::ALIEN {
                        continue;
                    }
                    for a in asteroids.iter_mut() {
                        if a.is_active() {
                            if aabb(a.get_box_area(), b.get_box_area()) {
                                if sat(a.get_tp(), b.get_tp()) {
                                    a.toggle_color(true);

                                    // init new explosions
                                    let x = b.get_position().x;
                                    let y = b.get_position().y;

                                    explosions.push(Explosion::new(x, y));
                                    match a.get_asteroid_type() {
                                        AsteroidSize::LARGE => {
                                            gen_type = GenAsteroid::MEDIUM(
                                                a.get_position().x,
                                                a.get_position().y,
                                            );
                                            gen_new_asteroids = true;
                                        }
                                        AsteroidSize::MEDIUM => {
                                            gen_type = GenAsteroid::SMALL(
                                                a.get_position().x,
                                                a.get_position().y,
                                            );
                                            gen_new_asteroids = true;
                                        }
                                        _ => {}
                                    }
                                    // remove
                                    a.kill();
                                    b.kill();

                                    explosion_sound.play();

                                    break;
                                }
                            }
                        }
                    }
                }
            }

            if gen_new_asteroids {
                match gen_type {
                    GenAsteroid::MEDIUM(x, y) => {
                        polulate_asteroids(x, y, AsteroidSize::MEDIUM, &mut asteroids);
                    }
                    GenAsteroid::SMALL(x, y) => {
                        polulate_asteroids(x, y, AsteroidSize::SMALL, &mut asteroids);
                    }
                    _ => {}
                }
                gen_new_asteroids = false;
            }

            if is_damaged {
                lives.remove_life();
                is_damaged = false;
            }

            // UPDATE ---
            // explosion.update(delta);
            if !explosions.is_empty() {
                for e in explosions.iter_mut() {
                    e.update(delta);
                }
            }

            // Ship
            ship.update(delta);

            // ship shooting
            shoot_time += delta;
            if ship.is_active() && ship.is_fireing() && shoot_time > max_shoot_time {
                let new_b = Bullet::new(
                    ship.get_position().x,
                    ship.get_position().y,
                    ship.get_angle(),
                    ShooterType::PLAYER,
                );

                bullets.push(new_b);

                fire_sound.play();

                shoot_time = 0.;
            }

            // Ufo
            ufo.update(delta);
            // ufo shooting
            if ufo.is_active() && ufo.is_shooting() && ship.is_active() {
                // get angle between ship and ufo
                let angle = v2_angle_to_point(ship.get_position(), ufo.get_position());
                let rng_fudge = random_number(-0.2, 0.2);

                let new_b = Bullet::new(
                    ufo.get_position().x,
                    ufo.get_position().y,
                    angle + rng_fudge,
                    ShooterType::ALIEN,
                );

                bullets.push(new_b);
            }

            // Bullets
            if !bullets.is_empty() {
                for bullet in bullets.iter_mut() {
                    bullet.update(delta);
                }
            }

            // Asteroids
            if !asteroids.is_empty() {
                for a in asteroids.iter_mut() {
                    a.update(delta);
                }
            }

            // Filter out inactive
            filter_out_inactive(&mut explosions);
            filter_out_inactive(&mut bullets);
            filter_out_inactive(&mut asteroids);

            lives.update();

            // RENDER ---

            window.clear(Color::BLACK);
            // -> start
            //ship
            ship.draw(&mut window);

            // ufo
            ufo.draw(&mut window);

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

            // explosions
            if !explosions.is_empty() {
                for e in explosions.iter_mut() {
                    e.draw(&mut window);
                }
            }

            // lives
            lives.draw(&mut window);
            // <- end
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
