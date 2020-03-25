extern crate sdl2; 

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect;
use sdl2::render;
use sdl2::event;
use std::time;

#[derive(PartialEq, Copy, Clone)]
enum SnakeDirection {
    Up,
    Down,
    Left,
    Right,
}

impl SnakeDirection {
    pub fn is_opposite(&self, other: SnakeDirection) -> bool {
        if (*self == SnakeDirection::Up && other == SnakeDirection::Down)  ||
           (*self == SnakeDirection::Down && other == SnakeDirection::Up) {
            return true;
        } else if (*self == SnakeDirection::Left && other == SnakeDirection::Right)  ||
                  (*self == SnakeDirection::Right && other == SnakeDirection::Left) {
            return true;
        } else {
            return false;
        }
    }
}

struct SnakeState {
    x: i32,
    y: i32,
    direction: SnakeDirection,
    requested_direction: SnakeDirection,
}

impl SnakeState {
    pub fn new(x: i32, y: i32) -> SnakeState {
        SnakeState {
            x: x,
            y: y,
            direction: SnakeDirection::Right,
            requested_direction: SnakeDirection::Right,
        }
    }
}

struct Food {
    x: i32,
    y: i32
}

impl Food {
    pub fn new() -> Food {
        Food {
            x: 30,
            y: 25
        }
    }
}

struct GameField {
    width: u32,
    height: u32,
}

pub struct GameState {
    done: bool,
    snake_state: SnakeState,
    food: Food,
    field: GameField,
}


impl GameState {
    pub fn new(x: i32, y: i32) -> GameState {
        GameState {
            done: false,
            snake_state: SnakeState::new(x, y),
            food: Food::new(),
            field: GameField { width: 60, height: 40},
        }
    }
}



pub fn render(state: &GameState, canvas: &mut render::WindowCanvas) {
    let margin = 80;
    let pixel_size = 20;
    
    let y_top = pixel_size * state.field.height + margin as u32;

    // draw background
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    
    // draw game field
    let game_rect = rect::Rect::new(margin, margin, pixel_size*state.field.width, pixel_size*state.field.height);
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    let _result = canvas.draw_rect(game_rect);

    // draw grid
    canvas.set_draw_color(Color::RGB(200, 200, 200));
    for i in 1..state.field.width as i32 {
        let start = rect::Point::new(margin + i *(pixel_size as i32), margin);
        let end = rect::Point::new(margin + i* (pixel_size as i32), margin + (pixel_size as i32) * state.field.height as i32);
        let _result = canvas.draw_line(start, end);
    }

    for i in 1..state.field.height as i32 {
        let start = rect::Point::new(margin, margin  + i *(pixel_size as i32));
        let end = rect::Point::new(margin + (pixel_size as i32) * state.field.width as i32, margin + i* (pixel_size as i32));
        let _result = canvas.draw_line(start, end);
    }

    // draw snake
    let snake_rect = rect::Rect::new(margin + state.snake_state.x*pixel_size as i32, 
                                     y_top as i32 - state.snake_state.y*pixel_size as i32,
                                     pixel_size,
                                     pixel_size);
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    let _result = canvas.fill_rect(snake_rect);

    println!("x pos : {}", state.snake_state.x as i32);
    println!("x: {}", margin + state.snake_state.x*pixel_size as i32);
    println!("y top: {}", y_top);

    // draw food

    canvas.set_draw_color(Color::RGB(50, 100, 50));
    let food_rect = rect::Rect::new(margin + state.food.x*pixel_size as i32, 
                                    y_top as i32 - state.food.y*pixel_size as i32,
                                    pixel_size,
                                    pixel_size);
    let _result = canvas.fill_rect(food_rect);

    canvas.present();
}

pub fn handle_events(state: &mut GameState, event_iter: event::EventPollIterator) {
    for event in event_iter {
        match event {
            Event::Quit {..} |
            Event::KeyUp { keycode: Some(Keycode::Escape), .. } => {
                state.done = true;
            },
            Event::KeyDown { keycode: Some(x), .. } => {
                match x {
                    Keycode::Left => state.snake_state.requested_direction = SnakeDirection::Left,
                    Keycode::Right => state.snake_state.requested_direction = SnakeDirection::Right,
                    Keycode::Up => state.snake_state.requested_direction = SnakeDirection::Up,
                    Keycode::Down => state.snake_state.requested_direction = SnakeDirection::Down,
                    _ => {}   
                }
            },
            // Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
            //     state.snake_state.x += 10;
            // },
            // Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
            //     state.snake_state.x -= 10;
            // },

            // Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                
            // },
            _ => {}
        }
    }
}

pub fn process_game_logic(state: &mut GameState) {
    // figure out snake heading
    if !state.snake_state.requested_direction.is_opposite(state.snake_state.direction) {
        state.snake_state.direction = state.snake_state.requested_direction;
    } else {
        state.snake_state.direction = state.snake_state.direction;
    }

    match state.snake_state.direction {
        SnakeDirection::Up => state.snake_state.y += 1,
        SnakeDirection::Down => state.snake_state.y -= 1,
        SnakeDirection::Left => state.snake_state.x -= 1,
        SnakeDirection::Right => state.snake_state.x += 1,
    }
}



pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let window = video_subsystem.window("snake demo", 1360, 960)
        .position_centered()
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas().build().unwrap();
    let mut state = GameState::new(10, 20);

    while !state.done {

        // render
        render(&state, &mut canvas);
        
        // handle input 
        handle_events(&mut state, event_pump.poll_iter());
        
        // apply game logic
        process_game_logic(&mut state);


        // slow it down a bit
        let wait_time = time::Duration::from_millis(80);
        ::std::thread::sleep(wait_time);
    }
}