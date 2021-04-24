pub mod plot;

fn main() {

    // Kezdeti paraméterek
    let s0: f64 = 1.0; // kezdeti pozíció
    let v0: f64 = 0.; // kezdeti sebesség
    let g: f64 = 9.8; // grav. gyorsulás
    let dt: f64 = 0.02; // Időlépték
    let dur_t: f64 = 5.; // Időtartam

    //y(t) értékek tömbje, y(0) = 1 m, t = 0 s
    let mut y: Vec<(f64, f64)> = vec![(0., s0)];

    //t = t + dt
    let mut t: f64 = dt;

    // y(dt) = s0 + v0 * dt, kezdeti pozíció approximációja v0 ismeretével
    y.push((t, v0 * dt + s0));

    // Differenciaegyenlet implementációja
    for i in 1..(dur_t / dt) as usize {
        t += dt;

        let s = 2. * y[i].1 - y[i - 1].1 - g * dt.powf(2.);

        // Ha y = 0, ne szimuláljon tovább
        if s <= 0. {
            break;
        } 
        // y(t + dt) = 2y(t) - y(t - dt) - g*dt^2
        y.push((t, s));
        if y[i].1 <= 0. {
            break;
        } 
    }

    // graph plot
    plot::plot(y, plot::exact()).unwrap();

}
