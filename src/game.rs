mod ball;
mod event_result;
mod paddle;

use ball::Ball;
use event_result::EventResult;
use paddle::Paddle;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use std::time::Duration;
use std::time::SystemTime;
const TITLE: &str = "title";
const WINDOW_WIDTH: u32 = 1024;
const WINDOW_HEIGHT: u32 = 768;
const WALL_THICKNESS: u32 = 10;
const PADDLE_INITIAL_X: u32 = 10;
const PADDLE_INITIAL_Y: u32 = WINDOW_HEIGHT / 2;
const PADDLE_WIDTH: u32 = 5;
const PADDLE_HEIGHT: u32 = 40;
const BALL_INITIAL_X: u32 = WINDOW_WIDTH / 2;
const BALL_INITIAL_Y: u32 = WINDOW_HEIGHT / 2;
const BALL_INITIAL_VX: u32 = 400;
const BALL_INITIAL_VY: u32 = 100;
const BALL_WIDTH: u32 = 10;
const BALL_HEIGHT: u32 = 10;

pub(crate) struct Game {
    sdl_context: sdl2::Sdl,
    canvas: WindowCanvas,
    last_frame_time: SystemTime,
    paddle: Paddle,
    ball: Ball,
}

impl Game {
    pub fn initialize() -> Result<Game, String> {
        let sdl_context = match sdl2::init() {
            Ok(context) => context,
            Err(_) => return Err("cannot initialize SDL.".to_string()),
        };

        let video_subsystem = match sdl_context.video() {
            Ok(video) => video,
            Err(_) => return Err("cannot initialize video.".to_string()),
        };

        let window = match video_subsystem
            .window(TITLE, WINDOW_WIDTH, WINDOW_HEIGHT)
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())
        {
            Ok(window) => window,
            Err(str) => return Err(format!("cannot initialize window: {str}")),
        };

        let canvas = match window.into_canvas().build().map_err(|e| e.to_string()) {
            Ok(canvas) => canvas,
            Err(str) => return Err(format!("cannot initialize window: {str}")),
        };

        let paddle_position = Point::new(PADDLE_INITIAL_X as i32, PADDLE_INITIAL_Y as i32);
        let paddle = Paddle::new(paddle_position, PADDLE_WIDTH, PADDLE_HEIGHT);

        let ball_position = Point::new(BALL_INITIAL_X as i32, BALL_INITIAL_Y as i32);
        let ball_velocity = Point::new(BALL_INITIAL_VX as i32, BALL_INITIAL_VY as i32);
        let ball = Ball::new(ball_position, ball_velocity, BALL_WIDTH, BALL_HEIGHT);

