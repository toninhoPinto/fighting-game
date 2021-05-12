use std::path::Path;

use sdl2::mixer::{Channel, Chunk};

pub fn load_from_file(sound_file: &Path) -> Result<Chunk, String> {
    Chunk::from_file(sound_file)
}

pub fn play_sound(sound_chunk: &Chunk) {
    let channel_to_play = Channel::all();
    if !channel_to_play.is_playing(){
        channel_to_play.play(&sound_chunk, 0).unwrap();
    }
}

pub fn play_sound_skip(sound_chunk: &Chunk, skip: i32) {
    Channel::all().fade_in(&sound_chunk, 0, skip).unwrap();
}