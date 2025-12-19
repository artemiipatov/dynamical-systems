use num_complex::Complex;
use plotters::prelude::*;
use std::f64::consts::PI;

pub fn solve() -> Result<(), Box<dyn std::error::Error>> {
    // alpha = 1/20, lambda = e^(i * 2pi/20)
    let alpha = 1.0 / 20.0;
    let lambda = Complex::from_polar(1.0, 2.0 * PI * alpha);

    println!("Lambda: {:.4}", lambda);

    // 2. Аналитический расчет точек

    // Неподвижные точки: z1 = 0, z2 = 1 - lambda
    let fixed_point_1 = Complex::new(0.0, 0.0);
    let fixed_point_2 = Complex::new(1.0, 0.0) - lambda;

    // Цикл периода 2: z^2 + (lambda+1)z + (lambda+1) = 0
    // D = b^2 - 4ac = (lambda+1)^2 - 4(lambda+1)
    let b = lambda + 1.0;
    let c = lambda + 1.0;
    let discriminant = b.powi(2) - 4.0 * c;
    let cycle_2_pt1 = (-b + discriminant.sqrt()) / 2.0;
    let cycle_2_pt2 = (-b - discriminant.sqrt()) / 2.0;

    println!("Fixed Point 1: {:.4}", fixed_point_1);
    println!("Fixed Point 2: {:.4}", fixed_point_2);
    println!("Cycle 2 Point A: {:.4}", cycle_2_pt1);
    println!("Cycle 2 Point B: {:.4}", cycle_2_pt2);

    // 3. Построение инвариантного множества (Метод обратных итераций)
    // w = z^2 + lambda*z  =>  z^2 + lambda*z - w = 0
    // z = (-lambda +/- sqrt(lambda^2 + 4w)) / 2

    let iterations = 500_000; // Количество итераций
    let mut w = Complex::new(0.1, 0.1); // Начальная точка для итераций
    let mut cloud_points = Vec::with_capacity(iterations);

    // Предварительно вычисляем lambda^2, чтобы не считать в цикле
    let lambda_sq = lambda.powi(2);

    for i in 0..iterations {
    // Решаем z^2 + lambda*z - w = 0 относительно z
        let disc: Complex<f64> = lambda_sq + 4.0 * w;
        let root: Complex<f64> = disc.sqrt();
        
        // Случайный выбор ветви
        if rand::random::<bool>() {
            w = (-lambda + root) / 2.0;
        } else {
            w = (-lambda - root) / 2.0;
        }

        // Сохраняем точки, пропустив первые 1000
        if i > 1000 {
            cloud_points.push((w.re, w.im));
        }
    }

    // 4. Отрисовка
    let root = BitMapBackend::new("result3.png", (1024, 1024)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
    .caption("Invariant Set, Fixed Points & 2-Cycle", ("sans-serif", 30))
    .margin(20)
    .x_label_area_size(40)
    .y_label_area_size(40)
    .build_cartesian_2d(-2.0..1.5, -1.8..1.8)?; // Чуть расширил границы

    chart.configure_mesh().draw()?;

    // Слой 1: Инвариантное множество (Синее облако)
    chart.draw_series(
    cloud_points.iter().map(|(x, y)| Circle::new((*x, *y), 1, BLUE.mix(0.15).filled()))
    )?;

    // Слой 2: Неподвижные точки (Красные большие точки)
    chart.draw_series(vec![
    Circle::new((fixed_point_1.re, fixed_point_1.im), 6, RED.filled()),
    Circle::new((fixed_point_2.re, fixed_point_2.im), 6, RED.filled()),
    ])?
    .label("Fixed Points")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED));

    // Слой 3: Точки цикла периода 2 (Зеленые большие точки)
    chart.draw_series(vec![
    Circle::new((cycle_2_pt1.re, cycle_2_pt1.im), 6, GREEN.filled()),
    Circle::new((cycle_2_pt2.re, cycle_2_pt2.im), 6, GREEN.filled()),
    ])?
    .label("Cycle Period 2")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], GREEN));

    chart.configure_series_labels()
    .background_style(&WHITE.mix(0.8))
    .border_style(&BLACK)
    .draw()?;

    root.present()?;
    println!("График сохранен в result3.png");

    Ok(())
}