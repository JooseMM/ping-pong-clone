use raylib::prelude::*;
use raylib::{drawing::RaylibDrawHandle, ffi::Color};

use crate::core::constant::{
    BALL_ACCELERATION, BALL_BONUS_ACCELERATION, BALL_MAX_SPEED, BALL_RADIUS, SCREEN_HEIGHT,
    SCREEN_WIDTH, UNIT_POINT,
};
use crate::core::player::Player;

pub struct Ball {
    position: Vector2,
    velocity: Vector2,
    radius: f32,
    color: Color,
}

impl Ball {
    pub fn new() -> Self {
        let position = Vector2::new(SCREEN_WIDTH as f32 / 2.0, SCREEN_HEIGHT as f32 / 2.0);
        let velocity = Vector2::new(-BALL_ACCELERATION / 2.0, BALL_ACCELERATION as f32 / 2.0);

        Self {
            position,
            velocity,
            radius: BALL_RADIUS,
            color: Color::WHITE,
        }
    }

    pub fn handle_movement_and_collision(
        &mut self,
        ctx: &RaylibDrawHandle,
        player: &Player,
        is_game_over: &mut bool,
        score: &mut u32,
    ) {
        let dt = ctx.get_frame_time();

        self.position += self.velocity * dt;

        if self.velocity.x < 0.0
            && self.position.x - self.radius <= player.body().x + player.body().width
            && self.position.x + self.radius >= player.body().x
            && self.position.y + self.radius >= player.body().y
            && self.position.y - self.radius <= player.body().y + player.body().height
        {
            if self.position.y > player.body().y + player.body().height
                && self.position.x < player.body().x + player.body().width
            {
                self.position.y = player.body().y + player.body().height + self.radius;
                self.velocity.y = -self.velocity.y + BALL_BONUS_ACCELERATION * 2.0;
            } else if self.position.y < player.body().y
                && self.position.x < player.body().x + player.body().width
            {
                self.position.y = player.body().y - self.radius;
                self.velocity.y = -self.velocity.y - BALL_BONUS_ACCELERATION * 2.0;
            } else {
                self.position.x = player.body().x + player.body().width + self.radius;
                self.velocity.x = -self.velocity.x + BALL_BONUS_ACCELERATION * 2.0;
                *score += UNIT_POINT;
            }
        }

        self.velocity = self.velocity.clamp_value(-BALL_MAX_SPEED, BALL_MAX_SPEED);

        if self.position.y - self.radius <= 0.0 {
            self.position.y = self.radius;
            self.velocity.y = -(self.velocity.y - BALL_BONUS_ACCELERATION);
        } else if self.position.y + self.radius >= SCREEN_HEIGHT as f32 {
            self.position.y = SCREEN_HEIGHT as f32 - self.radius;
            self.velocity.y = -(self.velocity.y + BALL_BONUS_ACCELERATION);
        }

        if self.position.x + self.radius >= SCREEN_WIDTH as f32 {
            self.position.x = SCREEN_WIDTH as f32 - self.radius;
            self.velocity.x = -self.velocity.x - BALL_BONUS_ACCELERATION;
        } else if self.position.x - self.radius <= 0.0 {
            *is_game_over = true;
        }
    }

    pub fn draw(&self, ctx: &mut RaylibDrawHandle) {
        ctx.draw_circle_v(self.position, self.radius, self.color);
    }
}
