mod trax;

use ::rand::RngExt;
use macroquad::prelude::*;
use rand_pcg::Pcg64;

const SPEED:f32 = 250.0;
const SIZE:f32 = 26.0;
const SPAWNRATE:f32 = 80.0;
#[macroquad::main("Wayfar: Tails of the flame")]
async fn main() {
    rand::srand(miniquad::date::now() as u64);
    let spawn = 1.0 / SPAWNRATE;
    let mut squares:Vec<Shape> = vec![];
    let mut circle = Shape {
        size: SIZE,
        speed: SPEED,
        x: screen_width() / 2.0,
        y: screen_height() / 2.0
    };
    let mut rng = Pcg64::new(0xcafef00d, 0xdeadbeef);
    next_frame().await;
    let mut time = 0.0;
    loop {
        clear_background(DARKPURPLE);
        let delta_time = get_frame_time();
        time += delta_time;
        while time >= spawn {
            if rng.random_range(1 .. 100) >= 95 {
                let size = rng.random_range(16.0 .. 64.0);
                squares.push(Shape {
                    size: size,
                    speed: rng.random_range(SPEED * 0.5 .. SPEED * 1.5), 
                    x: rng.random_range(size / 2.0 .. screen_width() - size / 2.0),
                    y: -size});
            }
            time -= spawn;
        }
        if is_key_down(KeyCode::Left) {
            circle.x -= circle.speed * delta_time;
        }
        if is_key_down(KeyCode::Up) {
            circle.y -= circle.speed * delta_time;
        }
        if is_key_down(KeyCode::Right) {
            circle.x += circle.speed * delta_time;
        }
        if is_key_down(KeyCode::Down) {
            circle.y += circle.speed * delta_time;
        }
        circle.x = clamp(circle.x, 0.0, screen_width());
        circle.y = clamp(circle.y, 0.0, screen_height());
        for square in &mut squares {
            draw_rectangle(square.x - square.size / 2.0, square.y - square.size / 2.0, square.size, square.size, GREEN);
        }
        for square in &mut squares {
            square.y += square.speed * delta_time;
        }
        squares.retain(|square| square.y < screen_height() + square.size + circle.size);
        draw_circle(circle.x, circle.y, circle.size, YELLOW);
        next_frame().await
    }
}

struct Shape {
    size: f32,
    speed: f32,
    x: f32,
    y: f32,
}