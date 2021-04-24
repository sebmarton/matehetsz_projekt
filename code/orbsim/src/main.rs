mod planets;
mod parser;
mod plot;

use planets::*;

#[cfg(test)]
mod tests;

const MAXTIME: f64 = 10000000.;
const SI_SCALING_FACTOR: f64 = 150000000. / 200.;

fn main() {

    let mut earth_pos: Vec<(f64, f64)> = Vec::new();

    // Adatok deszerializálása init.json-ból
    let mut system = parser::parse_init("init.json")
        .expect("Error parsing initial parameters");

    // Piston window inicializálása
    let mut window: PistonWindow = WindowSettings::new("orbsim v0.2.0", [WIDTH, HEIGHT])
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Time-step és render
    while let Some(e) = window.next() {
        if system.time <= MAXTIME {
        window.draw_2d(&e, |c, g, _| {
            system.render(c, g);
            system.step_time();
            earth_pos.push(((system.earth.pos.x - system.sun.pos.x) * SI_SCALING_FACTOR / 1000000., (system.earth.pos.y - system.sun.pos.y) * SI_SCALING_FACTOR / 1000000.));
        });
        }
    }

    plot::plot(earth_pos).unwrap();

}
