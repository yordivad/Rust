use crate::ui::app::Size;

pub struct Pixel {
    x: usize,
    y: usize,
    color: Color,
}

impl Pixel {
    pub fn new(x: usize, y: usize, color: Color) -> Pixel {
        Pixel { x, y, color }
    }
    pub fn color(&self) -> u32 {
        self.color.as_hex()
    }
}

#[derive(Clone, Copy)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub fn as_hex(&self) -> u32 {
        ((self.r as u32 & 0xff) << 16) + ((self.g as u32 & 0xff) << 8) + (self.b as u32 & 0xff)
    }

    pub fn as_color(&self) -> u32 {
        (self.r as u32) << 16 | (self.g as u32) << 8 | (self.b as u32)
    }

    pub fn from(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }

    pub fn red() -> Color {
        Color { r: 255, g: 0, b: 0 }
    }

    pub fn black() -> Color {
        Color { r: 0, g: 0, b: 0 }
    }

    pub fn green() -> Color {
        Color { r: 0, g: 255, b: 0 }
    }

    pub fn white() -> Color {
        Color {
            r: 255,
            g: 255,
            b: 255,
        }
    }
}

pub trait Drawing {
    fn draw(&self, canvas: &mut Canvas) -> ();
}

pub struct ViewPort {
    factor: i32,
    size: Size,
}

impl ViewPort {
    pub fn new(size: Size) -> ViewPort {
        ViewPort { factor: 1, size }
    }

    pub fn get_pixel(&self, x: i32, y: i32, color: Color) -> Pixel {
        let half_x = ((self.size.width() / 2) - 1) as i32;
        let half_y = ((self.size.height() / 2) - 1) as i32;

        let xp = (half_x + (x * self.factor)) as usize;
        let yp = (half_y + (y * -1 * self.factor)) as usize;

        let x = if xp > self.size.width() {
            self.size.width()
        } else {
            xp
        };
        let y = if yp > self.size.height() {
            self.size.height()
        } else {
            yp
        };

        Pixel::new(x, y, color)
    }
}

pub struct Canvas {
    buffer: Vec<u32>,
    view: ViewPort,
    size: Size,
    camera: Camera,
}

impl Canvas {
    pub fn new(size: Size) -> Canvas {
        let buffer: Vec<u32> = vec![0; size.width() * size.height()];
        Canvas {
            buffer,
            size,
            view: ViewPort::new(size),
            camera: Camera::origin(),
        }
    }

    pub fn render(&mut self) -> &[u32] {
        self.buffer.as_slice()
    }

    pub fn paint(&mut self, x: i32, y: i32, z: i32, color: Color) {
        let f = self.camera.focus(x, y, z);
        let p = self.view.get_pixel(f.0, f.1, color);
        let page = p.y * self.size.width();
        let pos = page + p.x;
        if pos < self.buffer.len() {
            self.buffer[pos] = p.color();
        }
    }

    pub fn draw(&mut self, figure: impl Drawing) -> &mut Canvas {
        figure.draw(self);
        self
    }

    pub fn set_camera(&mut self, camera: Camera) -> &mut Canvas {
        self.camera = camera;
        self
    }
}

pub struct Camera {
    cx: i32,
    cy: i32,
    cz: i32,
}

impl Camera {
    pub fn origin() -> Camera {
        Camera {
            cx: 0,
            cy: 0,
            cz: 0,
        }
    }

    pub fn new(cx: i32, cy: i32, cz: i32) -> Camera {
        Camera { cx, cy, cz }
    }

    pub fn focus(&self, x: i32, y: i32, z: i32) -> (i32, i32) {
        (0, 0)
    }
}
