use minifb::{Key, KeyRepeat, MouseMode, Window, WindowOptions};
use nalgebra_glm::Vec2;
use once_cell::sync::Lazy;
use rodio::Source;
use rodio::{OutputStream, Sink};
use std::f32::consts::PI;
use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;
use std::time::{Duration, Instant};

mod framebuffer;
use framebuffer::Framebuffer;

mod maze;
use maze::load_maze;

mod player;
use player::{process_events, Player};

mod caster;
use caster::cast_ray;

use gilrs::Gilrs;

mod texture;
use texture::Texture;

mod sfx;

static WALL1: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets\\asset7.png")));
static WALL2: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets\\asset2.png")));
static WALL3: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets\\asset10.png")));
static WALL4: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets\\asset13.png")));
static SKY: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets\\sky.jpg")));
static ENEMY: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets\\moai.png")));
static mut GAME_WON: bool = false;

pub fn trigger_win_condition() {
    unsafe {
        GAME_WON = true;
    }
}

fn cell_to_color(cell: char) -> u32 {
    match cell {
        '+' | '-' | '|' => 0x333333,
        ' ' => 0xAAAAAA,
        'g' => 0xFF0000,
        _ => 0xAAAAAA,
    }
}

fn cell_to_texture_color(cell: char, tx: u32, ty: u32) -> u32 {
    match cell {
        '+' => WALL4.get_pixel_color(tx, ty),
        '-' => WALL3.get_pixel_color(tx, ty),
        '|' => WALL2.get_pixel_color(tx, ty),
        'g' => WALL1.get_pixel_color(tx, ty),
        _ => 0x000000,
    }
}

fn draw_cell(framebuffer: &mut Framebuffer, xo: usize, yo: usize, block_size: usize, cell: char) {
    for x in xo..xo + block_size {
        for y in yo..yo + block_size {
            if cell != ' ' {
                let color = cell_to_color(cell);
                framebuffer.set_current_color(color);
                framebuffer.point(x, y);
            }
        }
    }
}

fn render2d(
    framebuffer: &mut Framebuffer,
    player: &Player,
    maze: &Vec<Vec<char>>,
    block_size: usize,
) {
    for row in 0..maze.len() {
        for col in 0..maze[row].len() {
            draw_cell(
                framebuffer,
                col * block_size,
                row * block_size,
                block_size,
                maze[row][col],
            );
        }
    }
    framebuffer.set_current_color(0x00FF00);
    framebuffer.point(player.pos.x as usize, player.pos.y as usize);

    let num_rays = 100;
    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let angle = player.angle - (player.fov / 2.0) + (player.fov * current_ray);
        cast_ray(framebuffer, &maze, player, angle, block_size, true);
    }
}

fn render_enemy(framebuffer: &mut Framebuffer, player: &Player, pos: &Vec2, zbuffer: &mut [f32]) {
    let sprite_angle = (pos.y - player.pos.y).atan2(pos.x - player.pos.x);
    let sprite_distance = ((player.pos.x - pos.x).powi(2) + (player.pos.y - pos.y).powi(2)).sqrt();

    let screen_height = framebuffer.height as f32;
    let screen_width = framebuffer.width as f32;

    let sprite_size = (screen_height / sprite_distance) * 70.0;

    let relative_angle = sprite_angle - player.angle;
    let start_x = ((relative_angle).tan() * (screen_width / 2.0) / (player.fov / 2.0))
        + (screen_width / 2.0)
        - (sprite_size / 2.0);
    let start_y = (screen_height / 2.0) - (sprite_size / 2.0);

    let end_x = (start_x + sprite_size) as usize;
    let end_y = (start_y + sprite_size) as usize;

    let start_x = start_x.max(0.0) as usize;
    let start_y = start_y.max(0.0) as usize;

    let ignore_color = 0xFF66C4;
    let threshold = 150;

    for x in start_x..end_x {
        for y in start_y..end_y {
            if x >= framebuffer.width || y >= framebuffer.height {
                continue;
            }

            let buffer_index = y * framebuffer.width + x;
            if buffer_index >= zbuffer.len() {
                continue;
            }

            let tx = (((x - start_x) * ENEMY.width as usize / sprite_size as usize) as u32)
                .min(ENEMY.width - 1);
            let ty = (((y - start_y) * ENEMY.height as usize / sprite_size as usize) as u32)
                .min(ENEMY.height - 1);

            let color = ENEMY.get_pixel_color(tx, ty);
            let color_diff = color_distance(color, ignore_color);

            if color_diff > threshold && sprite_distance < zbuffer[buffer_index] {
                framebuffer.set_current_color(color);
                framebuffer.point(x, y);
            }
        }
    }
}

