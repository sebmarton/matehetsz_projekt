use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::view::ContinuousView;
use plotlib::style::{PointMarker, PointStyle, LineStyle};

pub fn plot(data: Vec<(f64, f64)>, exact: Vec<(f64, f64)>) -> Result<(), Box<dyn std::error::Error>> {

    let s1 = Plot::new(exact).line_style(
      LineStyle::new()
      .colour("#000000")
    );

    let s2 = Plot::new(data).point_style(
      PointStyle::new()
      .marker(PointMarker::Circle)
      .colour("#0000FF")
    );

    let v = ContinuousView::new()
    .add(s1)
    .add(s2)
    .x_range(0f64, 0.5f64)
    .y_range(0f64, 1f64)
    .x_label("t [s]")
    .y_label("y(t) [m]");

    //error propagation unwrap() helyett
    Page::single(&v).save("diffplot.svg")?;

    Ok(())
}

pub fn exact() -> Vec<(f64, f64)> {

  let mut v: Vec<(f64, f64)> = Vec::new();

    for i in 0..10000+1 {
    let j = i as f64;
    let s = 1. - 1f64/2f64 * 9.81 * (j / 1000f64) * (j / 1000f64);

    if s <= 0. {
      break;
    }

    v.push((j / 1000f64, s));
  }

  v

}