        Ok(Game {
            sdl_context,
            canvas,
            last_frame_time: SystemTime::now(),
            paddle,
            ball,
        })
    }

    pub fn run_loop(&mut self) {
        self.last_frame_time = SystemTime::now();
        'running: loop {
            match self.process_input() {
                Ok(result) => {
                    if let EventResult::End = result {
                        break 'running;
                    }
                }
                Err(_) => break 'running,
            }
            let _ = self.update_game();
            self.generate_output();

            self.last_frame_time = SystemTime::now();
        }
    }

    fn process_input(&mut self) -> Result<EventResult, String> {
        let mut event_pump = self.sdl_context.event_pump()?;
        for event in event_pump.poll_iter() {
            match event {
                //quit game
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return Ok(EventResult::End),
                //move up paddle
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    if self.paddle.direction > -5 {
                        self.paddle.direction -= 1;
                    }
                }
                //move down paddle
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    if self.paddle.direction < 5 {
                        self.paddle.direction += 1;
                    }
                }
                //restart game
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    self.ball.position = Point::new(BALL_INITIAL_X as i32, BALL_INITIAL_Y as i32);
                    self.ball.velocity = Point::new(BALL_INITIAL_VX as i32, BALL_INITIAL_VY as i32);
                    self.paddle.position =
                        Point::new(PADDLE_INITIAL_X as i32, PADDLE_INITIAL_Y as i32);
                    self.paddle.direction = 0;
                }
                _ => {
                    self.paddle.direction = 0;
                }
            }
        }
        Ok(EventResult::Continue)
    }

    fn update_game(&mut self) -> Result<(), String> {
        //calcurate delta_time
        let mut delta_time: Duration = match self.last_frame_time.elapsed() {
            Ok(duration) => duration,
            Err(str) => return Err(str.to_string()),
        };

        const SEC_A_FRAME: u128 = 16;

        while delta_time.as_millis() < SEC_A_FRAME {
            delta_time = match self.last_frame_time.elapsed() {
                Ok(duration) => duration,
                Err(str) => return Err(str.to_string()),
            };
        }

        const MAX_DELTA_TIME: u64 = 50;

        if delta_time.as_millis() > MAX_DELTA_TIME.into() {
            delta_time = Duration::from_millis(MAX_DELTA_TIME);
        }

        //update paddle
        const PADDLE_SPEED: f64 = 300.0;

        self.paddle.position.y +=
            (self.paddle.direction as f64 * PADDLE_SPEED * delta_time.as_secs_f64()) as i32;

        if self.paddle.position.y < WALL_THICKNESS as i32 {
            self.paddle.position.y = WALL_THICKNESS as i32;
            self.paddle.direction = 0;
        } else if self.paddle.position.y
            > (WINDOW_HEIGHT - WALL_THICKNESS - self.paddle.height) as i32
        {
            self.paddle.position.y = (WINDOW_HEIGHT - self.paddle.height - WALL_THICKNESS) as i32;
            self.paddle.direction = 0;
        }

        //update ball
        self.ball.position.x += (self.ball.velocity.x as f64 * delta_time.as_secs_f64()) as i32;
        self.ball.position.y += (self.ball.velocity.y as f64 * delta_time.as_secs_f64()) as i32;

        //top and bottom wall collision
        if (self.ball.position.y <= WALL_THICKNESS as i32 && self.ball.velocity.y < 0)
            || (self.ball.position.y >= (WINDOW_HEIGHT - WALL_THICKNESS - self.ball.height) as i32
                && self.ball.velocity.y > 0)
        {
            self.ball.velocity.y *= -1;
        }

        if self.ball.position.x >= (WINDOW_WIDTH - WALL_THICKNESS - self.ball.width) as i32 {
            self.ball.velocity.x *= -1;
        }

        if (self.ball.position.x > self.paddle.position.x
            && self.ball.position.x < self.paddle.position.x + self.paddle.width as i32)
            && (self.ball.position.y > self.paddle.position.y
                && self.ball.position.y < self.paddle.position.y + self.paddle.height as i32)
            && self.ball.velocity.x < 0
        {
            self.ball.velocity.x *= -1
        }

        Ok(())
    }

    fn generate_output(&mut self) {
        self.canvas.set_draw_color(Color::RGBA(0, 0, 255, 255));
        self.canvas.clear();

        let top_wall = Rect::new(0, 0, WINDOW_WIDTH, WALL_THICKNESS);
        let bottom_wall = Rect::new(
            0,
            (WINDOW_HEIGHT - WALL_THICKNESS) as i32,
            WINDOW_WIDTH,
            WALL_THICKNESS,
        );
        let right_wall = Rect::new(
            (WINDOW_WIDTH - WALL_THICKNESS) as i32,
            0,
            WALL_THICKNESS,
            WINDOW_HEIGHT,
        );

        let ball = Rect::new(
            self.ball.position.x,
            self.ball.position.y,
            self.ball.width,
            self.ball.height,
        );

        let paddle = Rect::new(
            self.paddle.position.x,
            self.paddle.position.y,
            self.paddle.width,
            self.paddle.height,
        );

        self.canvas.set_draw_color(Color::RGBA(255, 255, 255, 255));
        let _ = self.canvas.fill_rect(top_wall);
        let _ = self.canvas.fill_rect(bottom_wall);
        let _ = self.canvas.fill_rect(right_wall);

        let _ = self.canvas.fill_rect(ball);
        let _ = self.canvas.fill_rect(paddle);

        self.canvas.present();
    }
}
