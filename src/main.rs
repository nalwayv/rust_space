// ----------
// IMPORTS
// ----------
// other files to include
mod asteroid;
mod baseobject;
mod bullet;
mod globals;
mod ship;

use crate::asteroid::*;
use crate::bullet::*;
use crate::globals::*;
use crate::ship::*;
use rand::{thread_rng, Rng};
use sfml::{graphics::*, system::*, window::*};
use std::collections::HashMap;
use std::f32::{consts::PI, INFINITY};

// ----------
// FUNCS
// ----------

// fn distance(start: Vector2f, end:Vector2f, min_dis:f32) -> bool{
//     let c = end-start;
//     let x = (c.x*c.x + c.y*c.y).sqrt();

//     if x < min_dis{
//         return true;
//     }
//     false
// }

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

// TODO: Separated Axis Theorem
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


fn filter_bullets(bullets: &mut Vec<Bullet>) {
    if !bullets.is_empty() {
        bullets.retain(|x| x.is_active());
    }
}

fn populate_key_map(key_map: &mut HashMap<&Key, bool>) {
    key_map.insert(&Key::W, false);
    key_map.insert(&Key::A, false);
    key_map.insert(&Key::D, false);
    key_map.insert(&Key::Space, false);
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

    // TODO: ship
    let center_x = width as f32 * 0.5;
    let center_y = height as f32 * 0.5;
    let mut ship = Ship::new(center_x, center_y, 0.);

    // TODO: bullets
    let mut shoot_time = 0.0;
    let max_shoot_time = 0.5;
    let mut bullets: Vec<Bullet> = Vec::new();


    // TODO: asteroid
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

    let mut asteroids = vec![asteroid1, asteroid2, asteroid3];

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

            // UPDATE ---

            ship.inputs(&key_map);
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

            filter_bullets(&mut bullets);

            if !asteroids.is_empty() {
                for a in asteroids.iter_mut() {
                    a.update(delta);
                }
            }


            // collision
            // only call sat if box collision goes off
            // ship asteroid
            for a in asteroids.iter_mut() {

                if sat(ship.get_tp(), a.get_tp()) {
                    ship.toggle_color(true);
                    a.toggle_color(true);
                } else {
                    ship.toggle_color(false);
                    a.toggle_color(false);
                }

            }
            // // asteroid bullet
            for b in bullets.iter_mut() {
                for a in asteroids.iter_mut() {
                    if sat(a.get_tp(), b.get_tp()) {
                        a.toggle_color(true);
                        b.toggle_color(true);
                    } else {
                        a.toggle_color(false);
                        b.toggle_color(false);
                    }
                }
            }

            // RENDER ---

            window.clear(Color::BLACK);
            // ->
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
