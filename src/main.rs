// todos
//
// score
// restart
// disable game input in paused/game_over state
// understand text rendering -> optimize, load font
// add starting position for snake

extern crate sdl2; 

use rand::Rng;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect;
use sdl2::render;
use sdl2::event;
use sdl2::ttf::{self, Font, Sdl2TtfContext};
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

struct Segment {
    x: i32,
    y: i32,
}

struct SnakeState {
    direction: SnakeDirection,
    requested_direction: SnakeDirection,
    segments: Vec<Segment>,
}

impl SnakeState {
    pub fn new(x: i32, y: i32, length: u32) -> SnakeState {
        let mut segments = Vec::new();

        for i in 0..length {
            segments.push(Segment {x:x - i as i32, y});
        }

        SnakeState {
            direction: SnakeDirection::Right,
            requested_direction: SnakeDirection::Right,
            segments: segments,
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
            x: 40,
            y: 28
        }
    }
}

struct GameField {
    width: u32,
    height: u32,
}

pub struct GameState {
    done: bool,
    game_over: bool,
    paused: bool,
    snake_state: SnakeState,
    food: Food,
    field: GameField,
}


impl GameState {
    pub fn new(x: i32, y: i32) -> GameState {
        GameState {
            done: false,
            game_over: false,
            paused: false,
            snake_state: SnakeState::new(x, y, 10),
            food: Food::new(),
            field: GameField { width: 60, height: 40},
        }
    }
}

pub struct Sdl2Components<'a> {
    canvas: sdl2::render::WindowCanvas,
    event_pump:  sdl2::EventPump,
    font: Font<'a, 'static>,
}

impl<'a> Sdl2Components<'a> {

    pub fn new(window_width: u32, window_height: u32, ttf_context: &'a Sdl2TtfContext) -> Sdl2Components<'a> {
        let sdl_context = sdl2::init().unwrap();
        let  event_pump = sdl_context.event_pump().unwrap();
        let window = sdl_context.video().unwrap().window("snake demo", window_width, window_height)
            .position_centered().build().unwrap();
        let canvas = window.into_canvas().build().unwrap();

        Sdl2Components {
            canvas: canvas,
            event_pump: event_pump,
            font: ttf_context.load_font("/usr/share/fonts/truetype/ubuntu/Ubuntu-B.ttf", 128).unwrap()
        }
    }
}


pub struct Config {
    window_width: u32,
    window_height: u32,
}

impl Config {
    // later we want to read the config from a configuration file
    pub fn new() -> Config {
        Config {
            window_width: 1360,
            window_height: 960,
        }
    }
}

pub fn initalize<'a>(config: &Config, ttf_context: &'a Sdl2TtfContext) -> (Sdl2Components<'a>, GameState) {

    let sdl2_components = Sdl2Components::new(config.window_width, config.window_height, &ttf_context);

    let game_state = GameState::new(10, 20);

    (sdl2_components, game_state)
}

pub fn render_text(canvas: &mut render::WindowCanvas, text:&str, target: &rect::Rect, color: &Color, style: ttf::FontStyle) {
    
}

