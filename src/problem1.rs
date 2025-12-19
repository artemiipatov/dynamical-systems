use plotters::prelude::*;

type Point = (f64, f64);
type PolygonalChain = Vec<Point>;

#[derive(Clone)]
struct SegmentIterationConfiguration {
    domain_min: Point,
    domain_max: Point,
    niters: usize,
    fragm_dist: f64,
    chain_base: PolygonalChain,
}

fn distance(a: Point, b: Point) -> f64 {
    ((a.0 - b.0).powi(2) + (a.1 - b.1).powi(2)).sqrt()
}

fn inside(a: Point, min: Point, max: Point) -> bool {
    a.0 >= min.0 && a.0 <= max.0 && a.1 >= min.1 && a.1 <= max.1
}

fn midpoint(a: Point, b: Point) -> Point {
    ((a.0 + b.0) / 2.0, (a.1 + b.1) / 2.0)
}

fn neat_uniq(mut v: Vec<Point>) -> Vec<Point> {
    v.dedup();
    v
}

fn system_henon(p: Point, params: &[f64]) -> Point {
    let (x, y) = p;
    let s1 = params[0];
    let s2 = params[1];
    (1.0 - s1 * x.powi(2) + y, s2 * x)
}

fn fragmentize_recursive<F>(
    p1: Point,
    p2: Point,
    system: &F,
    config: &SegmentIterationConfiguration,
    result: &mut Vec<Point>,
) where
    F: Fn(Point) -> Point,
{
    let fp1 = system(p1);
    let fp2 = system(p2);

    let in_d1 = inside(fp1, config.domain_min, config.domain_max);
    let in_d2 = inside(fp2, config.domain_min, config.domain_max);

    if !in_d1 || !in_d2 {
        return;
    }

    if distance(fp1, fp2) < config.fragm_dist {
        result.push(p1);
        result.push(p2);
    } else {
        let mid = midpoint(p1, p2);
        fragmentize_recursive(p1, mid, system, config, result);
        fragmentize_recursive(mid, p2, system, config, result);
    }
}

fn fragmentize<F>(
    chain: &PolygonalChain,
    system: &F,
    config: &SegmentIterationConfiguration,
) -> PolygonalChain
where
    F: Fn(Point) -> Point,
{
    let mut fragmentized = Vec::new();
    
    for i in 0..chain.len() - 1 {
        fragmentize_recursive(
            chain[i], 
            chain[i + 1], 
            system, 
            config, 
            &mut fragmentized
        );
    }
    
    neat_uniq(fragmentized)
}

fn run_segment_iteration<F>(
    system: F,
    config: SegmentIterationConfiguration,
) -> PolygonalChain
where
    F: Fn(Point) -> Point,
{
    let mut chain = config.chain_base.clone();
    
    for i in 0..config.niters {
        println!("Итерация {}/{} (точек: {})", i + 1, config.niters, chain.len());
        // 1. Фрагментация текущей цепи
        chain = fragmentize(&chain, &system, &config);
        // 2. Отображение цепи, переход к следующему состоянию
        chain = chain.into_iter().map(&system).collect();
    }
    
    chain
}

fn create_config(
    d0: Point,
    w: f64,
    h_dim: f64,
    s: f64,
    h_param: f64,
    n: usize,
) -> SegmentIterationConfiguration {
    let half_w = w / 2.0;
    let half_h = h_dim / 2.0;
    
    let domain_min = (d0.0 - half_w, d0.1 - half_h);
    let domain_max = (d0.0 + half_w, d0.1 + half_h);

    let signs = vec![(-1.0, -1.0), (-1.0, 1.0), (1.0, 1.0), (1.0, -1.0), (-1.0, -1.0)];
    let chain_base = signs
        .iter()
        .map(|(sx, sy)| (d0.0 + sx * s / 2.0, d0.1 + sy * s / 2.0))
        .collect();

    SegmentIterationConfiguration {
        domain_min,
        domain_max,
        niters: n,
        fragm_dist: h_param,
        chain_base,
    }
}

fn draw_plot(chain: &PolygonalChain, filename: &str, title: &str) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(filename, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let min_x = chain.iter().map(|p| p.0).fold(f64::INFINITY, f64::min);
    let max_x = chain.iter().map(|p| p.0).fold(f64::NEG_INFINITY, f64::max);
    let min_y = chain.iter().map(|p| p.1).fold(f64::INFINITY, f64::min);
    let max_y = chain.iter().map(|p| p.1).fold(f64::NEG_INFINITY, f64::max);

    let margin_x = (max_x - min_x) * 0.1;
    let margin_y = (max_y - min_y) * 0.1;

    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("sans-serif", 30).into_font())
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(
            (min_x - margin_x)..(max_x + margin_x), 
            (min_y - margin_y)..(max_y + margin_y)
        )?;

    chart.configure_mesh().draw()?;

    chart.draw_series(
        chain.iter().map(|(x, y)| Circle::new((*x, *y), 1, BLUE.filled())),
    )?;

    root.present()?;
    println!("График сохранен в файл: {}", filename);
    Ok(())
}

pub fn solve() -> Result<(), Box<dyn std::error::Error>> {
    println!("Henon");
    let henon_params = vec![1.4, 0.3]; // s1, s2
    let henon_conf = create_config(
        (0.0, 0.0),      // d0
        4.0, 4.0,  // W, H
        0.75,             // S
        0.01,       // h
        20                // n iterations
    );

    let result_henon = run_segment_iteration(
        |p| system_henon(p, &henon_params), 
        henon_conf
    );
    draw_plot(&result_henon, "result1.png", "Henon Invariant Set")?;

    Ok(())
}
