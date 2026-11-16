use raylib::prelude::*;
use raylib::{drawing::RaylibDrawHandle, ffi::Color};

use crate::core::constant::{
    PLAYER_ACCELERATION, PLAYER_FRICTION, PLAYER_HEIGHT, PLAYER_MAX_SPEED, PLAYER_WIDTH,
    SCREEN_HEIGHT,
};

pub enum RectanglePosition {
    TopLef,
}

pub struct Player {
    body: Rectangle,
    velocity: f32,
    color: Color,
}

impl Player {
    pub fn new(position: RectanglePosition) -> Self {
        let body = Self::generate_rectangle_from_position(position);

        Self {
            velocity: 0.0,
            body,
            color: Color::WHITE,
        }
    }

    pub fn body(&self) -> &Rectangle {
        &self.body
    }

    fn generate_rectangle_from_position(position: RectanglePosition) -> Rectangle {
        match position {
            RectanglePosition::TopLef => Rectangle::new(0.0, 0.0, PLAYER_WIDTH, PLAYER_HEIGHT),
        }
    }

    pub fn handle_movement(&mut self, ctx: &mut RaylibDrawHandle) {
        let dt = ctx.get_frame_time();

        if ctx.is_key_down(KeyboardKey::KEY_S) {
            self.velocity = (self.velocity + PLAYER_ACCELERATION * dt).min(PLAYER_MAX_SPEED);
        } else if ctx.is_key_down(KeyboardKey::KEY_W) {
            self.velocity = (self.velocity - PLAYER_ACCELERATION * dt).max(-PLAYER_MAX_SPEED);
        } else {
            if self.velocity > 0.0 {
                self.velocity = (self.velocity - PLAYER_FRICTION * dt).max(0.0);
            } else if self.velocity < 0.0 {
                self.velocity = (self.velocity + PLAYER_FRICTION * dt).min(0.0);
            }
        }

        self.body.y += self.velocity * dt;

        let max_y = SCREEN_HEIGHT as f32 - self.body.height;
        if self.body.y < 0.0 {
            self.body.y = 0.0;
            self.velocity = 0.0;
        } else if self.body.y > max_y {
            self.body.y = max_y;
            self.velocity = 0.0;
        }
    }

    pub fn draw(&self, ctx: &mut RaylibDrawHandle) {
        ctx.draw_rectangle_rec(self.body, self.color);
    }
}
