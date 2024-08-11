use gilrs::{Axis, Button, EventType, Gilrs};
use minifb::{Key, Window};
use nalgebra_glm::Vec2;
use std::f32::consts::PI;

pub struct Player {
    pub pos: Vec2,
    pub angle: f32,
}

pub fn process_events(window: &Window, player: &mut Player, gilrs: &mut Gilrs) {
    const MOVE_SPEED_KEYBOARD: f32 = 10.0;
    const ROTATION_SPEED_KEYBOARD: f32 = PI / 25.0;
    const DEAD_ZONE: f32 = 0.5;
    const ROTATION_SPEED_CONTROLLER: f32 = PI / 50.0;
    const MOVE_SPEED_CONTROLLER: f32 = 6.0;

    if window.is_key_down(Key::A) {
        player.angle -= ROTATION_SPEED_KEYBOARD;
    }

    if window.is_key_down(Key::D) {
        player.angle += ROTATION_SPEED_KEYBOARD;
    }

    if window.is_key_down(Key::W) {
        player.pos.x += MOVE_SPEED_KEYBOARD * player.angle.cos();
        player.pos.y += MOVE_SPEED_KEYBOARD * player.angle.sin();
    }

    if window.is_key_down(Key::S) {
        player.pos.x -= MOVE_SPEED_KEYBOARD * player.angle.cos();
        player.pos.y -= MOVE_SPEED_KEYBOARD * player.angle.sin();
    }

    if window.is_key_down(Key::Left) {
        player.angle -= ROTATION_SPEED_KEYBOARD;
    }

    if window.is_key_down(Key::Right) {
        player.angle += ROTATION_SPEED_KEYBOARD;
    }

    if window.is_key_down(Key::Up) {
        player.pos.x += MOVE_SPEED_KEYBOARD * player.angle.cos();
        player.pos.y += MOVE_SPEED_KEYBOARD * player.angle.sin();
    }

    if window.is_key_down(Key::Down) {
        player.pos.x -= MOVE_SPEED_KEYBOARD * player.angle.cos();
        player.pos.y -= MOVE_SPEED_KEYBOARD * player.angle.sin();
    }

    while let Some(event) = gilrs.next_event() {
        match event.event {
            EventType::ButtonPressed(button, _) => match button {
                Button::DPadLeft => player.angle -= ROTATION_SPEED_KEYBOARD,
                Button::DPadRight => player.angle += ROTATION_SPEED_KEYBOARD,
                Button::DPadUp => {
                    player.pos.x += MOVE_SPEED_KEYBOARD * player.angle.cos();
                    player.pos.y += MOVE_SPEED_KEYBOARD * player.angle.sin();
                }
                Button::DPadDown => {
                    player.pos.x -= MOVE_SPEED_KEYBOARD * player.angle.cos();
                    player.pos.y -= MOVE_SPEED_KEYBOARD * player.angle.sin();
                }
                _ => {}
            },
            EventType::AxisChanged(Axis::LeftStickX, value, _) => {
                if value > DEAD_ZONE {
                    player.angle += ROTATION_SPEED_CONTROLLER;
                } else if value < -DEAD_ZONE {
                    player.angle -= ROTATION_SPEED_CONTROLLER;
                }
            }
            EventType::AxisChanged(Axis::RightStickX, value, _) => {
                if value > DEAD_ZONE {
                    player.angle += ROTATION_SPEED_CONTROLLER;
                } else if value < -DEAD_ZONE {
                    player.angle -= ROTATION_SPEED_CONTROLLER;
                }
            }
            EventType::AxisChanged(Axis::LeftStickY, value, _) => {
                if value > DEAD_ZONE {
                    player.pos.x += MOVE_SPEED_CONTROLLER * player.angle.cos();
                    player.pos.y += MOVE_SPEED_CONTROLLER * player.angle.sin();
                } else if value < -DEAD_ZONE {
                    player.pos.x -= MOVE_SPEED_CONTROLLER * player.angle.cos();
                    player.pos.y -= MOVE_SPEED_CONTROLLER * player.angle.sin();
                }
            }
            _ => {}
        }
    }
}
