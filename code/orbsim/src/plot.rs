extern crate plotlib;

use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::view::ContinuousView;
use plotlib::style::{PointMarker, PointStyle, LineStyle};

const ASPECT_RATIO: f64 = 120. / 85.;

pub fn plot(data: Vec<(f64, f64)>) -> Result<(), Box<dyn std::error::Error>> {

    let sun: Vec<(f64, f64)> = vec!((0., 0.));

    let s1 = Plot::new(data).line_style(
      LineStyle::new()
      .colour("#000000")
    );

    let s2 = Plot::new(sun).point_style(
      PointStyle::new()
      .marker(PointMarker::Circle)
      .colour("#FFA500")
    );

    let v = ContinuousView::new()
    .add(s1)
    .add(s2)
    .x_range(-400f64, 400f64 * ASPECT_RATIO.powf(2.))
    .y_range(-400f64, 400f64)
    .x_label("x pozíció (millió km)")
    .y_label("y pozíció (millió km)");

    //error propagation unwrap() helyett
    Page::single(&v).save("diffplot.svg")?;

    Ok(())
}