fn color_distance(color1: u32, color2: u32) -> u32 {
    let r1 = (color1 >> 16) & 0xFF;
    let g1 = (color1 >> 8) & 0xFF;
    let b1 = color1 & 0xFF;

    let r2 = (color2 >> 16) & 0xFF;
    let g2 = (color2 >> 8) & 0xFF;
    let b2 = color2 & 0xFF;

    let r_diff = (r1 as i32 - r2 as i32).abs() as u32;
    let g_diff = (g1 as i32 - g2 as i32).abs() as u32;
    let b_diff = (b1 as i32 - b2 as i32).abs() as u32;

    r_diff + g_diff + b_diff
}

fn render_enemies(framebuffer: &mut Framebuffer, player: &Player, zbuffer: &mut [f32]) {
    let enemies = vec![
        Vec2::new(150.0, 400.0),
        Vec2::new(1151.0, 692.0),
        Vec2::new(174.0, 754.0),
        Vec2::new(541.0, 592.0),
        Vec2::new(1151.0, 149.0),
    ];

    for enemy in &enemies {
        render_enemy(framebuffer, &player, enemy, zbuffer)
    }
}

fn render3d(
    framebuffer: &mut Framebuffer,
    player: &Player,
    maze: &Vec<Vec<char>>,
    block_size: usize,
    zbuffer: &mut [f32],
) {
    let num_rays = framebuffer.width;
    let hh = framebuffer.height as f32 / 2.0;

    let texture_upper = &*SKY;

    for i in 0..framebuffer.width {
        for j in 0..(framebuffer.height / 2) {
            let tx = (i % texture_upper.width as usize) as u32;
            let ty = (j % texture_upper.height as usize) as u32;
            let color = texture_upper.get_pixel_color(tx, ty);
            framebuffer.set_current_color(color);
            framebuffer.point(i, j);
        }
        framebuffer.set_current_color(0x717171);
        for j in (framebuffer.height / 2)..framebuffer.height {
            framebuffer.point(i, j);
        }
    }

    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let angle = player.angle - (player.fov / 2.0) + (player.fov * current_ray);
        let intersect = cast_ray(framebuffer, &maze, player, angle, block_size, false);

        let distance = intersect.distance * (angle - player.angle).cos();
        let stake_height = (framebuffer.height as f32 / distance) * 70.0;

        let stake_top = (hh - (stake_height / 2.0)) as usize;
        let stake_bottom = (hh + (stake_height / 2.0)) as usize;

        for y in stake_top..stake_bottom {
            if y >= framebuffer.height {
                continue;
            }
            let ty =
                (y as f32 - stake_top as f32) / (stake_bottom as f32 - stake_top as f32) * 128.0;
            let tx = intersect.tx;
            let color = cell_to_texture_color(intersect.impact, tx as u32, ty as u32);
            let buffer_index = y * framebuffer.width + i;
            if buffer_index >= zbuffer.len() {
                continue;
            }
            if distance < zbuffer[buffer_index] {
                framebuffer.set_current_color(color);
                framebuffer.point(i, y);
                zbuffer[buffer_index] = distance;
            }
        }
    }
}

const SCALE: usize = 5;

fn draw_text(framebuffer: &mut Framebuffer, text: &str, x: usize, y: usize) {
    let mut cursor_x = x;

    for ch in text.chars() {
        draw_char(framebuffer, ch, cursor_x, y);
        cursor_x += 4 * SCALE + 10;
    }
}

