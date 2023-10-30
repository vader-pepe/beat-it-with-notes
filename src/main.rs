use raylib::{
    ffi::{
        BeginDrawing, ClearBackground, CloseWindow, DrawGrid, DrawRectangle, DrawRectangleLines,
        EndDrawing, GetFrameTime, InitWindow, SetTargetFPS, WindowShouldClose,
    },
    prelude::Color,
};
use std::ffi::CString;

const WINDOW_HEIGHT: i32 = 500;
const WINDOW_WIDTH: i32 = 500;
const ROWS: i32 = 25;
const COLS: i32 = 25;
const CELL_WIDTH: i32 = WINDOW_WIDTH / COLS;
const CELL_HEIGHT: i32 = WINDOW_HEIGHT / ROWS;

fn main() {
    unsafe {
        let title = CString::new("Beat it with Notes!").expect("Failed to create title string");

        let _d_t = GetFrameTime() as i32;
        InitWindow(WINDOW_WIDTH, WINDOW_HEIGHT, title.as_ptr());
        SetTargetFPS(30);
        let margin = 100;
        let board_width = WINDOW_WIDTH - margin * 2;
        let board_height = WINDOW_HEIGHT;

        while !WindowShouldClose() {
            BeginDrawing();
            ClearBackground(Color::WHITE.into());
            // Board
            DrawRectangle(margin, 0, board_width, board_height, Color::RED.into());
            let mut i = 0;
            let mut j = 0;
            while i < COLS {
                while j < ROWS {
                    DrawRectangleLines(
                        i * CELL_WIDTH + margin,
                        j * CELL_HEIGHT,
                        CELL_WIDTH,
                        CELL_HEIGHT,
                        Color::BLACK.into(),
                    );
                    j = j + 1;
                }
                i = i + 1;
            }

            EndDrawing();
        }

        CloseWindow();
    }
}
