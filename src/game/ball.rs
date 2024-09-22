use sdl2::rect::Point;
pub(crate) struct Ball {
    pub position: Point,
    pub velocity: Point,
    pub width: u32,
    pub height: u32,
}

impl Ball {
    pub fn new(position: Point, velocity: Point, width: u32, height: u32) -> Ball {
        Ball {
            position,
            velocity,
            width,
            height,
        }
    }
}
