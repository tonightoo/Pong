use sdl2::rect::Point;

pub struct Paddle {
    pub position: Point,
    pub direction: i32,
    pub width: u32,
    pub height: u32,
}

impl Paddle {
    pub fn new(position: Point, width: u32, height: u32) -> Paddle {
        Paddle {
            position,
            direction: 0,
            width,
            height,
        }
    }
}
