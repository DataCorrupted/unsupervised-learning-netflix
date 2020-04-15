use std::error::Error;

use log::info;
use plotters::prelude::*;

use crate::data::{Data, MetaData, Transaction};

pub fn plot_initial_matrix(data: &Data, metadata: &MetaData) -> Result<(), Box<dyn Error>> {
    let MetaData {
        num_customers: n,
        num_movies: m,
        num_train: _,
        num_cross_valid: _,
        trans_freq: _,
        tests_freq: _,
    } = metadata;
    let (n, m) = (*n as u32, *m as u32);

    let (x_label_size, y_label_size) = (100, 100);
    let margin = 100;
    let plot_size = (
        y_label_size + margin * 2 + m / 5,
        x_label_size + margin * 2 + n / 5,
    );

    let root = BitMapBackend::new("initial_matrix.png", plot_size).into_drawing_area();
    root.fill(&WHITE)?;
    let root = root.margin(margin, margin, margin, margin);
    // After this point, we should be able to draw construct a chart context
    let mut chart = ChartBuilder::on(&root)
        // Set the caption of the chart
        .caption(
            "Initial completion status of the matrix",
            ("sans-serif", 70).into_font(),
        )
        // Set the size of the label region
        .x_label_area_size(x_label_size)
        .y_label_area_size(y_label_size)
        // Finally attach a coordinate on the drawing area and make a chart context
        .build_ranged(0f32..m as f32, 0f32..n as f32)?;

    // Then we can draw a mesh
    chart
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        // We can customize the maximum number of labels allowed for each axis
        .x_labels(20)
        .y_labels(20)
        .x_desc("Movie id")
        .y_desc("User (virtual) id")
        // We can also change the format of the label text
        .x_label_formatter(&|x| format!("{}", x))
        .y_label_formatter(&|x| format!("{}", x))
        .axis_desc_style(("sans-serif", 50).into_font())
        .draw()?;

    let mut plot_points =
        |data: &Vec<Transaction>, color: &RGBColor| -> Result<(), Box<dyn Error>> {
            // Similarly, we can draw point series
            chart.draw_series(PointSeries::of_element(
                data.iter()
                    .map(|t| (t.movie_id as f32, t.customer_id as f32)),
                1,
                color,
                &|c, _s, st| {
                    return EmptyElement::at(c)    // We want to construct a composed element on-the-fly
                + Pixel::new((0,0),st.filled()); // At this point, the new pixel coordinate is established
                },
            ))?;
            Ok(())
        };

    plot_points(&data.train, &BLUE)?;
    plot_points(&data.cross_valid, &YELLOW)?;
    plot_points(&data.test_data, &RED)?;
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
        num_train: _,
        num_cross_valid: _,
        trans_freq,
        tests_freq,
    } = metadata.clone();
    let n = n as u32;
    let (max_trans, max_tests) = trans_freq.iter().zip(tests_freq.iter()).fold(
        (0, 0),
        |(max_trans, max_tests), (&trans, &tests)| {
            (u32::max(max_trans, trans), u32::max(max_tests, tests))
        },
    );
    info!("max # of trans: {}", max_trans);
    info!("max # of tests: {}", max_tests);

    plot_freq_histogram(
        &trans_freq,
        n,
        max_trans + 10,
        "Transaction Frequency",
        "trans_freq.png",
    )?;
    info!("Plotted transaction frequency");

    plot_freq_histogram(
        &tests_freq,
        n,
        max_tests + 10,
        "Test Frequency",
        "tests_freq.png",
    )?;
    info!("Plotted test frequency");

    Ok(())
}