fn draw_char(framebuffer: &mut Framebuffer, ch: char, x: usize, y: usize) {
    let bitmap = match ch {
        '0' => [0x7E, 0x99, 0x91, 0x89, 0x7E],
        '1' => [0x00, 0x00, 0xFF, 0x00, 0x00],
        '2' => [0xE2, 0x91, 0x91, 0x91, 0x8E],
        '3' => [0x42, 0x81, 0x89, 0x89, 0x76],
        '4' => [0x18, 0x14, 0x92, 0xFF, 0x90],
        '5' => [0x4F, 0x89, 0x89, 0x89, 0x71],
        '6' => [0x7E, 0x89, 0x89, 0x89, 0x72],
        '7' => [0x01, 0xE1, 0x11, 0x09, 0x07],
        '8' => [0x76, 0x89, 0x89, 0x89, 0x76],
        '9' => [0x4E, 0x91, 0x91, 0x91, 0x7E],
        'A' => [0x7E, 0x11, 0x11, 0x11, 0x7E],
        'C' => [0x7E, 0x81, 0x81, 0x81, 0x42],
        'D' => [0xFF, 0x81, 0x81, 0x42, 0x3C],
        'E' => [0xFF, 0x91, 0x91, 0x91, 0x81],
        'F' => [0xFF, 0x09, 0x09, 0x01, 0x00],
        'N' => [0xFF, 0x10, 0x20, 0x40, 0xFF],
        'P' => [0xFF, 0x11, 0x11, 0x11, 0x0E],
        'R' => [0xFF, 0x19, 0x29, 0x49, 0x86],
        'S' => [0x8E, 0x91, 0x91, 0x91, 0x62],
        'T' => [0x01, 0x01, 0xFF, 0x01, 0x01],
        'U' => [0x7F, 0x80, 0x80, 0x80, 0x7F],
        'W' => [0xFF, 0x40, 0x20, 0x40, 0xFF],
        'M' => [0xFF, 0x02, 0x04, 0x02, 0xFF],
        'O' => [0x7E, 0x81, 0x81, 0x81, 0x7E],
        'Y' => [0x07, 0x08, 0xF0, 0x08, 0x07],
        'L' => [0xFF, 0x80, 0x80, 0x80, 0x80],
        '!' => [0x00, 0x00, 0xCF, 0x00, 0x00],
        '-' => [0x08, 0x08, 0x08, 0x08, 0x08],
        '\'' => [0x00, 0x07, 0x00, 0x00, 0x00],
        ' ' => [0x00, 0x00, 0x00, 0x00, 0x00],
        _ => [0x00, 0x00, 0x00, 0x00, 0x00],
    };

    for (i, &byte) in bitmap.iter().enumerate() {
        for bit in 0..8 {
            if byte & (1 << bit) != 0 {
                draw_pixel(framebuffer, x + i * SCALE, y + bit * SCALE, SCALE);
            }
        }
    }
}

fn draw_pixel(framebuffer: &mut Framebuffer, x: usize, y: usize, scale: usize) {
    for dx in 0..scale {
        for dy in 0..scale {
            framebuffer.set_current_color(0xFFFFFF);
            framebuffer.point(x + dx, y + dy);
        }
    }
}

fn draw_minimap(
    framebuffer: &mut Framebuffer,
    player: &Player,
    maze: &Vec<Vec<char>>,
    minimap_scale: usize,
    minimap_x: usize,
    minimap_y: usize,
    block_size: usize,
) {
    for (y, row) in maze.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            let color = match cell {
                '+' | '-' | '|' => 0x333333,
                ' ' => 0xAAAAAA,
                'g' => 0xFF0000,
                _ => 0xAAAAAA,
            };
            for dy in 0..minimap_scale {
                for dx in 0..minimap_scale {
                    framebuffer.set_current_color(color);
                    framebuffer.point(
                        minimap_x + x * minimap_scale + dx,
                        minimap_y + y * minimap_scale + dy,
                    );
                }
            }
        }
    }

    let player_x = (player.pos.x as usize * minimap_scale / block_size) + minimap_x;
    let player_y = (player.pos.y as usize * minimap_scale / block_size) + minimap_y;
    let player_size = 6;
    framebuffer.set_current_color(0xFF0000);
    for dx in 0..player_size {
        for dy in 0..player_size {
            framebuffer.point(
                player_x + dx - player_size / 2,
                player_y + dy - player_size / 2,
            );
        }
    }
}

