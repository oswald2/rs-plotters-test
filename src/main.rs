pub use backend::CairoBackend;
use gtk::prelude::*;
use gtk::{ApplicationWindow, Builder};
use plotters::prelude::{
    ChartBuilder, Circle, DiscreteRanged, EmptyElement, IntoDrawingArea, IntoLinspace, PathElement,
    Text,
};
use plotters::series::{LineSeries, PointSeries};
use plotters::style::{full_palette::*, ShapeStyle};


pub mod backend;

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.builder_basics"),
        Default::default(),
    );

    application.connect_activate(build_ui);

    application.run();
}

fn build_ui(application: &gtk::Application) {
    let glade_src = include_str!("main.ui");
    let builder = Builder::from_string(glade_src);

    let window: ApplicationWindow = builder
        .object("mainWindow")
        .expect("Couldn't get mainWindow");
    window.set_application(Some(application));

    let drawing_area: gtk::DrawingArea = builder
        .object("drawingArea")
        .expect("Couldn't get drawing area");

    drawing_area.connect_draw(|da, ctx| {
        let width = da.allocated_width();
        let height = da.allocated_height();
        let root_area = CairoBackend::new(&ctx, (width as u32, height as u32))
            .unwrap()
            .into_drawing_area();

        root_area.fill(&WHITE).unwrap();

        let root_area = root_area.titled("Image Title", ("sans-serif", 60)).unwrap();

        let (upper, lower) = root_area.split_vertically(512);

        let x_axis = (-3.4f32..3.4).step(0.1);

        let mut cc = ChartBuilder::on(&upper)
            .margin(5)
            .set_all_label_area_size(50)
            .caption("Sine and Cosine", ("sans-serif", 40))
            .build_cartesian_2d(-3.4f32..3.4, -1.2f32..1.2f32)
            .unwrap();

        cc.configure_mesh()
            .x_labels(20)
            .y_labels(10)
            .disable_mesh()
            .x_label_formatter(&|v| format!("{:.1}", v))
            .y_label_formatter(&|v| format!("{:.1}", v))
            .draw()
            .unwrap();

        cc.draw_series(LineSeries::new(x_axis.values().map(|x| (x, x.sin())), &RED))
            .unwrap()
            .label("Sine")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

        cc.draw_series(LineSeries::new(
            x_axis.values().map(|x| (x, x.cos())),
            &BLUE,
        ))
        .unwrap()
        .label("Cosine")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

        cc.configure_series_labels().border_style(&BLACK).draw().unwrap();

        /*
        // It's possible to use a existing pointing element
         cc.draw_series(PointSeries::<_, _, Circle<_>>::new(
            (-3.0f32..2.1f32).step(1.0).values().map(|x| (x, x.sin())),
            5,
            Into::<ShapeStyle>::into(&RGBColor(255,0,0)).filled(),
        ))?;*/

        // Otherwise you can use a function to construct your pointing element yourself
        cc.draw_series(PointSeries::of_element(
            (-3.0f32..2.1f32).step(1.0).values().map(|x| (x, x.sin())),
            5,
            ShapeStyle::from(&RED).filled(),
            &|coord, size, style| {
                EmptyElement::at(coord)
                    + Circle::new((0, 0), size, style)
                    + Text::new(format!("{:?}", coord), (0, 15), ("sans-serif", 15))
            },
        )).unwrap();

        let drawing_areas = lower.split_evenly((1, 2));

        for (drawing_area, idx) in drawing_areas.iter().zip(1..) {
            let mut cc = ChartBuilder::on(&drawing_area)
                .x_label_area_size(30)
                .y_label_area_size(30)
                .margin_right(20)
                .caption(format!("y = x^{}", 1 + 2 * idx), ("sans-serif", 40))
                .build_cartesian_2d(-1f32..1f32, -1f32..1f32).unwrap();
            cc.configure_mesh()
                .x_labels(5)
                .y_labels(3)
                .max_light_lines(4)
                .draw().unwrap();

            cc.draw_series(LineSeries::new(
                (-1f32..1f32)
                    .step(0.01)
                    .values()
                    .map(|x| (x, x.powf(idx as f32 * 2.0 + 1.0))),
                &BLUE,
            )).unwrap();
        } // After this point, we should be able to draw construct a chart context

        root_area.present().unwrap();

        Inhibit(true)
    });

    window.show_all();
}
