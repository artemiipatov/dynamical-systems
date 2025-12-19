use num_complex::Complex;
use plotters::prelude::*;
use rand::{Rng, rng};
use std::error::Error;

type Point = (f64, f64);

pub fn solve() -> Result<(), Box<dyn Error>> {
    let c = Complex::new(-0.123, 0.745);
    let iterations = 200_000;
    let mut z = Complex::new(0.0, 0.0);
    let mut points: Vec<Point> = Vec::with_capacity(iterations);

    let mut rng = rng();
    for i in 0..iterations {
        let sign = if rng.random::<bool>() { 1.0 } else { -1.0 };
        // z = ±√(z - c)
        z = (z - c).sqrt() * sign;

        if i > 50 {
            points.push((z.re, z.im));
        }
    }

    let path = "result2.png";
    let root = BitMapBackend::new(path, (1000, 1000)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(format!("Inverse Iterations Julia Set (c = {} + {}i)", c.re, c.im), ("sans-serif", 30))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(-1.5..1.5, -1.5..1.5)?;

    // Рисуем сетку
    chart.configure_mesh()
        .x_labels(10)
        .y_labels(10)
        .light_line_style(&TRANSPARENT)
        .draw()?;

    chart.draw_series(
        points.iter().map(|(x, y)| {
            Circle::new((*x, *y), 1, BLUE.mix(0.3).filled())
        })
    )?;

    root.present()?;
    println!("График сохранен в: {}", path);

    Ok(())
}