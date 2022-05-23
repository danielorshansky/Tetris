use sdl2::pixels::Color;
use sdl2::event::Event;

struct Piece {
    points: Vec<[i8; 2]>,
    center: usize, // index of the center point
    size: [u8; 2]
}

fn create_piece(piece_id: u8) -> Option<Piece> {
    match piece_id {
        0 => Some(Piece { points: vec![[0, 0], [1, 0], [2, 0], [3, 0]], center: 1, size: [4, 1] }),
        1 => Some(Piece { points: vec![[0, 0], [1, 0], [2, 0], [1, 1]], center: 3, size: [3, 2] }),
        2 => Some(Piece { points: vec![[0, 0], [1, 0], [2, 0], [0, 1]], center: 1, size: [3, 2] }),
        3 => Some(Piece { points: vec![[1, 0], [2, 0], [0, 1], [1, 1]], center: 3, size: [3, 2] }),
        4 => Some(Piece { points: vec![[0, 0], [1, 0], [0, 1], [1, 1]], center: 2, size: [2, 2] }),
        _ => None
    }
}

fn rotate_piece(piece: &mut Piece) { // rotate clockwise, the rotation formula comes from rotation matrices
    let cx = piece.points[piece.center][0]; // center x pos
    let cy = piece.points[piece.center][1]; // center y pos
    for point in piece.points.iter_mut() {
        *point = [point[1] - cy + cx, cx - point[0] + cy];
    }
    piece.size = [piece.size[1], piece.size[0]];
}

fn main() {
    let context = sdl2::init().unwrap();
    let mut canvas = context.video().unwrap().window("Tetris", 480, 840).position_centered().opengl().build().unwrap().into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 255));
    canvas.clear();
    canvas.present();
    
    let mut events = context.event_pump().unwrap();
    'running: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                _ => {}
            }
        }

        canvas.clear();
        canvas.present();
    }
}
