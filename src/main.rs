use rand::Rng;
use std::time::Instant;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Scancode;

const TILE_SIZE: u8 = 40;
const SIZE: [usize; 2] = [10, 20];

struct Piece {
    points: [[i32; 2]; 4],
    reach: [usize; 4], // left, up, right, down; how many pieces extend from "center" point
    id: u8
}

fn create_piece(piece_id: u8) -> Option<Piece> {
    match piece_id { // positions based around center of piece
        0 => Some(Piece { points: [[-1, 0], [0, 0], [1, 0], [2, 0]], reach: [1, 0, 3, 1], id: 0 }),
        1 => Some(Piece { points: [[-1, -1], [0, -1], [1, -1], [0, 0]], reach: [1, 1, 2, 1], id: 1 }),
        2 => Some(Piece { points: [[-1, 0], [0, 0], [1, 0], [-1, 1]], reach: [1, 0, 2, 2], id: 2 }),
        3 => Some(Piece { points: [[0, -1], [1, -1], [-1, 0], [0, 0]], reach: [1, 1, 2, 2], id: 3 }),
        4 => Some(Piece { points: [[0, 0], [1, 0], [0, 1], [1, 1]], reach: [0, 0, 2, 2], id: 4 }),
        _ => None
    }
}

fn rotate_cw(piece: &mut Piece) { // rotate clockwise, formula comes from rotation matrices, signs from matrices for y-coord are flipped b/c positive is down
    for point in piece.points.iter_mut() {
        *point = [-point[1], point[0]];
    }
    piece.reach.rotate_right(1);
}

fn rotate_ccw(piece: &mut Piece) { // rotate counterclockwise, formula comes from rotation matrices, signs from matrices for y-coord are flipped b/c positive is down
    for point in piece.points.iter_mut() {
        *point = [point[1], -point[0]];
    }
    piece.reach.rotate_left(1);
}

fn main() {
    let context = sdl2::init().unwrap();
    let mut canvas = context.video().unwrap().window("Tetris", 400, 800).position_centered().opengl().build().unwrap().into_canvas().build().unwrap();

    canvas.clear();
    canvas.present();
    
    let mut rng = rand::thread_rng();
    let mut piece = create_piece(rng.gen_range(0..5)).unwrap();
    let mut position: [i32; 2] = [rng.gen_range(piece.reach[0]..(SIZE[0] - piece.reach[2] + 1)).try_into().unwrap(), piece.reach[1].try_into().unwrap()];
    let mut events = context.event_pump().unwrap();
    let mut time = Instant::now();
    let mut ticks = 0;
    let mut rotate = 0;
    let mut rotate_ticks = 0;
    let mut shift = 0;
    let mut shift_ticks = 0;
    //let mut map: [[u8; SIZE[0]]; SIZE[1]] = [[0; SIZE[0]]; SIZE[1]];
    let mut gravity; // frames per tile
    'running: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                _ => {}
            };
        }

        if events.keyboard_state().is_scancode_pressed(Scancode::Up) && piece.id != 4 {
            rotate = 1;
        }
        if events.keyboard_state().is_scancode_pressed(Scancode::RCtrl) && piece.id != 4 && rotate >= 0 {
            rotate -= 1;
        }
        if rotate != 0 && rotate_ticks == 0 {
            rotate_ticks = 20;
        } else if rotate == 0 {
            rotate_ticks = 0;
        }
        if events.keyboard_state().is_scancode_pressed(Scancode::Left) {
            shift = -1;
        }
        if events.keyboard_state().is_scancode_pressed(Scancode::Right) && shift <= 0 {
            shift += 1;
        }
        if shift != 0 && shift_ticks == 0 {
            shift_ticks = 10;
        } else if shift == 0 {
            shift_ticks = 0;
        }
        if events.keyboard_state().is_scancode_pressed(Scancode::Down) {
            gravity = 6;
        } else {
            gravity = 48;
        }

        if time.elapsed().as_nanos() >= 100_000_000 / 6 {
            time = Instant::now();

            if ticks % gravity == 0 && ticks > 0 {
                position[1] += 1;
            }

            if shift_ticks > 0 {
                if shift_ticks == 10 {
                    position[0] += shift;
                }
                shift = 0;
                shift_ticks -= 1;
            }

            if rotate_ticks > 0 {
                if rotate == 1 && rotate_ticks == 20 {
                    rotate_cw(&mut piece);
                }
                if rotate == -1 && rotate_ticks == 20 {
                    rotate_ccw(&mut piece);
                }
                rotate = 0;
                rotate_ticks -= 1;
            }

            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.clear();

            canvas.set_draw_color(Color::RGB(0, 0, 255));
            for point in piece.points.iter() {
                canvas.fill_rect(Rect::new((position[0] + point[0]) * i32::from(TILE_SIZE), (position[1] + point[1]) * i32::from(TILE_SIZE), TILE_SIZE.into(), TILE_SIZE.into())).unwrap();
            }

            canvas.present();
            
            ticks += 1;
        }
    }
}