fn main() {
    let window_width = 1300;
    let window_height = 900;

    let framebuffer_width = 1300;
    let framebuffer_height = 900;

    let frame_delay = Duration::from_millis(2);

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);

    let (stream, stream_handle) =
        OutputStream::try_default().expect("Failed to get default output stream");

    let mut background_music_sink = Sink::try_new(&stream_handle).expect("Failed to create a sink");
    let file = File::open("assets/bg_music.mp3").expect("Failed to open music file");
    let source = rodio::Decoder::new(BufReader::new(file)).expect("Failed to decode music file");
    background_music_sink.append(source.repeat_infinite());
    background_music_sink.set_volume(0.3);

    let mut window = Window::new(
        "Unnamed Raycaster - Press 'S' to Start",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .expect("Failed to create window");

    let mut gilrs = Gilrs::new().unwrap();

    let maze = load_maze("maze.txt");
    let block_size = 100;

    window.set_position(100, 100);

    let initial_mouse_x = window
        .get_mouse_pos(MouseMode::Pass)
        .map_or(0.0, |(x, _)| x as f32);

    framebuffer.set_background_color(0xAAAAAA);

    let mut player = Player {
        pos: Vec2::new(150.0, 150.0),
        angle: PI / 3.0,
        fov: PI / 3.0,
        last_mouse_x: initial_mouse_x,
    };

    let mut mode = "3D";
    let mut in_start_screen = true;

    while window.is_open() && in_start_screen {
        framebuffer.clear();
        draw_text(&mut framebuffer, "WELCOME TO UNNAMED RAYCASTER!", 225, 400);
        draw_text(&mut framebuffer, "PRESS 'S' TO START!", 375, 460);
        window
            .update_with_buffer(&framebuffer.buffer, framebuffer.width, framebuffer.height)
            .expect("Failed to update window");

        if window.is_key_down(Key::S) {
            in_start_screen = false;
        }
    }

    while window.is_open() {
        let current_time = Instant::now();

        if !unsafe { GAME_WON } {
            process_events(
                &window,
                &mut player,
                &mut gilrs,
                &maze,
                block_size,
                &stream_handle,
            );
        }

        framebuffer.clear();
        let mut zbuffer = vec![f32::INFINITY; framebuffer_width * framebuffer_height];

        let minimap_x = framebuffer.width - 300;
        let minimap_y = framebuffer.height - 200;

        if unsafe { GAME_WON } {
            draw_text(&mut framebuffer, "YOU WON!", 500, 475);
        } else {
            if mode == "2D" {
                render2d(&mut framebuffer, &player, &maze, block_size);
            } else {
                render3d(&mut framebuffer, &player, &maze, block_size, &mut zbuffer);
                render_enemies(&mut framebuffer, &player, &mut zbuffer);
                draw_minimap(
                    &mut framebuffer,
                    &player,
                    &maze,
                    20,
                    minimap_x,
                    minimap_y,
                    block_size,
                );
            }
        }

        let fps = calculate_fps(current_time);

        let text_x = framebuffer.width - 250;
        draw_text(&mut framebuffer, &format!("FPS: {}", fps), text_x, 20);
        window
            .update_with_buffer(&framebuffer.buffer, framebuffer.width, framebuffer.height)
            .expect("Failed to update window");

        if window.is_key_down(Key::Escape)
            || unsafe { GAME_WON && window.is_key_pressed(Key::Enter, KeyRepeat::No) }
        {
            break;
        }

        if window.is_key_down(Key::M) {
            mode = if mode == "2D" { "3D" } else { "2D" };
        }

        std::thread::sleep(frame_delay);
    }
}

fn calculate_fps(start_time: Instant) -> u32 {
    let elapsed = start_time.elapsed();
    if elapsed.as_secs_f32() == 0.0 {
        0
    } else {
        (1.0 / elapsed.as_secs_f32()) as u32
    }
}
