use plotters::prelude::*;

pub fn draw_line(title: &str, data: &Vec<f64>) {
    let min_element = data.iter().cloned().reduce(f64::min).unwrap();
    let max_element = data.iter().cloned().reduce(f64::max).unwrap();

    let path = format!("assets/img/{}", title.replace(' ', "_")).to_ascii_lowercase();

    let root_drawing_area = BitMapBackend::new(&path, (1024, 768)).into_drawing_area();

    root_drawing_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_drawing_area)
        .margin(15)
        .caption("Probability Function", ("Arial", 30))
        .set_all_label_area_size(50)
        .build_cartesian_2d(0..data.len(), min_element..max_element)
        .unwrap();

    // Axis
    ctx.configure_mesh()
        .x_desc("Ratio between Items and Total Cells")
        .y_desc("Probability")
        .axis_desc_style(("Jetbrains Mono", 20))
        .draw()
        .unwrap();

    // Line Plot
    let style = ShapeStyle {
        color: BLUE.into(),
        filled: false,
        stroke_width: 5,
    };
    ctx.draw_series(LineSeries::new(data.iter().cloned().enumerate(), style))
        .unwrap()
        .label(title)
        .legend(|(x, y)| {
            PathElement::new(
                vec![(x, y), (x + 20, y)],
                ShapeStyle {
                    color: RED.into(),
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
