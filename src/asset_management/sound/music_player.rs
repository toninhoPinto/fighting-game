use sdl2::mixer::Music;
use std::path::Path;

pub fn load_from_file(music_file: &Path) -> Result<Music, String> {
    Music::from_file(music_file)
}

pub fn play_music(music: &Music) {
    Music::set_volume(10);
    music.play(-1).unwrap();
}
