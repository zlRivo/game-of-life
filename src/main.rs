use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;
use std::time::Duration;
use std::time::Instant;
use std::thread;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;

use sdl2::render::WindowCanvas;

use anyhow::Result;
use anyhow::bail;

const DIFFS: &[(i32, i32)] = &[
    (-1, -1), (-1, 0), (-1, 1),
    (0, -1),           (0, 1),
    (1, -1),  (1, 0),  (1, 1)
];

const CELL_SIZE: i32 = 10;

const FPS: u64 = 144;
const SLEEP_TIME: Duration = Duration::from_millis(1000 / FPS);

const STEPS_RATE: u64 = 12; // Steps per second. DO NOT PUT TO ZERO
const STEP_FRAME: u64 = FPS / STEPS_RATE;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: i32,
    y: i32,
}

trait Draw {
    fn draw(&self, canvas: &mut WindowCanvas, x: i32, y: i32) -> Result<()>;
}

impl Draw for HashSet<Coord> {
    fn draw(&self, canvas: &mut WindowCanvas, x: i32, y: i32) -> Result<()> {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        for c in self.iter() {
            let draw_result = canvas.fill_rect(Rect::new(
                x + c.x * CELL_SIZE,
                y + c.y * CELL_SIZE,
                CELL_SIZE as u32,
                CELL_SIZE as u32
            ));

            if let Err(_) = draw_result {
                bail!("Couldn't draw");
            }
        }

        Ok(())
    }
}

/// Assigns all the neighbour indices to the given reference
fn neighbours(cell: Coord, neighbours: &mut [Coord; 8]) {
    for (i, (dx, dy)) in DIFFS.iter().enumerate() {
        neighbours[i] = Coord {
            x: cell.x + dx,
            y: cell.y + dy,
        };
    }
}

/// Returns the neighbour count for each position
fn neighbour_counts(cells: &HashSet<Coord>) -> HashMap<Coord, usize> {
    let mut counts = HashMap::new();

    // Loop for each alive cell
    for c in cells.iter() {
        let mut nei = [Coord { x: 0, y: 0 }; 8];
        neighbours(*c, &mut nei);
        for n in nei {
            // Increment neighbour reference count
            *counts.entry(n).or_insert(0) += 1
        }
    }

    counts
}

/// Converts the mouse position to grid coordinates
fn mouse_to_grid(x: i32, y: i32, cam_x: i32, cam_y: i32) -> Coord {
    Coord {
        x: (x + cam_x) / CELL_SIZE,
        y: (y + cam_y) / CELL_SIZE,
    }
}

/// Generates the new generation of cells
fn step(cells: &HashSet<Coord>) -> HashSet<Coord> {
    let mut next_gen = HashSet::new();
    
    for (cell, count) in neighbour_counts(cells) {
        if count == 3 || cells.contains(&cell) && count == 2 {
            next_gen.insert(cell);
        }
    }
    
    next_gen
}

fn run() {
    let mut cells = HashSet::new();

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Conway's Game of Life", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut frame_i = 0;
    let mut cam_x = 0;
    let mut cam_y = 0;

    let mut w_down = false;
    let mut a_down = false;
    let mut s_down = false;
    let mut d_down = false;

    let mut stepping = true;

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        let frame_time = Instant::now();

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // Mouse
        let mouse_state = event_pump.mouse_state();
        let mouse_x = mouse_state.x();
        let mouse_y = mouse_state.y();
        let lmb_pressed = mouse_state.left();
        let rmb_pressed = mouse_state.right();

        // Input
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::W), .. } => { w_down = true; },
                Event::KeyDown { keycode: Some(Keycode::A), .. } => { a_down = true; },
                Event::KeyDown { keycode: Some(Keycode::S), .. } => { s_down = true; },
                Event::KeyDown { keycode: Some(Keycode::D), .. } => { d_down = true; },
                Event::KeyUp { keycode: Some(Keycode::W), .. } => { w_down = false; },
                Event::KeyUp { keycode: Some(Keycode::A), .. } => { a_down = false; },
                Event::KeyUp { keycode: Some(Keycode::S), .. } => { s_down = false; },
                Event::KeyUp { keycode: Some(Keycode::D), .. } => { d_down = false; },

                Event::KeyDown { keycode: Some(Keycode::Space), .. } => { stepping = !stepping },

                Event::KeyDown { keycode: Some(Keycode::Delete), .. } => { cells.clear(); },
                _ => {}
            }
        }

        // Update
        if w_down { cam_y -= 15; }
        if a_down { cam_x -= 15; }
        if s_down { cam_y += 15; }
        if d_down { cam_x += 15; }

        if lmb_pressed {
            // Get grid position
            let grid_pos = mouse_to_grid(mouse_x, mouse_y, cam_x, cam_y);

            // Add cell to grid
            cells.insert(grid_pos);
        }

        if rmb_pressed {
            // Get grid position
            let grid_pos = mouse_to_grid(mouse_x, mouse_y, cam_x, cam_y);

            // Remove cell from grid
            cells.remove(&grid_pos);
        }
        
        // Render
        let _ = cells.draw(&mut canvas, -cam_x, -cam_y);

        if stepping {
            if frame_i >= STEP_FRAME {
                frame_i = 0;
                cells = step(&cells);
            }

            frame_i += 1;
        }
        
        canvas.present();

        // Ensure the game is running at the right framerate
        if let Some(d) = SLEEP_TIME.checked_sub(frame_time.elapsed()) {
            thread::sleep(d);
        }
    }
}

fn main() {
    run();
}