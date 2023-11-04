use rand::Rng;
use raylib::prelude::*;

const WINDOW_HEIGHT: i32 = 600;
const WINDOW_WIDTH: i32 = 500;
const CELL_SIZE: i32 = 50;
const MARGIN: i32 = 100;
const BOARD_WIDTH: i32 = WINDOW_WIDTH - MARGIN * 2;
const BOARD_HEIGHT: i32 = WINDOW_HEIGHT;
const CIRCLE_DISPLAY_TIME: f32 = 1.0; // in seconds

struct TimedCircle {
    position: Vector2,
    elapsed_time: f32,
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Beat It with Notes!")
        .build();

    let mut circles: Vec<TimedCircle> = Vec::new();
    let mut rng = rand::thread_rng();
    let mut last_spawn_time = 0.0;

    while !rl.window_should_close() {
        let delta_time = rl.get_frame_time();
        last_spawn_time += delta_time;

        {
            let mut d = rl.begin_drawing(&thread);
            d.clear_background(Color::WHITE);

            // Draw board
            d.draw_rectangle(MARGIN, 0, BOARD_WIDTH, BOARD_HEIGHT, Color::RED);

            // Draw Grid
            for i in 0..BOARD_WIDTH / CELL_SIZE {
                d.draw_rectangle_lines(
                    MARGIN + i * CELL_SIZE,
                    0,
                    CELL_SIZE,
                    BOARD_HEIGHT,
                    Color::BLACK,
                );

                for j in 0..BOARD_HEIGHT / CELL_SIZE {
                    d.draw_rectangle_lines(
                        MARGIN + i * CELL_SIZE,
                        j * CELL_SIZE,
                        CELL_SIZE,
                        CELL_SIZE,
                        Color::BLACK,
                    );
                }
            }

            let mut indices_to_remove = Vec::new();

            for (i, circle) in circles.iter_mut().enumerate() {
                circle.elapsed_time += delta_time;

                if circle.elapsed_time >= CIRCLE_DISPLAY_TIME {
                    indices_to_remove.push(i);
                }
            }

            indices_to_remove.sort_by(|a, b| b.cmp(a)); // Sort in reverse order for removal

            for &index in &indices_to_remove {
                circles.remove(index);
            }

            for circle in &circles {
                d.draw_circle_v(circle.position, 12.5, Color::BLACK);
            }
        }
    }
}
