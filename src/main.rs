extern crate minifb;

use crate::ui::app::App;
use crate::ui::app::Size;
use crate::ui::canvas::{Camera, Color};
use crate::ui::image::Image;
use crate::ui::shapes::{Line, Position};

mod ui;

fn main() {
    let mut app = App::new("sample", Size::new(1024, 1000));
    app.canvas()
        .set_camera(Camera::new(0, 0, 0))
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
