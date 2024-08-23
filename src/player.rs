use crate::{sfx, trigger_win_condition};
use gilrs::{Axis, Button, EventType, Gilrs};
use minifb::{Key, MouseMode, Window, WindowOptions};
use nalgebra_glm::{length, rotate_vec2, Vec2};
use std::f32::consts::PI;

use crate::maze::{is_blocked, is_goal};

pub struct Player {
    pub pos: Vec2,
    pub angle: f32,
    pub fov: f32,
    pub last_mouse_x: f32,
}

static mut TOTAL_DISTANCE: f32 = 0.0;
const DISTANCE_THRESHOLD: f32 = 30.0;

pub fn process_events(
    window: &Window,
    player: &mut Player,
    gilrs: &mut Gilrs,
    maze: &Vec<Vec<char>>,
    block_size: usize,
    stream_handle: &rodio::OutputStreamHandle,
) {
    let forward = Vec2::new(player.angle.cos(), player.angle.sin());
    let mut moved = false;
    let old_pos = player.pos.clone();

    const MOVE_SPEED_KEYBOARD: f32 = 5.0;
    const ROTATION_SPEED_KEYBOARD: f32 = PI / 25.0;
    const DEAD_ZONE: f32 = 0.5;
    const MOVE_SPEED_CONTROLLER: f32 = 3.0;
    const ROTATION_SPEED_CONTROLLER: f32 = PI / 50.0;
    const ROTATION_SPEED_MOUSE: f32 = PI / 75.0;

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
            moved = true;
        }
        if is_goal(
            maze,
            new_pos.x as usize / block_size,
            new_pos.y as usize / block_size,
        ) {
            trigger_win_condition();
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
            moved = true;
        }
        if is_goal(
            maze,
            new_pos.x as usize / block_size,
            new_pos.y as usize / block_size,
        ) {
            trigger_win_condition();
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
            moved = true;
        }
        if is_goal(
            maze,
            new_pos.x as usize / block_size,
            new_pos.y as usize / block_size,
        ) {
            trigger_win_condition();
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
            moved = true;
        }
        if is_goal(
            maze,
            new_pos.x as usize / block_size,
            new_pos.y as usize / block_size,
        ) {
            trigger_win_condition();
        }
    }

    if let Some((mouse_x, _)) = window.get_mouse_pos(MouseMode::Pass) {
        let delta_x = mouse_x as f32 - player.last_mouse_x;
        player.angle += delta_x * ROTATION_SPEED_MOUSE;
        player.last_mouse_x = mouse_x as f32;
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
                        moved = true;
                    }
                    if is_goal(
                        maze,
                        new_pos.x as usize / block_size,
                        new_pos.y as usize / block_size,
                    ) {
                        trigger_win_condition();
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
                        moved = true;
                    }
                    if is_goal(
                        maze,
                        new_pos.x as usize / block_size,
                        new_pos.y as usize / block_size,
                    ) {
                        trigger_win_condition();
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
                        moved = true;
                    }
                    if is_goal(
                        maze,
                        new_pos.x as usize / block_size,
                        new_pos.y as usize / block_size,
                    ) {
                        trigger_win_condition();
                    }
                }
            }
            _ => {}
        }
    }

    if moved {
        let distance = nalgebra_glm::distance(&old_pos, &player.pos);
        unsafe {
            TOTAL_DISTANCE += distance;
            if TOTAL_DISTANCE >= DISTANCE_THRESHOLD {
                if let Ok(_) = sfx::play_footstep_sound(stream_handle) {
                    TOTAL_DISTANCE = 0.0;
                }
            }
        }
    }
}
