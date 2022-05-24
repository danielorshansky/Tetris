use rand::Rng;
use std::time::Instant;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

const TILE_SIZE: u8 = 40;
const SIZE: [i32; 2] = [10, 20];

struct Piece {
    points: Vec<[i32; 2]>,
    reach: [i32; 4], // left, up, right, down; how many pieces extend from "center" point
    id: u8
}

fn create_piece(piece_id: u8) -> Option<Piece> {
    match piece_id {
        0 => Some(Piece { points: vec![[-1, 0], [0, 0], [1, 0], [2, 0]], reach: [1, 0, 3, 1], id: 0 }),
        1 => Some(Piece { points: vec![[-1, -1], [0, -1], [1, -1], [0, 0]], reach: [1, 1, 2, 1], id: 1 }),
        2 => Some(Piece { points: vec![[-1, 0], [0, 0], [1, 0], [-1, 1]], reach: [1, 0, 2, 2], id: 2 }),
        3 => Some(Piece { points: vec![[0, -1], [1, -1], [-1, 0], [0, 0]], reach: [1, 1, 2, 2], id: 3 }),
        4 => Some(Piece { points: vec![[0, 0], [1, 0], [0, 1], [1, 1]], reach: [0, 0, 2, 2], id: 4 }),
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
    let mut position: [i32; 2] = [rng.gen_range(piece.reach[0]..(SIZE[0] - piece.reach[2] + 1)), piece.reach[1]];
    
    let mut events = context.event_pump().unwrap();
    let mut time = Instant::now();
    let mut ticks = 0;
    let mut rotate = 0;
    let mut rotate_ticks = 0;
    'running: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    if piece.id != 4 && rotate_ticks == 0 {
                        rotate = 1;
                        rotate_ticks = 20;
                    }
                },
                Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                    if piece.id != 4 && rotate_ticks == 0 {
                        rotate = 2;
                        rotate_ticks = 20;
                    }
                },
                Event::KeyUp { keycode: Some(Keycode::S), .. } | Event::KeyUp { keycode: Some(Keycode::D), .. } => rotate_ticks = 0,
                _ => {}
            };
        }

        if time.elapsed().as_nanos() >= 100_000_000 / 6 {
            time = Instant::now();

            if ticks % 48 == 0 && ticks > 0 {
                position[1] += 1;
            }

            if rotate_ticks > 0 {
                if rotate == 1 && rotate_ticks == 20 {
                    rotate_cw(&mut piece);
                    rotate = 0;
                }
                if rotate == 2 && rotate_ticks == 20 {
                    rotate_ccw(&mut piece);
                    rotate = 0;
                }
                rotate_ticks -= 1;
            }

            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.clear();

            canvas.set_draw_color(Color::RGB(0, 0, 255));
            for point in piece.points.iter() {
                canvas.fill_rect(sdl2::rect::Rect::new((position[0] + point[0]) * i32::from(TILE_SIZE), (position[1] + point[1]) * i32::from(TILE_SIZE), TILE_SIZE.into(), TILE_SIZE.into())).unwrap();
            }

            canvas.present();
            
            ticks += 1;
        }
    }
}