pub fn render(state: &GameState, canvas: &mut render::WindowCanvas, font: &mut Font) {
    let margin = 80;
    let pixel_size = 20;
    
    let y_top = pixel_size * state.field.height + margin as u32;

    // draw background
    canvas.set_draw_color(Color::RGB(240, 240, 240));
    canvas.clear();

    // draw game field
    let game_rect = rect::Rect::new(margin, margin, pixel_size*state.field.width, pixel_size*state.field.height);
    
    if state.game_over {
        canvas.set_draw_color(Color::RGB(0xb2, 0x22, 0x22));
    } else {
        //canvas.set_draw_color(Color::RGB(0xbf, 0xdb, 0xf7));
        canvas.set_draw_color(Color::RGB(240,248,255));
    }
    
    let _result = canvas.fill_rect(game_rect);

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    let _result = canvas.draw_rect(game_rect);

    // draw grid
    // canvas.set_draw_color(Color::RGB(200, 200, 200));
    // for i in 1..state.field.width as i32 {
    //     let start = rect::Point::new(margin + i *(pixel_size as i32), margin);
    //     let end = rect::Point::new(margin + i* (pixel_size as i32), margin + (pixel_size as i32) * state.field.height as i32);
    //     let _result = canvas.draw_line(start, end);
    // }

    // for i in 1..state.field.height as i32 {
    //     let start = rect::Point::new(margin, margin  + i *(pixel_size as i32));
    //     let end = rect::Point::new(margin + (pixel_size as i32) * state.field.width as i32, margin + i* (pixel_size as i32));
    //     let _result = canvas.draw_line(start, end);
    // }

    // draw snake
    canvas.set_draw_color(Color::RGB(0x05, 0x3c, 0x5e));

    for segment in state.snake_state.segments.iter() {
        let snake_rect = rect::Rect::new(margin + segment.x*pixel_size as i32, 
            y_top as i32 - (segment.y+1)*pixel_size as i32,
            pixel_size,
            pixel_size);

        let _result = canvas.fill_rect(snake_rect);
    }

    // draw food
    canvas.set_draw_color(Color::RGB(0x1f, 0x7a, 0x8c));
    let food_rect = rect::Rect::new(margin + state.food.x*pixel_size as i32, 
                                    y_top as i32 - (state.food.y+1)*pixel_size as i32,
                                    pixel_size,
                                    pixel_size);
    let _result = canvas.fill_rect(food_rect);

    if state.paused {
        let texture_creator = canvas.texture_creator();

        font.set_style(sdl2::ttf::FontStyle::BOLD);

        // render a surface, and convert it to a texture bound to the canvas
        let surface = font.render("Paused")
            .blended(Color::RGBA(255, 255, 255, 255)).map_err(|e| e.to_string()).unwrap();
        let texture = texture_creator.create_texture_from_surface(&surface)
            .map_err(|e| e.to_string()).unwrap();

        let target_rect = rect::Rect::new(480, 400, 400, 120);
        let text_background = rect::Rect::new(0, 200, 1360, 500);

        canvas.set_draw_color(Color::RGB(0x05+10, 0x3c+10, 0x5e+10));
        canvas.set_blend_mode(sdl2::render::BlendMode::Blend);
        let _result = canvas.fill_rect(text_background);
        canvas.copy(&texture, None, Some(target_rect)).unwrap();
    }

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
                    Keycode::R => {
                        state.snake_state = SnakeState::new(10, 20, 10);
                        state.game_over = false;
                    },
                    Keycode::P => state.paused = !state.paused,
                    Keycode::G => state.game_over = true, // for debugging purposes 
                    _ => {}   
                }
            },
            _ => {}
        }
    }
}

pub fn process_game_logic(state: &mut GameState) {
    if !state.game_over && !state.paused {
        // figure out snake heading
        if !state.snake_state.requested_direction.is_opposite(state.snake_state.direction) {
            state.snake_state.direction = state.snake_state.requested_direction;
        } else {
            state.snake_state.direction = state.snake_state.direction;
        }

        let last_seg = state.snake_state.segments.len();
        let last = state.snake_state.segments.last().unwrap();
        let back = (last.x, last.y);

        for i in 1..last_seg {
            let prev = last_seg - i;
            let cur = prev - 1;
            state.snake_state.segments[prev].x = state.snake_state.segments[cur].x;
            state.snake_state.segments[prev].y = state.snake_state.segments[cur].y;
        }

        match state.snake_state.direction {
            SnakeDirection::Up => state.snake_state.segments[0].y = (state.snake_state.segments[0].y + 1) % state.field.height as i32,
            SnakeDirection::Down => {
                state.snake_state.segments[0].y -= 1;
                if state.snake_state.segments[0].y < 0 {
                    state.snake_state.segments[0].y = state.field.height as i32 - 1;
                }
            },
            SnakeDirection::Left => {
                state.snake_state.segments[0].x -= 1;
                if state.snake_state.segments[0].x < 0 {
                    state.snake_state.segments[0].x = state.field.width as i32 - 1;
                }
            },
            SnakeDirection::Right => state.snake_state.segments[0].x = (state.snake_state.segments[0].x + 1) % state.field.width as i32,
        }

        // check collision with itself
        for i in 1..last_seg {
            if (state.snake_state.segments[0].x == state.snake_state.segments[i].x) &&
               (state.snake_state.segments[0].y == state.snake_state.segments[i].y) {
                state.game_over = true;
                return;
            }
        }

        // eating
        if state.snake_state.segments[0].x == state.food.x && state.snake_state.segments[0].y == state.food.y {
            let mut rng = rand::thread_rng(); // not very efficient
            let new_x = rng.gen_range(0, state.field.width); // TODO: prohibit that it spawns inside of snake
            let new_y = rng.gen_range(0, state.field.height);
            state.food.x = new_x as i32;
            state.food.y = new_y as i32;
            state.snake_state.segments.push(Segment {x:back.0 as i32, y:back.1});
        }
    }
}



pub fn main() {
 
    let config = Config::new();

    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();
    let (mut sdl_components, mut state) = initalize(&config, &ttf_context);


    while !state.done {

        // render
        render(&state, &mut sdl_components.canvas, &mut sdl_components.font);
        
        // handle input 
        handle_events(&mut state, sdl_components.event_pump.poll_iter());
        
        // apply game logic
        process_game_logic(&mut state);


        // slow it down a bit
        let wait_time = time::Duration::from_millis(50);
        ::std::thread::sleep(wait_time);
    }
}

