extern crate minifb;
extern crate plotly;

use crate::ui::app::App;
use crate::ui::app::Size;
use crate::ui::canvas::{Camera, Color};
use crate::ui::image::Image;
use crate::ui::shapes::{Line, Position};
use plotly::charts::{Axis, Layout, Mode, Scatter, Title};
use plotly::Plot;
use std::env::var;

mod ui;

fn main() {
    let mut layout = Layout::new("Data Labels");

    let mut x_axis = Axis::new("X");
    x_axis.range = Some(vec![-10.0, 10.0]);
    let mut y_axis = Axis::new("y");
    y_axis.range = Some(vec![-10.0, 10.0]);

    layout.title = Some(Title::new("graph".parse().unwrap()));
    layout.xaxis = Some(x_axis);
    layout.yaxis = Some(y_axis);

    let mut trace = Scatter::new("trace1", vec![1, 2, 3, 4, 10], (0..10).collect());
    trace.mode = Mode::Markers;

    let mut plot = Plot::new();
    plot.add_trace(trace);
    plot.add_layout(layout);
    plot.show();

    //  my_view();
}

fn my_view() {
    let mut app = App::new("sample", Size::new(1024, 1000));
    app.canvas()
        .set_camera(Camera::new(10, 100, 10))
        .draw(Line::new(
            Position::from_2d(-300, 0),
            Position::from_2d(300, 0),
            Color::green(),
        ))
        .draw(Line::new(
            Position::from_2d(0, -300),
            Position::from_2d(0, 300),
            Color::green(),
        ))
        .draw(Line::new(
            Position::from_3d(0, 0, -300),
            Position::from_3d(0, 0, 300),
            Color::red(),
        ))
        .draw(Image::new("/Users/rgonzalez/data/dog.png"));
    app.run();
}
