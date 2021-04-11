use std::path::Path;

use sdl2::mixer::{Chunk, Channel};

const SFX_VOLUME: i32 = 10;

pub fn load_from_file(sound_file: &Path) -> Result<Chunk, String> {
    Chunk::from_file(sound_file)
}

//TODO need to be able to stop already playing sounds and skip some few miliseconds for rollback to work
pub fn play_sound(sound_chunk: &mut Chunk) {
    sound_chunk.set_volume(SFX_VOLUME);
    Channel::all().play(&sound_chunk, 0).unwrap();
}

pub fn play_sound_skip(sound_chunk: &mut Chunk, skip: i32) {
    sound_chunk.set_volume(SFX_VOLUME);
    //fade_in_timed
    Channel::all().fade_in(&sound_chunk, 0, skip).unwrap();
}