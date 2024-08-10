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
            if (cell != ' ') {
                let color = cell_to_color(cell);
                framebuffer.set_current_color(color);
                framebuffer.point(x, y);
            }
        }
    }
}

fn render2d(framebuffer: &mut Framebuffer, player: &Player) {
    let maze = load_maze("maze.txt");
    let block_size = 100;

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
    cast_ray(framebuffer, &maze, player, player.angle, block_size, true);
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

    window.set_position(100, 100);
    window.update();

    framebuffer.set_background_color(0x333355);
    let mut player = Player {
        pos: Vec2::new(150.0, 150.0),
        angle: PI / 3.0,
    };

    while window.is_open() {
        if window.is_key_down(minifb::Key::Escape) {
            break;
        }

        process_events(&window, &mut player);

        framebuffer.clear();

        render2d(&mut framebuffer, &player);

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer.width, framebuffer.height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}
