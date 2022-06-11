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
    id: u8,
    rotation: i32
}

fn create_piece(piece_id: u8) -> Option<Piece> {
    match piece_id { // positions based around center of piece
        0 => Some(Piece { points: [[-1, 0], [0, 0], [1, 0], [2, 0]], reach: [1, 0, 3, 1], id: 0, rotation: 0 }),
        1 => Some(Piece { points: [[-1, -1], [0, -1], [1, -1], [0, 0]], reach: [1, 1, 2, 1], id: 1, rotation: 0 }),
        2 => Some(Piece { points: [[-1, 0], [0, 0], [1, 0], [-1, 1]], reach: [1, 0, 2, 2], id: 2, rotation: 0 }),
        3 => Some(Piece { points: [[0, -1], [1, -1], [-1, 0], [0, 0]], reach: [1, 1, 2, 1], id: 3, rotation: 0 }),
        4 => Some(Piece { points: [[0, 0], [1, 0], [0, 1], [1, 1]], reach: [0, 0, 2, 2], id: 4, rotation: 0 }),
        _ => None
    }
}

fn rotate_piece(piece: &mut Piece, direction: i32) { // rotate, -1 direction is counterclockwise, 1 is clockwise
    piece.rotation = (piece.rotation + 4 + direction) % 4;
    for point in &mut piece.points {
        *point = [-direction * point[1], direction * point[0]]; // formula comes from rotation matrices, y-coord signs are flipped b/c positive is down
    }
    if direction == 1 {
        piece.reach.rotate_right(1);
    } else {
        piece.reach.rotate_left(1);
    }
}

fn check_rotation(piece: &Piece, position: &[i32; 2], map: &[[u8; SIZE[0]]; SIZE[1]], direction: i32) -> bool { // check if rotation will cause collision
    for point in &piece.points {
        if map[(position[1] + direction * point[0]) as usize][(position[0] - direction * point[1]) as usize] == 1 {
            return false;
        }
    }
    true
}

