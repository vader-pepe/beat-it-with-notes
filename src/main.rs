use std::ffi::CString;

use rand::Rng;
use raylib::{
    ffi::{
        CloseAudioDevice, InitAudioDevice, LoadMusicStream, LoadSound, PlayMusicStream, PlaySound,
        StopMusicStream, UnloadMusicStream, UnloadSound, UpdateMusicStream,
    },
    prelude::*,
};

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
    // Init audio device and stream music from file
    unsafe {
        InitAudioDevice();
    };
    let file_name =
        CString::new("Hollow-Knight-Remix-Grimm_Nightmare-King.ogg").expect("Failed to open file");
    let music = unsafe { LoadMusicStream(file_name.as_ptr()) };

    unsafe { PlayMusicStream(music) }

    while !rl.window_should_close() {
        if rl.is_key_down(KeyboardKey::KEY_SPACE) {
            unsafe { UpdateMusicStream(music) }
        }

        let delta_time = rl.get_frame_time();
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        d.draw_text("Press SPACE to play the sound!", 10, 10, 20, Color::BLACK);
    }

    unsafe {
        StopMusicStream(music); // Stop music playing
        UnloadMusicStream(music); // Unload music stream
        CloseAudioDevice();
    }
}
