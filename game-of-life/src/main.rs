use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::rect::Rect;
use rand::prelude::*;

#[derive(Copy)]
#[derive(Clone)]
#[derive(PartialEq)]
enum State {
    Dead = 0,
    Alive
}

const ROWS: usize = 200;
const COLS: usize = 200;
const CELL_SIZE: i32 = 4;


pub fn main() {
    let sdl_context = sdl2::init().expect("Can't init sdl context");
    let video_subsystem = sdl_context.video().expect("Can't init video");
    let width = CELL_SIZE as u32 * COLS as u32;
    let height = CELL_SIZE as u32 * ROWS as u32;
    let window = video_subsystem.window("rust-sdl2 demo", width, height)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut state: [[State; COLS]; ROWS] = [[State::Dead ; COLS] ; ROWS];
    random_state(&mut state);
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    break 'running;
                },
                Event::KeyDown { keycode: Some(Keycode::R), ..} => {
                    random_state(&mut state);
                },
                _ => {}
            }
        }
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        draw_state(&mut canvas, &state);
        update_state(&mut state);
        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000 / 30));
    }
}
fn random_state(state: &mut [[State; COLS]; ROWS]) {
    let mut rng = rand::thread_rng();
    for i in 0..ROWS {
        for j in 0..COLS {
            state[i][j] = if (rng.gen::<i32>())%2 == 0 {State::Dead} else {State::Alive};
        }
    }
}

fn draw_state(canvas: &mut sdl2::render::WindowCanvas, state: &[[State; COLS]; ROWS]) {
    for r in 0..ROWS {
        for c in 0..COLS {
            let row:i32 = r.try_into().unwrap();
            let col:i32 = c.try_into().unwrap();
            match state[r][c] {
                State::Dead => {
                    canvas.set_draw_color(Color::RGB(30, 30, 30));
                },
                State::Alive => {
                    canvas.set_draw_color(Color::RGB(222, 222, 222));
                }
            }
            canvas.fill_rect(Rect::new(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE as u32, CELL_SIZE as u32)).expect("Can't draw");
        }
    }
}
fn wrap(x: i32, min: i32, max: i32) -> i32 {
    if x < min {
        return max;
    } else if x > max {
        return min;
    }
    x
}
fn count_alive_neighbors(state: &[[State; COLS]; ROWS], r: i32, c: i32) -> u32 {
    static DIRECTIONS: [(i32, i32) ; 4] = [
        (0, 1), (1, 0), (1, 1), (-1, 1)
    ];
    let mut count: u32 = 0;
    for direction in DIRECTIONS {
        let mut pr = wrap(r + direction.0, 0, ROWS as i32 - 1);
        let mut pc = wrap(c + direction.1, 0, COLS as i32 - 1);
        if !(pr < 0 || pr >= ROWS as i32 || pc < 0 || pc >= COLS as i32) {
            count += if state[pr as usize][pc as usize] == State::Alive {1} else {0};
        }
        pr = wrap(r - direction.0, 0, ROWS as i32 - 1);
        pc = wrap(c - direction.1, 0, COLS as i32 - 1);
        if !(pr < 0 || pr >= ROWS as i32 || pc < 0 || pc >= COLS as i32) {
            count += if state[pr as usize][pc as usize] == State::Alive {1} else {0};
        }
    }
    count
}
//    Any live cell with fewer than two live neighbours dies, as if by underpopulation.
//    Any live cell with two or three live neighbours lives on to the next generation.
//    Any live cell with more than three live neighbours dies, as if by overpopulation.
//    Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
fn update_state(state: &mut [[State; COLS]; ROWS]) {
    let mut new_state: [[State; COLS]; ROWS] = [[State::Dead ; COLS] ; ROWS];
    for r in 1..ROWS-1 {
        for c in 1..COLS-1 {
            let n_alive_neighbors = count_alive_neighbors(state, r as i32, c as i32);
            if n_alive_neighbors < 2 || n_alive_neighbors > 3 {
                new_state[r][c] = State::Dead;
            } else if n_alive_neighbors < 3 {
                new_state[r][c] = state[r][c];
            } else if n_alive_neighbors == 3 {
                new_state[r][c] = State::Alive;
            }
        }
    }
    *state = new_state;
}
