// USE
use sfml::{graphics::*};
//
use std::f32::consts::PI;
//
use crate::isactive::IsActive;
use crate::particle::Particle;
use crate::globals::random_number;


pub struct Explosion {
    // position: Vector2f,
    // radius: f32,
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
    pub fn new(x:f32, y:f32) -> Self {
        let mut  p = [
            Particle::new(),
            Particle::new(),
            Particle::new(),
            Particle::new(),
            Particle::new(),
            Particle::new(),
            Particle::new(),
            Particle::new(),
        ];

        let r = 3.;

        let n = p.len() as f32;
        let slice = (PI * 2.) / n;
        for (idx, particle) in p.iter_mut().enumerate() {
            let rng_ang: f32 = random_number(1., 3.14);

            let ang = (idx as f32) + rng_ang * slice;

            // println!("ang: {}", ang);

            let px = x + ang.cos() * r;
            let py = y + ang.sin() * r;

            // rng life and speed
            let life: f32 = random_number(1.5, 2.5);
            let acc: f32 = random_number(100., 200.);

            
            particle.init(px, py, acc, ang, life);
        }

        Self {
            // position: Vector2f::default(),
            tally: 0,
            // radius: 3.,
            particles: p,
            is_active: true,
        }
    }

    // pub fn init(&mut self, x: f32, y: f32) {
    //     //let mut angle = 0.0;
    //     self.position.x = x;
    //     self.position.y = y;
    //     self.is_active = true;
    //     let n = self.particles.len() as f32;
    //     let slice = (PI * 2.) / n;

    //     for (idx, particle) in self.particles.iter_mut().enumerate() {
    //         let rng_ang: f32 = random_number(1., 3.14);

    //         let ang = (idx as f32) + rng_ang * slice;

    //         // println!("ang: {}", ang);

    //         let px = self.position.x + ang.cos() * self.radius;
    //         let py = self.position.y + ang.sin() * self.radius;

    //         // rng life and speed
    //         let life: f32 = random_number(1.5, 2.5);
    //         let acc: f32 = random_number(100., 200.);

    //         particle.init(px, py, acc, ang, life);
    //     }
    // }

    pub fn draw(&mut self, window: &mut RenderWindow) {
        if self.is_active {
            for p in self.particles.iter_mut() {
                p.draw(window);
            }
        }
    }

    pub fn update(&mut self, delta: f32) {
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
