use std::path::Path;
use sdl2::mixer::Music;

const MUSIC_VOLUME: i32 = 10;

pub fn load_from_file(music_file: &Path) -> Result<Music, String> {
    Music::from_file(music_file)
}

pub fn play_music(music: &Music){
    Music::set_volume(10);
    music.play(-1);
}
