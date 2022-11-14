use plotters::prelude::*;

#[allow(unused_imports)]
use crate::consts::MAX_PLOT_POINTS;

pub fn draw_line(title: &str, data: &Vec<f64>) {
    // let chunk_size = (data.len() / MAX_PLOT_POINTS).max(1);
    // let data: Vec<_> = data
    //     .chunks(chunk_size)
    //     .map(|c| *c.last().unwrap())
    //     .collect();

    // let min_element = data.iter().cloned().reduce(f64::min).unwrap();
    let max_element = data.iter().cloned().reduce(f64::max).unwrap();

    let path = format!("assets/img/{}.png", title.replace(' ', "_")).to_ascii_lowercase();

    let root_drawing_area = BitMapBackend::new(&path, (1024, 768)).into_drawing_area();

    root_drawing_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_drawing_area)
        .margin(15)
        .caption(title, ("Arial", 30))
        .set_label_area_size(LabelAreaPosition::Left, 50)
        .set_label_area_size(LabelAreaPosition::Bottom, 50)
        .build_cartesian_2d(0..data.len(), 0.0..max_element)
        .unwrap();

    // Axis
    ctx.configure_mesh()
        .x_desc("Iteration")
        .y_desc(title)
        .axis_desc_style(("Jetbrains Mono", 20))
        .draw()
        .unwrap();

    // Line Plot
    let style = ShapeStyle {
        color: BLUE.into(),
        filled: false,
        stroke_width: 1,
    };
    let iter = data.iter().cloned().enumerate();
    ctx.draw_series(LineSeries::new(iter, style))
        .unwrap()
        .label(title)
        .legend(|(x, y)| {
            PathElement::new(
                vec![(x, y), (x + 20, y)],
                ShapeStyle {
                    color: BLUE.into(),
                    filled: false,
                    stroke_width: 5,
                },
            )
        });

    // Label
    ctx.configure_series_labels()
        .border_style(&BLACK)
        .background_style(&WHITE.mix(0.8))
        .label_font(("Jetbrains Mono", 20))
        .draw()
        .unwrap();
}
