use std::f32::consts::PI;

use minifb::{Key, Window};
use nalgebra_glm::Vec2;

pub struct Player {
    pub pos: Vec2,
    pub angle: f32,
}

pub fn process_events(window: &Window, player: &mut Player) {
    const MOVE_SPEED: f32 = 10.0;
    const ROTATION_SPEED: f32 = PI / 25.0;

    if window.is_key_down(Key::A) {
        player.angle -= ROTATION_SPEED;
    }

    if window.is_key_down(Key::D) {
        player.angle += ROTATION_SPEED;
    }

    if window.is_key_down(Key::W) {
        player.pos.x += MOVE_SPEED * player.angle.cos();
        player.pos.y += MOVE_SPEED * player.angle.sin();
    }

    if window.is_key_down(Key::S) {
        player.pos.x -= MOVE_SPEED * player.angle.cos();
        player.pos.y -= MOVE_SPEED * player.angle.sin();
    }
}
