pub use backend::CairoBackend;
use gtk::prelude::*;
use gtk::{ApplicationWindow, Builder};
use plotters::prelude::{
    ChartBuilder, Circle, DiscreteRanged, EmptyElement, IntoDrawingArea, IntoLinspace, PathElement,
    Text, Rectangle,
};
use plotters::series::{LineSeries, PointSeries, SurfaceSeries};
use plotters::style::{full_palette::*, ShapeStyle, Color};


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
        let area = CairoBackend::new(&ctx, (width as u32, height as u32))
            .unwrap()
            .into_drawing_area();

        area.fill(&WHITE).unwrap();

        let x_axis = (-3.0..3.0).step(0.1);
        let z_axis = (-3.0..3.0).step(0.1);
    
        let mut chart = ChartBuilder::on(&area)
            .caption(format!("3D Plot Test"), ("sans", 20))
            .build_cartesian_3d(x_axis.clone(), -3.0..3.0, z_axis.clone()).unwrap();
    
        chart.with_projection(|mut pb| {
            pb.yaw = 0.5;
            pb.scale = 0.9;
            pb.into_matrix()
        });
    
        chart
            .configure_axes()
            .light_grid_style(BLACK.mix(0.15))
            .max_light_lines(3)
            .draw().unwrap();
    
        chart
            .draw_series(
                SurfaceSeries::xoz(
                    (-30..30).map(|f| f as f64 / 10.0),
                    (-30..30).map(|f| f as f64 / 10.0),
                    |x, z| (x * x + z * z).cos(),
                )
                .style(BLUE.mix(0.2).filled()),
            ).unwrap()
            .label("Surface")
            .legend(|(x, y)| Rectangle::new([(x + 5, y - 5), (x + 15, y + 5)], BLUE.mix(0.5).filled()));
    
        chart
            .draw_series(LineSeries::new(
                (-100..100)
                    .map(|y| y as f64 / 40.0)
                    .map(|y| ((y * 10.0).sin(), y, (y * 10.0).cos())),
                &BLACK,
            )).unwrap()
            .label("Line")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLACK));
    
        chart
            .configure_series_labels()
            .border_style(&BLACK)
            .draw().unwrap();
    
        // To avoid the IO failure being ignored silently, we manually call the present function
        area.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");

        area.present().unwrap();

        Inhibit(true)
    });

    window.show_all();
}
