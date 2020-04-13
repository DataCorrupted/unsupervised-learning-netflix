use std::error::Error;

use log::info;
use plotters::prelude::*;

use crate::data::{Data, MetaData};

pub fn plot_initial_matrix(data: &Data, metadata: &MetaData) -> Result<(), Box<dyn Error>> {
    let MetaData {
        num_customers: n,
        num_movies: m,
        trans_freq: _,
        test_freq: _,
    } = metadata;

    let (x_label_size, y_label_size) = (20, 60);
    let margin = 10;
    let plot_size = (
        y_label_size + margin * 2 + m / 10,
        x_label_size + margin * 2 + n / 10,
    );

    let root = BitMapBackend::new("initial_matrix.png", plot_size).into_drawing_area();
    root.fill(&WHITE)?;
    let root = root.margin(10, 10, 10, 10);
    // After this point, we should be able to draw construct a chart context
    let mut chart = ChartBuilder::on(&root)
        // Set the caption of the chart
        .caption(
            "Initial completion status of the matrix",
            ("sans-serif", 40).into_font(),
        )
        // Set the size of the label region
        .x_label_area_size(x_label_size)
        .y_label_area_size(y_label_size)
        // Finally attach a coordinate on the drawing area and make a chart context
        .build_ranged(0f32..*m as f32, 0f32..*n as f32)?;

    // Then we can draw a mesh
    chart
        .configure_mesh()
        // We can customize the maximum number of labels allowed for each axis
        .x_labels(5)
        .y_labels(5)
        // We can also change the format of the label text
        .y_label_formatter(&|x| format!("{:.3}", x))
        .draw()?;

    // Similarly, we can draw point series
    chart.draw_series(PointSeries::of_element(
        data.transactions
            .iter()
            .map(|t| (t.movie_id as f32, t.customer_id as f32)),
        1,
        &BLUE,
        &|c, s, st| {
            return EmptyElement::at(c)    // We want to construct a composed element on-the-fly
            + Circle::new((0,0),s,st.filled()); // At this point, the new pixel coordinate is established
        },
    ))?;
    chart.draw_series(PointSeries::of_element(
        data.test_data
            .iter()
            .map(|t| (t.movie_id as f32, t.customer_id as f32)),
        1,
        &RED,
        &|c, s, st| {
            return EmptyElement::at(c)    // We want to construct a composed element on-the-fly
            + Circle::new((0,0),s,st.filled()); // At this point, the new pixel coordinate is established
        },
    ))?;
    Ok(())
}

fn plot_freq_histogram(
    data: &Vec<u32>,
    max_x: u32,
    max_y: u32,
    title: &'static str,
    path: &'static str,
) -> Result<(), Box<dyn Error>> {
    let root = BitMapBackend::new(path, (1920, 1080)).into_drawing_area();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(35)
        .y_label_area_size(40)
        .margin(5)
        .caption(title, ("sans-serif", 50.0).into_font())
        .build_ranged(0u32..max_x, 0u32..max_y)?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .line_style_1(&WHITE.mix(0.3))
        .x_label_offset(30)
        .y_desc("Count")
        .x_desc("Customer id")
        .axis_desc_style(("sans-serif", 15).into_font())
        .draw()?;

    chart.draw_series(
        Histogram::vertical(&chart)
            .style(BLUE.mix(0.5).filled())
            .data(data.iter().map(|x: &u32| (*x, 1))),
    )?;

    Ok(())
}

pub fn plot_data_freq(metadata: &MetaData) -> Result<(), Box<dyn Error>> {
    let MetaData {
        num_customers: n,
        num_movies: _,
        trans_freq,
        test_freq,
    } = metadata.clone();
    let max_freq = trans_freq
        .iter()
        .zip(test_freq.iter())
        .fold(0, |max_freq, (&trans, &tests)| {
            u32::max(max_freq, u32::max(trans, tests))
        });
    info!("max freq: {}", max_freq);

    plot_freq_histogram(
        &trans_freq,
        n,
        300,
        "Transaction Frequency",
        "trans_freq.png",
    )?;
    info!("Plotted transaction frequency");

    plot_freq_histogram(&test_freq, n, 300, "Test Frequency", "test_freq.png")?;
    info!("Plotted test frequency");

    Ok(())
}
