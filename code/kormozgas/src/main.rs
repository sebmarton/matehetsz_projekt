mod plot;

struct Allapot {
    x: f32, // X koordinata
    y: f32, // Y koordinata
    vx: f32, // V sebesseg X iranyu komponense
    vy: f32, // V sebesseg Y iranyu komponense
    t: f32 // Eltelt ido
}

// konstruktor method
impl Allapot {
    fn new(x: f32, y: f32, vx: f32, vy: f32, t: f32) -> Self {
        Self {x, y, vx, vy, t}
    }
}

// Display trait impementációja könnyebb kiírásért
impl std::fmt::Display for Allapot {

    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "t = {} s || x = {} m, vx = {} m/s, y = {} m, vy = {} m/s", self.t, self.x, self.vx, self.y, self.vy)
    }

}

fn kormozgas(x0: f32, y0: f32, v0x: f32, v0y: f32, alpha: f32, dt: f32, m: f32, n: usize) -> Vec<Allapot> {

    // y(t) és y(t) értékek tömbje
    let mut allapotok: Vec<Allapot> = Vec::new();

    // kezdeti értékek 
    allapotok.push(Allapot::new(x0, y0, v0x, v0y, 0.0));

    for t in 1 .. n+1 {

        //vx(t + dt) = vx(t) - alpha * dt / m 
        let vx = -alpha * allapotok[t-1].x / m * dt + allapotok[t-1].vx;

        //x(t + dt) = x(t) + vx(t) * dt
        let x = vx * dt + allapotok[t-1].x;

        //vy(t + dt) = vy(t) - alpha * dt / m 
        let vy = -alpha * allapotok[t-1].y / m * dt + allapotok[t-1].vy;

        //x(t + dt) = x(t) + vx(t) * dt
        let y = vy * dt + allapotok[t-1].y;

        // állapotok a tömmbe való helyezése
        allapotok.push(Allapot::new(x, y, vx ,vy, allapotok[t-1].t + dt));

    }

    allapotok
}

fn main() {

    // Kezdeti paraméterek, x0 = 1, y0 = 0, v0x = 0, v0y = 1
    let allapotok = kormozgas(1.0, 0.0, 0.0, 1.0, 7.0, 0.01, 1., 500);

    let mut x: Vec<(f64, f64)> = Vec::new();
    let mut y: Vec<(f64, f64)> = Vec::new();

    for a in allapotok {

        //Értékek kinyomtatása
        println!("{}", a);
        
        x.push((a.t as f64, a.x as f64));
        y.push((a.t as f64, a.y as f64));
    }

    plot::plot(x, y).unwrap();

}
