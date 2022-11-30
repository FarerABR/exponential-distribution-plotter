use plotters::prelude::*;
use std::{env, f32::consts};

/**
## config for IO argumants

```
param: [String]

returns: lam:i32, perc:i32, dom:i32
```
*/
fn config(args: &[String]) -> (i32, i32, i32) {
    if args.len() < 4 {
        panic!("Not enough argumants");
    }

    (
        args[1].parse::<i32>().unwrap(),
        args[2].parse::<i32>().unwrap(),
        args[3].parse::<i32>().unwrap(),
    )
}

fn draw(lam: i32, perc: f32, dom: f32) {
    let mut y: Vec<(f32, f32)> = Vec::new();

    for i in 0..=(dom * perc) as i32 {
        y.push((i as f32 / perc, exp(i as f32 / perc, lam)));
    }

    let drawing_area = BitMapBackend::new("./figure.png", (1920, 1080)).into_drawing_area();
    drawing_area.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&drawing_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption(
            format!("Exponential distibution - exp({})", lam),
            ("calibri", 40),
        )
        .build_cartesian_2d(0.0..dom + 0.1, -0.001f32..y[0].1 + 0.01)
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    chart
        .draw_series(
            y.iter()
                .map(|&point| Circle::new(point, 2, Into::<ShapeStyle>::into(&RED).filled())),
        )
        .unwrap();
}

/**
## The norm distribution function
```
param: x:f32, lam:i32
```
*/
fn exp(x: f32, lam: i32) -> f32 {
    lam as f32 * consts::E.powf(-(lam as f32 * x))
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let (lam, perc, dom) = config(&args);
    // exp(1.0, lam);
    draw(lam, perc as f32, dom as f32);
}
