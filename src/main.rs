use rand::Rng;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

struct Piece {
    points: Vec<[i32; 2]>,
    origin: [u8; 4] // left, up, right, down
}

fn create_piece(piece_id: u8) -> Option<Piece> {
    match piece_id {
        0 => Some(Piece { points: vec![[-1, 0], [0, 0], [1, 0], [2, 0]], origin: [1, 0, 3, 1] }),
        1 => Some(Piece { points: vec![[-1, -1], [0, -1], [1, -1], [0, 0]], origin: [1, 1, 2, 1] }),
        2 => Some(Piece { points: vec![[-1, 0], [0, 0], [1, 0], [-1, 1]], origin: [1, 0, 2, 2] }),
        3 => Some(Piece { points: vec![[0, -1], [1, -1], [-1, 0], [0, 0]], origin: [1, 1, 1, 2] }),
        4 => Some(Piece { points: vec![[0, -1], [1, -1], [0, 0], [1, 0]], origin: [0, 1, 2, 1] }),
        _ => None
    }
}

fn rotate_cw(piece: &mut Piece) { // rotate clockwise, the rotation formula comes from rotation matrices, note: signs from matrices are flipped b/c positive is down
    for point in piece.points.iter_mut() {
        *point = [-point[1], point[0]];
    }
    piece.origin.rotate_right(1);
}

fn rotate_ccw(piece: &mut Piece) { // rotate counterclockwise, the rotation formula comes from rotation matrices, note: signs from matrices are flipped b/c positive is down
    for point in piece.points.iter_mut() {
        *point = [point[1], -point[0]];
    }
    piece.origin.rotate_left(1);
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut piece = create_piece(rng.gen_range(0..5)).unwrap();

    let context = sdl2::init().unwrap();
    let mut canvas = context.video().unwrap().window("Tetris", 480, 840).position_centered().opengl().build().unwrap().into_canvas().build().unwrap();

    canvas.clear();
    canvas.present();
    
    let mut events = context.event_pump().unwrap();
    'running: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown { keycode: Some(Keycode::S), .. } => rotate_cw(&mut piece),
                Event::KeyDown { keycode: Some(Keycode::D), .. } => rotate_ccw(&mut piece),
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(0, 0, 255));
        for point in piece.points.iter() {
            canvas.fill_rect(sdl2::rect::Rect::new(200 - point[0] * 40, 200 - point[1] * 40, 40, 40)).unwrap();
        }

        canvas.present();
    }
}
