use gilrs::{Axis, Button, EventType, Gilrs};
use minifb::{Key, Window};
use nalgebra_glm::{length, rotate_vec2, Vec2};
use std::f32::consts::PI;

use crate::maze::is_blocked;

pub struct Player {
    pub pos: Vec2,
    pub angle: f32,
    pub fov: f32,
}

pub fn process_events(
    window: &Window,
    player: &mut Player,
    gilrs: &mut Gilrs,
    maze: &Vec<Vec<char>>,
    block_size: usize,
) {
    const MOVE_SPEED_KEYBOARD: f32 = 10.0;
    const ROTATION_SPEED_KEYBOARD: f32 = PI / 25.0;
    const DEAD_ZONE: f32 = 0.5;
    const MOVE_SPEED_CONTROLLER: f32 = 6.0;
    const ROTATION_SPEED_CONTROLLER: f32 = PI / 50.0;

    let forward = Vec2::new(player.angle.cos(), player.angle.sin());

    if window.is_key_down(Key::A) {
        player.angle -= ROTATION_SPEED_KEYBOARD;
    }

    if window.is_key_down(Key::D) {
        player.angle += ROTATION_SPEED_KEYBOARD;
    }

    if window.is_key_down(Key::W) {
        let move_vec = forward * MOVE_SPEED_KEYBOARD;
        let new_pos = player.pos + move_vec;
        if !is_blocked(
            maze,
            new_pos.x as usize / block_size,
            new_pos.y as usize / block_size,
        ) {
            player.pos = new_pos;
        }
    }

    if window.is_key_down(Key::S) {
        let move_vec = forward * -MOVE_SPEED_KEYBOARD;
        let new_pos = player.pos + move_vec;
        if !is_blocked(
            maze,
            new_pos.x as usize / block_size,
            new_pos.y as usize / block_size,
        ) {
            player.pos = new_pos;
        }
    }

    if window.is_key_down(Key::Left) {
        player.angle -= ROTATION_SPEED_KEYBOARD;
    }

    if window.is_key_down(Key::Right) {
        player.angle += ROTATION_SPEED_KEYBOARD;
    }

    if window.is_key_down(Key::Up) {
        let move_vec = forward * MOVE_SPEED_KEYBOARD;
        let new_pos = player.pos + move_vec;
        if !is_blocked(
            maze,
            new_pos.x as usize / block_size,
            new_pos.y as usize / block_size,
        ) {
            player.pos = new_pos;
        }
    }

    if window.is_key_down(Key::Down) {
        let move_vec = forward * -MOVE_SPEED_KEYBOARD;
        let new_pos = player.pos + move_vec;
        if !is_blocked(
            maze,
            new_pos.x as usize / block_size,
            new_pos.y as usize / block_size,
        ) {
            player.pos = new_pos;
        }
    }

    while let Some(event) = gilrs.next_event() {
        match event.event {
            EventType::ButtonPressed(button, _) => match button {
                Button::DPadLeft => player.angle -= ROTATION_SPEED_CONTROLLER,
                Button::DPadRight => player.angle += ROTATION_SPEED_CONTROLLER,
                Button::DPadUp => {
                    let move_vec = forward * MOVE_SPEED_CONTROLLER;
                    let new_pos = player.pos + move_vec;
                    if !is_blocked(
                        maze,
                        new_pos.x as usize / block_size,
                        new_pos.y as usize / block_size,
                    ) {
                        player.pos = new_pos;
                    }
                }
                Button::DPadDown => {
                    let move_vec = forward * -MOVE_SPEED_CONTROLLER;
                    let new_pos = player.pos + move_vec;
                    if !is_blocked(
                        maze,
                        new_pos.x as usize / block_size,
                        new_pos.y as usize / block_size,
                    ) {
                        player.pos = new_pos;
                    }
                }
                _ => {}
            },
            EventType::AxisChanged(Axis::LeftStickX, value, _) => {
                if value > DEAD_ZONE || value < -DEAD_ZONE {
                    player.angle += value * ROTATION_SPEED_CONTROLLER;
                }
            }
            EventType::AxisChanged(Axis::LeftStickY, value, _) => {
                if value > DEAD_ZONE || value < -DEAD_ZONE {
                    let move_vec = forward * (value * MOVE_SPEED_CONTROLLER);
                    let new_pos = player.pos + move_vec;
                    if !is_blocked(
                        maze,
                        new_pos.x as usize / block_size,
                        new_pos.y as usize / block_size,
                    ) {
                        player.pos = new_pos;
                    }
                }
            }
            _ => {}
        }
    }
}