fn main() {
    let context = sdl2::init().unwrap();
    let mut canvas = context.video().unwrap().window("Tetris", 400, 800).position_centered().opengl().build().unwrap().into_canvas().build().unwrap();

    canvas.clear();
    canvas.present();
    
    let mut rng = rand::thread_rng();
    let mut piece = create_piece(rng.gen_range(0..5)).unwrap();
    let mut position: [i32; 2] = [rng.gen_range(piece.reach[0]..(SIZE[0] - piece.reach[2] + 1)) as i32, piece.reach[1] as i32];
    let mut events = context.event_pump().unwrap();
    let mut time = Instant::now();
    let mut ticks = 0;
    let mut rotate = 0;
    let mut rotate_ticks = 0;
    let mut shift = 0;
    let mut shift_ticks = 0;
    let mut map: [[u8; SIZE[0]]; SIZE[1]] = [[0; SIZE[0]]; SIZE[1]];
    let mut gravity; // frames per tile
    let corrections = [[0, 0], [-1, 0], [-1, -1], [0, -1]]; // adjusting for rendering bug when rotated
    let mut corrected_pos: [i32; 2] = position;
    let mut game_over = false;
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
            gravity = 42;
        }

        if time.elapsed().as_nanos() >= 100_000_000 / 6 { // 60 FPS
            time = Instant::now();

            if shift_ticks > 0 {
                if shift_ticks == 10 {
                    if position[0] - piece.reach[0] as i32 + shift >= 0 && position[0] + piece.reach[2] as i32 + shift <= SIZE[0] as i32 {
                        let mut collided = false;
                        for point in &piece.points {
                            if map[(corrected_pos[1] + point[1]) as usize][(corrected_pos[0] + point[0] + shift) as usize] == 1 {
                                collided = true;
                                shift_ticks = 0;
                                break;
                            }
                        }
                        if !collided {
                            position[0] += shift;
                            shift_ticks -= 1;
                        }
                    } else {
                        shift_ticks = 0;
                    }
                } else {
                    shift_ticks -= 1;
                }
                shift = 0;
            }
            corrected_pos = [position[0] + corrections[piece.rotation as usize][0], position[1] + corrections[piece.rotation as usize][1]];
            
            if rotate_ticks > 0 {
                if rotate == 1 && rotate_ticks == 20 {
                    if position[0] - piece.reach[3] as i32 >= 0 && position[0] + piece.reach[1] as i32 <= SIZE[0] as i32 && position[1] - piece.reach[0] as i32 >= 0 && position[1] + piece.reach[2] as i32 <= SIZE[1] as i32 {
                        if check_rotation(&piece, &corrected_pos, &map, 1) {
                            rotate_piece(&mut piece, 1);
                        }
                    }
                }
                if rotate == -1 && rotate_ticks == 20 {
                    if position[0] - piece.reach[1] as i32 >= 0 && position[0] + piece.reach[3] as i32 <= SIZE[0] as i32 && position[1] - piece.reach[2] as i32 >= 0 && position[1] + piece.reach[0] as i32 <= SIZE[1] as i32 {
                        if check_rotation(&piece, &corrected_pos, &map, -1) {
                            rotate_piece(&mut piece, -1);
                        }
                    }
                }
                rotate = 0;
                rotate_ticks -= 1;
            }
            corrected_pos = [position[0] + corrections[piece.rotation as usize][0], position[1] + corrections[piece.rotation as usize][1]];

            let mut collided = false;
            let dead = position[1] + piece.reach[3] as i32 >= SIZE[1] as i32;
            if !dead {
                for point in &piece.points {
                    if map[(corrected_pos[1] + point[1] + 1) as usize][(corrected_pos[0] + point[0]) as usize] == 1 {
                        collided = true;
                        break;
                    }
                }
            }
            if dead || collided {
                gravity = 12;
            }
            
            if ticks % gravity == 0 && ticks > 0 {
                if dead || collided {
                    for point in &piece.points {
                        if corrected_pos[1] + point[1] > 0 {
                            map[(corrected_pos[1] + point[1]) as usize][(corrected_pos[0] + point[0]) as usize] = 1;
                        } else {
                            game_over = true;
                            break;
                        }
                    }
                    let mut rows: Vec<usize> = vec![];
                    for i in 0..SIZE[1] {
                        if map[i] == [1; SIZE[0]] {
                            let mut distance = 0;
                            if rows.is_empty() {
                                map[i] = [0; SIZE[0]];
                            } else {
                                for row in rows.iter_mut().rev() {
                                    println!("2: {}", distance);
                                    map[i - distance] = map[*row];
                                    map[*row] = [0; SIZE[0]];
                                    *row = i - distance;
                                    distance += 1;
                                }
                            }
                        } else if map[i] != [0; SIZE[0]] {
                            rows.push(i);
                        }
                    }
                    piece = create_piece(rng.gen_range(0..5)).unwrap();
                    position = [rng.gen_range(piece.reach[0]..(SIZE[0] - piece.reach[2] + 1)) as i32, piece.reach[1] as i32];
                    ticks = 0;
                    shift_ticks = 0;
                    rotate_ticks = 0;
                    rotate = 0;
                    shift = 0;
                } else {
                    position[1] += 1;
                    ticks += 1;
                }
            } else {
                ticks += 1
            }
            corrected_pos = [position[0] + corrections[piece.rotation as usize][0], position[1] + corrections[piece.rotation as usize][1]];

            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.clear();

            canvas.set_draw_color(Color::RGB(0, 0, 255));
            for point in &piece.points {
                canvas.fill_rect(Rect::new((corrected_pos[0] + point[0]) * TILE_SIZE as i32, (corrected_pos[1] + point[1]) * TILE_SIZE as i32, TILE_SIZE as u32, TILE_SIZE as u32)).unwrap();
            }
            let mut empty: bool;
            for y in (0..SIZE[1]).rev() {
                empty = true;
                for x in 0..SIZE[0] {
                    if map[y][x] == 1 {
                        canvas.fill_rect(Rect::new(x as i32 * TILE_SIZE as i32, y as i32 * TILE_SIZE as i32, TILE_SIZE as u32, TILE_SIZE as u32)).unwrap();
                        empty = false;
                    }
                }
                if empty {
                    break;
                }
            }
            canvas.present();

            if game_over {
                break 'running;
            }
        }
    }
}
