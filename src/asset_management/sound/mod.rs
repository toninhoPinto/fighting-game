use sdl2::mixer::{init, open_audio};
use sdl2::mixer::{InitFlag as AudioInitFlag, Sdl2MixerContext, AUDIO_S16LSB, DEFAULT_CHANNELS};

pub mod audio_player;
pub mod music_player;

pub fn init_sound() -> Result<Sdl2MixerContext, String> {
    let frequency = 44_100;
    let format = AUDIO_S16LSB; // signed 16 bit samples, in little-endian byte order
    let channels = DEFAULT_CHANNELS; // Stereo
    let chunk_size = 1_024;
    open_audio(frequency, format, channels, chunk_size).unwrap();

    let mixer_context =
        init(AudioInitFlag::MP3 | AudioInitFlag::FLAC | AudioInitFlag::MOD | AudioInitFlag::OGG);

    sdl2::mixer::allocate_channels(4);

    mixer_context
}
