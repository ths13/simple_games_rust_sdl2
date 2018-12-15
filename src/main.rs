extern crate sdl2;
extern crate rand;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
//use sdl2::EventPump;
//use sdl2::render::Canvas;
//use std::{thread, time};

// MODEL CONSTANTS

#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    Right,
    Left,
    Up,
    Down,
}

pub mod simple_snake {
//    use sdl2::rect::Rect;
    use std::collections::VecDeque;
    use super::Direction;
    use rand::{thread_rng, Rng};

    //pub const SCENE_RANGE: Rect = Rect::new(0, 0, 1024, 768);
    pub const SCENE_RANGE_X: u32 = 1024;
    pub const SCENE_RANGE_Y: u32 = 768;
    pub const HALF_SPRITE_SIZE: u32 = 5;
    pub const SPRITE_SIZE: u32 = 2 * HALF_SPRITE_SIZE;
    pub const MIN_X: u32 = 0;
    pub const MAX_X: u32 = SCENE_RANGE_X / SPRITE_SIZE - 1;
    pub const MIN_Y: u32 = 0;
    pub const MAX_Y: u32 = SCENE_RANGE_Y / SPRITE_SIZE - 1;

    pub const MAX_FOOD: usize = 3;

    #[derive(Clone, PartialEq)]
    pub struct Position {
        pub x: u32,
        pub y: u32,
    }

    pub struct Food {
        pub locs: Vec<Position>,
    }

    impl Food {
        fn new() -> Food {
            let locs: Vec<Position> = vec![];

            Food { locs }
        }
    }

    pub struct Snake {
        pub dir: Direction,
        pub segments: VecDeque<Position>,
    }

    impl Snake {
        fn new() -> Snake {
            let dir: Direction = Direction::Left;
            let segments: VecDeque<Position> = VecDeque::new();

            Snake { dir, segments }
        }
        fn update(&mut self) {
            let mut new_head: Position = self.segments.front().unwrap().clone();
            match self.dir {
                Direction::Down => new_head.y += 1,
                Direction::Up => new_head.y -= 1,
                Direction::Left => new_head.x -= 1,
                Direction::Right => new_head.x += 1,
            }
            self.segments.pop_back();
            self.segments.push_front(new_head);
        }

        fn grow(&mut self) {
            let new_head: Position = self.segments.back().unwrap().clone();
            self.segments.push_back(new_head);
        }
    }

    pub struct Model {
        pub food: Food,
        pub snake: Snake,
    }

    impl Model {
        fn new() -> Model {
            let food: Food = Food::new();
            let snake: Snake = Snake::new();

            Model { food, snake }
        }

        fn add_random_food(&mut self) {
            while self.food.locs.len() < MAX_FOOD {
                self.food.locs.push(
                    random_position(MIN_X, MAX_X, MIN_Y, MAX_Y)
                );
            }
        }

        fn add_snake_start(&mut self) {
            self.snake.segments.push_back(
                random_position(MIN_X, MAX_X, MIN_Y, MAX_Y)
            );
        }

        fn food_collision(&mut self) -> bool {
            let snake_head: &Position = self.snake.segments.front().expect("expected front food_collision");
            for i in 0..self.snake.segments.len() {
                if &self.food.locs[i] == snake_head {
                    self.food.locs.remove(i);
                    return true
                }
            }
            false
        }

        fn self_collision(&self) -> bool {
            let snake_head: &Position = self.snake.segments.front().expect("expected front self_collision");
            for i in 1..self.snake.segments.len() {
                if &self.snake.segments[i] == snake_head {
                    return true
                }
            }
            false
        }

        fn out_of_bounds(&self) -> bool {
            let snake_head: &Position = self.snake.segments.front().expect("expected front out_of_bounds");
            snake_head.x < MIN_X || snake_head.x > MAX_X || snake_head.y < MIN_Y || snake_head.y > MAX_Y
        }

        fn update(&mut self) {
            self.snake.update()
        }
    }

    pub struct SimpleSnake {
        pub model: Model,
        pub game_over: bool,
        pub is_paused: bool,
    }

    impl SimpleSnake {
        pub fn new() -> SimpleSnake {
            let game_over: bool = false;
            let is_paused: bool = false;

            let model: Model = Model::new();

            SimpleSnake {
                model,
                game_over,
                is_paused,
            }
        }

        pub fn on_start(&mut self) {
            self.model.add_random_food();
            self.model.add_snake_start();
        }

        pub fn update(&mut self) {
            if !self.is_paused && !self.game_over {
                self.model.update();
            }
            if self.model.out_of_bounds() || self.model.self_collision() {
                self.game_over = true;
            }
            if self.model.food_collision() {
                self.model.snake.grow();
                self.model.add_random_food();
            }
        }
    }

    pub fn random_position(min_x: u32, max_x: u32, min_y: u32, max_y: u32) -> Position {
        let mut rng = thread_rng();
        let x: u32 = rng.gen_range(min_x, max_x);
        let y: u32 = rng.gen_range(min_y, max_y);
        let pos = Position { x, y };
        pos
    }
}


pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("simple snake game: sdl2", 1024, 768)
        .position_centered()
        .opengl() // unnecessary?
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut simple_snake = simple_snake::SimpleSnake::new();
    simple_snake.on_start();

    let mut frame: u32 = 0;

    'running: loop {
        for event in event_pump.poll_iter() {
            let cur_dir: Direction = simple_snake.model.snake.dir;

            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    if cur_dir != Direction::Right {
                        simple_snake.model.snake.dir = Direction::Left;
                    }
                }
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    if cur_dir != Direction::Left {
                        simple_snake.model.snake.dir = Direction::Right;
                    }
                }
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    if cur_dir != Direction::Down {
                        simple_snake.model.snake.dir = Direction::Up;
                    }
                }
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    if cur_dir != Direction::Up {
                        simple_snake.model.snake.dir = Direction::Down;
                    }
                }
                Event::KeyDown { keycode: Some(Keycode::P), .. } => {
                    simple_snake.is_paused = !simple_snake.is_paused;
                }
                _ => {}
            }
        }
        //thread::sleep(time::Duration::from_millis(10));
        println!("here, frame {}", frame);
        if frame >= 30 {
            simple_snake.update();
            frame = 0;
        }

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();
        canvas.present();

        if !simple_snake.is_paused {
            frame += 1;
        }
    }
}
