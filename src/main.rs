// ----------
// IMPORTS
// ----------
// other files to include
mod asteroid;
mod baseobject;
mod boxshape;
mod bullet;
mod ship;

use sfml::{graphics::*, system::*, window::*};
//
use std::collections::HashMap;
use std::f32::INFINITY;
//
use rand::{thread_rng, Rng};
//
use crate::asteroid::*;
use crate::boxshape::*;
use crate::bullet::*;
use crate::ship::*;

// ----------
// CONSTS
// ----------
const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

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
fn random_angle() -> f32 {
    let mut rng = thread_rng();
    let result: f32 = rng.gen_range(0.0, 360.0);
    result
}

/// get vector normal between two points
fn get_vector_normal(p1: (f32, f32), p2: (f32, f32)) -> (f32, f32) {
    // (-y, x)
    (-(p2.1 - p1.1), p2.0 - p1.0)
}

/// get vector dot product between two points
fn get_dot_product(v1: (f32, f32), v2: (f32, f32)) -> f32 {
    let x = v1.0 * v2.0;
    let y = v1.1 * v2.1;
    let result = x + y;
    result
}

// TODO: Separated Axis Theorem
// link https://www.youtube.com/watch?v=7Ik2vowGcU0&t=414s
// problem is all objects can not have concave points
fn sat(points_1: Vec<(f32, f32)>, points_2: Vec<(f32, f32)>) -> bool {
    // check shadow overlap
    let mut p1 = &points_1;
    let mut p2 = &points_2;

    for i in 0..2 {
        if i == 1 {
            // swap
            p1 = &points_2;
            p2 = &points_1;
        }

        for x in 0..p1.len() {
            let y = (x + 1) % points_1.len();
            // norm
            let normal = get_vector_normal(p1[x], p1[y]);
            // unit vector
            // let length = (normal.0 * normal.0 + normal.1 * normal.1).sqrt();
            // normal.0 /= length;
            // normal.1 /= length;

            // shape 1
            let mut min_value1 = INFINITY;
            let mut max_value1 = -INFINITY;
            for idx in 0..p1.len() {
                let dot = get_dot_product(p1[idx], normal);
                // update min/max
                min_value1 = min_value1.min(dot);
                max_value1 = max_value1.max(dot);
            }

            // shape 2
            let mut min_value2 = INFINITY;
            let mut max_value2 = -INFINITY;
            for idx in 0..p2.len() {
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

/// main run for sfml window
fn run(width: u32, height: u32) {
    let mut window = RenderWindow::new((width, height), "space", Style::CLOSE, &Default::default());
    window.set_mouse_cursor_visible(false);
    window.set_framerate_limit(30);
    let mut is_running = true;
    let mut is_paused = false;
    let mut clock = Clock::start();

    // key maps
    let mut key_map: HashMap<&Key, bool> = HashMap::new();
    key_map.insert(&Key::W, false);
    key_map.insert(&Key::A, false);
    key_map.insert(&Key::D, false);
    key_map.insert(&Key::Space, false);

    // TODO: ship
    let center_x = width as f32 * 0.5;
    let center_y = height as f32 * 0.5;
    let mut ship = Ship::new(center_x, center_y, 0.);
    let mut ship_box = BoxShape::new(ship.get_position_x(), ship.get_position_y(), 70.0, 70.0);

    // TODO: bullets
    let mut is_shooting = false;
    let mut can_shoot_timer = 0.0;
    let max_shoot_time = 0.5;
    let mut bullets: Vec<Bullet> = Vec::new();

    // TODO: asteroid
    let mut asteroid = Asteroid::new();
    asteroid.init_large(500.0, 350.0, random_angle(), random_bool());
    let mut asteroid_box = BoxShape::new(
        asteroid.get_position_x(),
        asteroid.get_position_y(),
        150.0,
        150.0,
    );

    while window.is_open() && is_running {
        // INPUTS ---
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed => is_running = false,
                Event::KeyPressed { code, .. } => match code {
                    Key::Escape => is_running = false,
                    Key::P => is_paused = !is_paused,
                    Key::Tab =>{
                        // display box
                        ship_box.toggle_debug();
                        asteroid_box.toggle_debug();
                    }
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
                        is_shooting = true;
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
                        is_shooting = false;
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        if !is_paused {
            let delta = clock.restart().as_seconds();

            // RENDER ---
            window.clear(Color::BLACK);
            // ->
            //ship
            ship.draw(&mut window);
            ship_box.draw(&mut window);
            asteroid_box.draw(&mut window);
            // asteroid
            asteroid.draw(&mut window);
            // bullets
            if !bullets.is_empty() {
                for bullet in bullets.iter_mut() {
                    if bullet.is_active() {
                        bullet.draw(&mut window);
                    }
                }
            }
            // <-
            window.display();

            // INPUTS ---
            ship.inputs(&key_map);

            // UPDATE ---
            // collision
            // only call sat if box collision goes off
            if ship_box.check_collision(&asteroid_box) {
                asteroid_box.toggle_color(true);

                if sat(ship.get_transform_points(), asteroid.get_transform_points()) {
                    println!("hit asteroid");
                }
            } else {
                asteroid_box.toggle_color(false);
            }

            // ship
            can_shoot_timer += delta;
            if is_shooting && can_shoot_timer > max_shoot_time {
                let px = ship.get_position_x();
                let py = ship.get_position_y();
                let ang = ship.get_angle();

                let mut new_b = Bullet::new();
                new_b.init((px, py), ang);
                bullets.push(new_b);

                can_shoot_timer = 0.0;
            }

            ship.update(delta);
            ship_box.set_position(ship.get_position());
            ship_box.update(delta);
            
            // asteroid
            asteroid.update(delta);
            asteroid_box.set_position(asteroid.get_position());
            asteroid_box.update(delta);

            // bullets
            if !bullets.is_empty() {
                for bullet in bullets.iter_mut() {
                    if bullet.is_active() {
                        bullet.update(delta);
                    }
                }
            }
            if !bullets.is_empty() {
                bullets.retain(|x| x.is_active());
            }
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
