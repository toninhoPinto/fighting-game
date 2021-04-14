use std::path::Path;

use sdl2::mixer::{Chunk, Channel};

pub fn load_from_file(sound_file: &Path) -> Result<Chunk, String> {
    Chunk::from_file(sound_file)
}

pub fn play_sound(sound_chunk: &Chunk) {
    Channel::all().play(&sound_chunk, 0).unwrap();
}

pub fn play_sound_skip(sound_chunk: &Chunk, skip: i32) {
    Channel::all().fade_in(&sound_chunk, 0, skip).unwrap();
}