use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::Vec2;
use std::f32::consts::PI;
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

fn cell_to_color(cell: char) -> u32 {
    match cell {
        '+' => 0xAA00AA,
        '-' => 0x991199,
        '|' => 0x881188,
        'g' => 0xFF0000,
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

fn render3d(
    framebuffer: &mut Framebuffer,
    player: &Player,
    maze: &Vec<Vec<char>>,
    block_size: usize,
) {
    let num_rays = framebuffer.width;
    let hh = framebuffer.height as f32 / 2.0;

    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let angle = player.angle - (player.fov / 2.0) + (player.fov * current_ray);
        let intersect = cast_ray(framebuffer, &maze, player, angle, block_size, false);

        let stake_height = (framebuffer.height as f32 / intersect.distance) * 70.0;

        let stake_top = (hh - (stake_height / 2.0)) as usize;
        let stake_bottom = (hh + (stake_height / 2.0)) as usize;

        for y in stake_top..stake_bottom {
            let color = cell_to_color(intersect.impact);
            framebuffer.set_current_color(color);
            framebuffer.point(i, y);
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

    framebuffer.set_background_color(0x333355);
    let mut player = Player {
        pos: Vec2::new(150.0, 150.0),
        angle: PI / 3.0,
        fov: PI / 3.0,
    };

    let mut mode = "2D";

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

        if mode == "2D" {
            render2d(&mut framebuffer, &player, &maze, block_size);
        } else {
            render3d(&mut framebuffer, &player, &maze, block_size);
        }

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer.width, framebuffer.height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}
