use crate::ui::canvas::{Canvas, Color, Drawing};
use std::ops::Range;

pub struct Position {
    x: i32,
    y: i32,
    z: i32,
}

impl Position {
    pub fn new(x: i32, y: i32, z: i32) -> Position {
        Position { x, y, z }
    }

    pub fn from_2d(x: i32, y: i32) -> Position {
        Position { x, y, z: 0 }
    }

    pub fn from_3d(x: i32, y: i32, z: i32) -> Position {
        Position { x, y, z }
    }
}

pub struct Point {
    pos: Position,
    color: Color,
}

impl Point {
    pub fn new(x: i32, y: i32, z: i32, color: Color) -> Point {
        Point {
            pos: Position::new(x, y, z),
            color,
        }
    }
}

impl Drawing for Point {
    fn draw(&self, canvas: &mut Canvas) -> () {
        canvas.paint(self.pos.x, self.pos.y, self.pos.z, self.color);
    }
}

pub struct Line {
    a: Position,
    b: Position,
    color: Color,
}

impl Line {
    pub fn new(from: Position, to: Position, color: Color) -> Line {
        Line {
            a: from,
            b: to,
            color,
        }
    }
}

impl Drawing for Line {
    fn draw(&self, canvas: &mut Canvas) -> () {
        for i in range(self.a.x, self.b.x) {
            for j in range(self.a.y, self.b.y) {
                for k in range(self.a.z, self.b.z) {
                    canvas.paint(i, j, k, self.color)
                }
            }
        }
    }
}

fn range(a: i32, b: i32) -> Range<i32> {
    if a > b {
        (b..(a + 1))
    } else {
        (a..(b + 1))
    }
}

/*
pub struct Rectangle {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

impl Drawing for Rectangle {
    fn draw(&self, &canvas: Canvas) -> () {}
}

pub struct Line {
    x: i32,
    y: i32,
    color: Color,
}

impl Drawing for Line {
    fn draw(&self, &canvas: Canvas) -> () {}
}
*/
