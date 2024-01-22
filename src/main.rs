use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let oakland_values = vec![75, 50, 40, 15, 125, 15];
    draw_chart_helper("Downtown OakLand - Water", oakland_values)?;

    let san_francisco_bay = vec![175, 150, 155, 110, 140, 120, 120];
    draw_chart_helper("San Francisco Bay Rock", san_francisco_bay)?;

    let san_francisco_middle = vec![70, 60, 50, 80, 80, 140, 40];
    draw_chart_helper("Middle of San Francisco - Tree", san_francisco_middle)?;

    let walnut_creek = vec![120, 100, 120, 50, 100, 140, 70];
    draw_chart_helper("Walnut Creek - Building", walnut_creek)?;


    Ok(())
}

fn draw_chart_helper(
    title: &'static str,
    values: Vec<i32>
) -> Result<(), Box<dyn std::error::Error>> {
    let path = format!("{title}.png").trim().to_string();

    let root = BitMapBackend::new(&path, (1200, 800)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("Arial", 20).into_font())
        .x_label_area_size(30)
        .y_label_area_size(30)
        .margin(100)
        .build_cartesian_2d(0..7, 0..300)?;

    chart.configure_mesh().draw()?;

    chart.draw_series(
        LineSeries::new(
            (0..values.len()).zip(values.into_iter()).map(|(x, y)| (x as i32, y)),
            &RED
        )
    )?;

    Ok(())
}
