use image::codecs::qoi;
use minifb::{Key, MouseMode, Window, WindowOptions};
use nalgebra_glm::Vec2;
use once_cell::sync::Lazy;
use std::f32::consts::PI;
use std::sync::Arc;
use std::time::Duration;

mod framebuffer;
use framebuffer::Framebuffer;

mod maze;
use maze::load_maze;

mod player;
use player::{process_events, Player};

mod caster;
use caster::{cast_ray, Intersect};

use gilrs::Gilrs;

mod texture;
use texture::Texture;

static WALL1: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets\\asset1.png")));
static WALL2: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets\\asset2.png")));
static WALL3: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets\\asset10.png")));
static WALL4: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets\\asset13.png")));
static ENEMY: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets\\moai.png")));

fn cell_to_color(cell: char) -> u32 {
    match cell {
        '+' => 0xAA00AA,
        '-' => 0x991199,
        '|' => 0x881188,
        'g' => 0xFF0000,
        _ => 0x000000,
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

    // Calcular el tamaño del sprite basado en la distancia y un factor de escala.
    let sprite_size = (screen_height / sprite_distance) * 70.0;

    // Ajustar la posición inicial basada en el ángulo relativo del sprite y el ángulo del jugador.
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

    r_diff + g_diff + b_diff // Suma simple de diferencias absolutas
}

fn render_enemies(framebuffer: &mut Framebuffer, player: &Player, zbuffer: &mut [f32]) {
    let enemies = vec![Vec2::new(150.0, 400.0)];

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

    for i in 0..framebuffer.width {
        framebuffer.set_current_color(0x383838);

        for j in 0..(framebuffer.height / 2) {
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
                continue; // Asegúrate de que y no exceda la altura del framebuffer
            }
            let ty =
                (y as f32 - stake_top as f32) / (stake_bottom as f32 - stake_top as f32) * 128.0;
            let tx = intersect.tx;
            let color = cell_to_texture_color(intersect.impact, tx as u32, ty as u32);
            let buffer_index = y * framebuffer.width + i;
            if buffer_index >= zbuffer.len() {
                continue; // Asegúrate de que el índice no exceda el tamaño del zbuffer
            }
            if distance < zbuffer[buffer_index] {
                framebuffer.set_current_color(color);
                framebuffer.point(i, y);
                zbuffer[buffer_index] = distance;
            }
        }
    }
}

fn main() {
    let window_width = 1300;
    let window_height = 900;

    let framebuffer_width = 1300;
    let framebuffer_height = 900;

    let frame_delay = Duration::from_millis(0);

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);

    let mut window = Window::new(
        "unnamed-raycaster",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap();

    let mut gilrs = Gilrs::new().unwrap();

    let maze = load_maze("maze.txt");
    let block_size = 100;

    window.set_position(100, 100);
    window.update();

    let initial_mouse_x = window
        .get_mouse_pos(MouseMode::Pass)
        .map_or(0.0, |(x, _)| x as f32);

    framebuffer.set_background_color(0x333355);
    let mut player = Player {
        pos: Vec2::new(150.0, 150.0),
        angle: PI / 3.0,
        fov: PI / 3.0,
        last_mouse_x: initial_mouse_x,
    };

    let mut mode = "3D";

    while window.is_open() {
        if window.is_key_down(minifb::Key::Escape) {
            break;
        }
        if window.is_key_down(Key::M) {
            mode = if mode == "2D" { "3D" } else { "2D" };
        }

        process_events(
            &window,
            &mut player,
            &mut gilrs,
            &maze::load_maze("maze.txt"),
            100,
        );

        framebuffer.clear();

        let mut zbuffer = vec![f32::INFINITY; framebuffer_width * framebuffer_height];

        if mode == "2D" {
            render2d(&mut framebuffer, &player, &maze, block_size);
        } else {
            render3d(&mut framebuffer, &player, &maze, block_size, &mut zbuffer);
            render_enemies(&mut framebuffer, &player, &mut zbuffer);
        }

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer.width, framebuffer.height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}
