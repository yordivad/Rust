use crate::ui::canvas::Canvas;
use minifb::{Key, Scale, ScaleMode, Window, WindowOptions};

#[derive(Clone, Copy)]
pub struct Size {
    height: usize,
    width: usize,
}

impl Size {
    pub fn new(width: usize, height: usize) -> Size {
        Size { height, width }
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }
}

pub struct App {
    size: Size,
    win: Window,
    canvas: Canvas,
}

impl App {
    pub fn new(name: &str, size: Size) -> App {
        size.height();
        let win = Window::new(
            name,
            size.width(),
            size.height(),
            WindowOptions {
                borderless: true,
                title: true,
                resize: true,
                scale: Scale::X1,
                scale_mode: ScaleMode::Stretch,
            },
        )
        .unwrap();

        App {
            size,
            canvas: Canvas::new(size),
            win,
        }
    }

    pub fn canvas(&mut self) -> &mut Canvas {
        &mut self.canvas
    }

    pub fn run(&mut self) -> () {
        while self.win.is_open() & &!self.win.is_key_down(Key::Escape) {
            self.win
                .update_with_buffer(self.canvas.render(), self.size.width(), self.size.height())
                .unwrap();
        }
    }
}
