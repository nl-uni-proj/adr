use plotters::prelude::*;

const Y_NAME: &str = "DAY";
const X_NAME: &str = "SALES";

pub fn render(data: &[f64], dir: Option<&str>, name: &str, caption: &str) {
    let mut path = std::env::current_dir().expect("cwd");
    path.push("adr_graphs");
    if !path.exists() {
        std::fs::create_dir(&path).expect("dir created");
    }
    if let Some(dir) = dir {
        path.push(dir);
        if !path.exists() {
            std::fs::create_dir(&path).expect("dir created");
        }
    }
    path.push(format!("{}.png", name));

    let root = BitMapBackend::new(&path, (1600, 900)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let ceiling = (data.iter().copied().map(|v| v as i32).max().unwrap() as f32 * 1.1) as i32;

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(50)
        .y_label_area_size(50)
        .margin(10)
        .caption(caption, ("sans-serif", 36))
        .build_cartesian_2d((0..(data.len() - 1)).into_segmented(), 0..ceiling)
        .unwrap();

    chart
        .configure_mesh()
        .disable_x_mesh()
        .bold_line_style(WHITE.mix(0.3))
        .y_desc(X_NAME)
        .x_desc(Y_NAME)
        .axis_desc_style(("sans-serif", 15))
        .draw()
        .unwrap();

    chart
        .draw_series(
            AreaSeries::new(
                (0..data.len()).map(|x| (SegmentValue::Exact(x), data[x] as i32)),
                0,
                RED.mix(0.2),
            )
            .border_style(RED),
        )
        .unwrap();

    root.present().expect("unable to write output graph");
    println!("graph has been saved to `{}`", path.to_string_lossy());
}
