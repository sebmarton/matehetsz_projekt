use super::piston_window::*;

const ARROW_SCALING: f64 = 4.;

//Vektornyilak kirajzolása, csúnya lett, de nincs rá szebb mód
pub fn draw_vector(color: [f32; 4], width: f64, coords: [f64; 4],  ctx: Context, gfx: &mut G2d) {

    let offset = 2f64.sqrt() / 2.; // sin(45) és cos(45)

    let dx = coords[2] - coords[0];
    let dy = coords[3] - coords[1];

    let length = (dx.powf(2.) + dy.powf(2.)).sqrt(); 

    let sin = dy / length;
    let cos = dx / length;

    let sindiff1 = sin * offset + cos * offset; //sin(szög + offset)
    let sindiff2 = sin * offset - cos * offset; //sin(szög - offset) 

    let cosdiff1 = cos * offset - sin * offset; //cos(szög + offset)
    let cosdiff2 = cos * offset + sin * offset; //cos(szög - offset)

    line(color, width, coords, ctx.transform, gfx);

    line(color,
        width,
        [coords[2], coords[3], coords[2] - length / ARROW_SCALING * sindiff1, coords[3] + length / ARROW_SCALING * cosdiff1],
        ctx.transform,
        gfx
    );

    line(color,
        width,
        [coords[2], coords[3], coords[2] + length / ARROW_SCALING * sindiff2, coords[3] - length / ARROW_SCALING * cosdiff2],
        ctx.transform,
        gfx
    );
}