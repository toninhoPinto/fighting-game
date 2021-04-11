use sdl2::mixer::{AUDIO_S16LSB, DEFAULT_CHANNELS, InitFlag as AudioInitFlag, Sdl2MixerContext};
use sdl2::mixer::{open_audio, init};

pub mod audio_player;
pub mod music_player;

pub fn init_sound() -> Result<Sdl2MixerContext, String>{
    let frequency = 44_100;
    let format = AUDIO_S16LSB; // signed 16 bit samples, in little-endian byte order
    let channels = DEFAULT_CHANNELS; // Stereo
    let chunk_size = 1_024;
    open_audio(frequency, format, channels, chunk_size);

    let mixer_context = init(AudioInitFlag::MP3 | AudioInitFlag::FLAC | AudioInitFlag::MOD | AudioInitFlag::OGG);

    sdl2::mixer::allocate_channels(4);

    mixer_context
}