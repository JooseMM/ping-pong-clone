use raylib::{
    drawing::{RaylibDraw, RaylibDrawHandle},
    ffi::Color,
};

use crate::core::{
    ball::Ball,
    constant::{NET_WIDTH, SCORE_FONT_SIZE, SCREEN_HEIGHT, SCREEN_WIDTH},
    player::{Player, RectanglePosition},
};

mod core;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(
            SCREEN_WIDTH.try_into().unwrap(),
            SCREEN_HEIGHT.try_into().unwrap(),
        )
        .title("Ping Pong Trash")
        .build();

    let mut score: u32 = 0;

    rl.set_target_fps(60);

    let mut player = Player::new(RectanglePosition::TopLef);
    let mut ball = Ball::new();

    let mut is_game_over = false;
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        if is_game_over {
            let text = "Game Over!";
            let font_size = 40;
            let text_measure = d.measure_text(text, font_size);

            d.draw_text(
                "Game Over!!",
                SCREEN_WIDTH as i32 / 2 - text_measure / 2,
                SCREEN_HEIGHT as i32 / 2 - font_size / 2,
                font_size,
                Color::WHITE,
            );
            continue;
        }

        draw_net(&mut d);
        draw_score(&mut d, &mut score);

        player.handle_movement(&mut d);
        player.draw(&mut d);

        ball.handle_movement_and_collision(&d, &player, &mut is_game_over, &mut score);
        ball.draw(&mut d);
    }
}

fn draw_score(ctx: &mut RaylibDrawHandle, score: &mut u32) {
    let score = &score.to_string();
    let text_w = ctx.measure_text(score, SCORE_FONT_SIZE as i32);
    let padding = 20;

    ctx.draw_text(
        score,
        SCREEN_WIDTH as i32 / 2 - text_w - padding,
        padding,
        SCORE_FONT_SIZE as i32,
        Color::WHITE,
    );
}

fn draw_net(ctx: &mut RaylibDrawHandle) {
    let gap = 20;
    let height = (SCREEN_WIDTH - gap) / 10;

    let x = (SCREEN_WIDTH / 2 + NET_WIDTH / 2) as i32;
    for i in 0..10 {
        let current_y = (height * i + gap * i) as i32;
        ctx.draw_rectangle(x, current_y, NET_WIDTH as i32, height as i32, Color::WHITE);
    }
}
