// Ez a fájl szimulációhoz szükséges adatstruktúrákat és methodokat tartalmazza.

extern crate piston_window;

pub use piston_window::*;

use super::parser::Deserialize;

pub const WIDTH: f64 = 1024.;
pub const HEIGHT: f64 = 768.;

const DISPLAY_SCALE: f64 = 100000.;

mod vectordspl;
use vectordspl::draw_vector;

#[derive(Deserialize)]
pub struct PlanetarySystem {

    pub sun: Planet,
    pub earth: Planet,
    pub grav_coefficent: f64,
    pub time: f64,
    pub timescale: f64,

}

impl PlanetarySystem {

    pub fn new(sun: Planet, earth: Planet, grav_coefficent: f64, timescale: f64) -> Self {
        Self {sun, earth, grav_coefficent, time: 0., timescale}
    }
    
    // Kiszámolja a Föld helyét és sebességét a Naphoz függően
    // A Napra nem hat gravitáció az egyszerűség kedvéért
    pub fn grav_accel(&self) -> (f64, f64) {

        (
        self.grav_coefficent * self.sun.mass * (self.sun.pos.x - self.earth.pos.x) / self.earth.distance(&self.sun).powf(3.),
        self.grav_coefficent * self.sun.mass * (self.sun.pos.y - self.earth.pos.y) / self.earth.distance(&self.sun).powf(3.)
        )

    }

    // Előrelépteti az időt, és ahhoz függően mozgatja a Földet.
    pub fn step_time(&mut self) {

        let (gx, gy) = self.grav_accel();

        self.earth.vel.x += gx * self.timescale;
        self.earth.vel.y += gy * self.timescale;

        self.earth.pos.x += self.earth.vel.x * self.timescale;
        self.earth.pos.y += self.earth.vel.y * self.timescale;

        self.sun.pos.x += self.sun.vel.x * self.timescale;
        self.sun.pos.y += self.sun.vel.y * self.timescale;
        
        if ((self.sun.pos.x - self.earth.pos.x).powf(2.) + (self.sun.pos.y - self.earth.pos.y).powf(2.)).sqrt() <= self.sun.radius + self.earth.radius {
            panic!("Collision")
        }
        

        self.time += self.timescale;

    }

    // Kirendereli a bolygókat a képernyőre.
    pub fn render(&self, ctx: Context, gfx: &mut G2d) {

        clear([1.; 4], gfx);

        ellipse([1., 0.64, 0., 1.],
            [self.sun.pos.x - self.sun.radius, HEIGHT - self.sun.pos.y - self.sun.radius, self.sun.radius * 2.0, self.sun.radius * 2.0],
            ctx.transform,
            gfx);

        ellipse([0., 0., 1., 1.],
            [self.earth.pos.x - self.earth.radius, HEIGHT - self.earth.pos.y - self.earth.radius, self.earth.radius * 2.0, self.earth.radius * 2.0],
            ctx.transform,
            gfx);

        draw_vector([0., 0., 0., 0.5],
            1.,
            [self.sun.pos.x, HEIGHT - self.sun.pos.y, self.earth.pos.x, HEIGHT - self.earth.pos.y],
            ctx,
            gfx);

        draw_vector([0., 1., 0., 1.,],
            0.7,
            [self.earth.pos.x, HEIGHT - self.earth.pos.y, self.earth.pos.x + self.earth.vel.x * DISPLAY_SCALE, HEIGHT - (self.earth.pos.y + self.earth.vel.y * DISPLAY_SCALE)],
            ctx,
            gfx);

        let (gx, gy) = self.grav_accel();

        draw_vector([1., 0., 0., 1.,],
            0.7,
            [self.earth.pos.x, HEIGHT - self.earth.pos.y, self.earth.pos.x + gx * DISPLAY_SCALE.powf(2.12), HEIGHT - (self.earth.pos.y + gy * DISPLAY_SCALE.powf(2.12))],
            ctx,
            gfx); 
        

    }

}

// Vektor adatstruktúra
#[derive(Deserialize)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
}

impl Vector {

    // konstruktor
    fn new(x: f64, y: f64) -> Self {
        Self {x, y}
    }

    // két vektor távolságát adja meg
    fn diff(&self, other: &Self) -> f64 {
        ((self.x - other.x).powf(2.) + (self.y - other.y).powf(2.)).sqrt() 
    }

}

// Bolygó adatsruktúra
#[derive(Deserialize)]
pub struct Planet {

    pub mass: f64,
    pub radius: f64, 
    pub pos: Vector,
    pub vel: Vector,

}

impl Planet {

    // konstruktor
    pub fn new(mass: f64, radius: f64, x: f64, vx: f64, y: f64, vy: f64) -> Self {
        Self {mass, radius, pos: Vector::new(x, y), vel: Vector::new(vx, vy)}
    }

    // Két bolygó távolsága
    pub fn distance(&self, other: &Self) -> f64 {
        self.pos.diff(&other.pos)
    }

}