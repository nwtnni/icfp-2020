use std::cmp;
use std::env;
use std::fmt::Write as _;
use std::fs;
use std::rc::Rc;
use std::time;

use minifb::Key;
use minifb::KeyRepeat;
use minifb::MouseButton;
use minifb::MouseMode;
use minifb::Window;
use minifb::WindowOptions;

use icfp::ast::Atom;
use icfp::ast::AtomCache;
use icfp::ast::Exp;

const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;

fn main() -> anyhow::Result<()> {

    env_logger::init();

    let client = icfp::Client::new()?;

    let path = env::var("ICFP_PROTOCOL")
        .unwrap_or_else(|_| String::from("data/galaxy.txt"));

    let file = fs::read_to_string(path)?;
    let tokens = icfp::lex(&file);
    let protocol = icfp::parse::interaction_protocol(tokens);

    let mut cache = AtomCache::default();
    let nil = cache.get(Atom::Nil);
    let mut state = Rc::clone(&nil);
    let mut vector = Exp::cons(
        Exp::Atom(Atom::Int(0)),
        Exp::Atom(Atom::Int(0)),
    );

    let mut title_buffer = String::new();
    let mut data_buffer = Vec::new();
    let mut debounce = time::Instant::now();

    let mut current_x = 0i64;
    let mut current_y = 0i64;
    let mut speed = 1;
    let mut scale = 16;
    let mut filter = None;

    let mut window_buffer = vec![0u32; WIDTH * HEIGHT];
    let mut window = Window::new(
        "Galaxy UI",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )?;

    window.limit_update_rate(Some(time::Duration::from_micros(16600)));

    while window.is_open() {

        let in_state = std::mem::replace(&mut state, Rc::clone(&nil));
        let in_vector = std::mem::replace(&mut vector, Rc::clone(&nil));

        let (out_state, out_data) = icfp::interact(
            &client,
            &protocol,
            &mut cache,
            in_state,
            in_vector,
        );

        data_buffer.clear();
        icfp::draw::multidraw_exp(&out_data, &mut data_buffer);

        redraw(
            &mut window_buffer,
            &data_buffer,
            current_x,
            current_y,
            scale,
            filter,
            &mut window,
        )?;

        let (x, y) = loop {

            let mut dirty = false;

            if debounce.elapsed() > time::Duration::from_millis(250) {
                if window.get_mouse_down(MouseButton::Left) {
                    if let Some((x, y)) = window.get_mouse_pos(MouseMode::Discard) {
                        debounce = time::Instant::now();
                        break (x as i64 / scale + current_x, y as i64 / scale + current_y);
                    }
                }
            }

            if window.is_key_pressed(Key::Escape, KeyRepeat::Yes) {
                return Ok(())
            }

            if window.is_key_pressed(Key::Left, KeyRepeat::Yes)
            || window.is_key_pressed(Key::A, KeyRepeat::Yes) {
                current_x -= speed;
                dirty = true;
            }
            if window.is_key_pressed(Key::Right, KeyRepeat::Yes)
            || window.is_key_pressed(Key::D, KeyRepeat::Yes) {
                current_x += speed;
                dirty = true;
            }

            // Note: inverted Y coordinate
            if window.is_key_pressed(Key::Up, KeyRepeat::Yes)
            || window.is_key_pressed(Key::W, KeyRepeat::Yes) {
                current_y -= speed;
                dirty = true;
            }
            if window.is_key_pressed(Key::Down, KeyRepeat::Yes)
            || window.is_key_pressed(Key::S, KeyRepeat::Yes) {
                current_y += speed;
                dirty = true;
            }

            if window.is_key_pressed(Key::Q, KeyRepeat::Yes) {
                speed = cmp::max(speed, 1);
            }
            if window.is_key_pressed(Key::E, KeyRepeat::Yes) {
                speed += 1;
            }

            if window.is_key_pressed(Key::Key0, KeyRepeat::Yes) { filter = None; dirty = true; }
            if window.is_key_pressed(Key::Key1, KeyRepeat::Yes) { filter = Some(0); dirty = true; }
            if window.is_key_pressed(Key::Key2, KeyRepeat::Yes) { filter = Some(1); dirty = true; }
            if window.is_key_pressed(Key::Key3, KeyRepeat::Yes) { filter = Some(2); dirty = true; }
            if window.is_key_pressed(Key::Key4, KeyRepeat::Yes) { filter = Some(3); dirty = true; }
            if window.is_key_pressed(Key::Key5, KeyRepeat::Yes) { filter = Some(4); dirty = true; }
            if window.is_key_pressed(Key::Key6, KeyRepeat::Yes) { filter = Some(5); dirty = true; }

            if window.is_key_pressed(Key::E, KeyRepeat::Yes) {
                speed += 1;
            }

            if window.is_key_pressed(Key::Minus, KeyRepeat::Yes) {
                scale = cmp::max(scale >> 1, 1);
                dirty = true;
            }
            if window.is_key_pressed(Key::Equal, KeyRepeat::Yes) {
                scale = cmp::min(scale << 1, 32);
                dirty = true;
            }

            title_buffer.clear();
            write!(
                &mut title_buffer,
                "Galaxy Position: ({}, {}) @ Speed {} & Scale {}",
                current_x,
                current_y,
                speed,
                scale,
            )?;
            window.set_title(&title_buffer);

            if dirty {
                redraw(
                    &mut window_buffer,
                    &data_buffer,
                    current_x,
                    current_y,
                    scale,
                    filter,
                    &mut window,
                )?;
            } else {
                window.update();
            }
        };

        let _ = std::mem::replace(&mut state, out_state);
        let _ = std::mem::replace(&mut vector, Exp::cons(
            Exp::Atom(Atom::Int(x)),
            Exp::Atom(Atom::Int(y)),
        ));
    }

    Ok(())
}

fn redraw(
    window_buffer: &mut Vec<u32>,
    data_buffer: &[Vec<(i64, i64)>],
    current_x: i64,
    current_y: i64,
    scale: i64,
    filter: Option<usize>,
    window: &mut Window,
) -> anyhow::Result<()> {
    // Clear window buffer
    window_buffer
        .iter_mut()
        .for_each(|pixel| *pixel = 0);

    let scaled_width = WIDTH as i64 / scale;
    let scaled_height = HEIGHT as i64 / scale;

    // Draw points on GUI
    for (color, frame) in data_buffer.iter().enumerate() {

        // Filter specific frames
        if let Some(filter) = filter {
            if color != filter {
                continue;
            }
        }

        for (x, y) in frame {
            if *x < current_x
            || *x >= current_x + scaled_width
            || *y < current_y
            || *y >= current_y + scaled_height {
                continue;
            }

            let x = (x - current_x) * scale;
            let y = (y - current_y) * scale;

            for dy in 0..scale {
                for dx in 0..scale {
                    let index = ((y + dy) as usize) * WIDTH + ((x + dx) as usize);
                    let shift = (color % 3) * 8;
                    let apply = (((window_buffer[index] >> shift) as u8).saturating_add(127) as u32) << shift;
                    window_buffer[index] |= apply;
                }
            }
        }
    }

    window
        .update_with_buffer(&window_buffer, WIDTH, HEIGHT)
        .map_err(anyhow::Error::from)
}
