use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::view::ContinuousView;
use plotlib::style::{LineStyle};

pub fn plot(x: Vec<(f64, f64)>, y: Vec<(f64, f64)>) -> Result<(), Box<dyn std::error::Error>> {
  
    let s1 = Plot::new(x).line_style(
      LineStyle::new()
      .colour("#0000FF")
    );

    let s2 = Plot::new(y).line_style(
        LineStyle::new()
        .colour("#FF0000")
      );

    let v = ContinuousView::new()
    .add(s1)
    .add(s2)
    .x_range(0f64, 5f64)
    .y_range(-1f64, 1f64)
    .x_label("t [s]")
    .y_label("x(t), y(t) [m]");

    //error propagation unwrap() helyett
    Page::single(&v).save("diffplot.svg")?;

    Ok(())
}
