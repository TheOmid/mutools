mod sound;
mod transform;
mod midi;

#[cfg(test)]
mod tests {
    use crate::transform::{CropTransform,
                           LerpTransform,
                           SoundTransform,
                           PitchTransform };

    use super::sound::{Sound,
                       FileDescriptor,
                       PlaybackManager};
    use super::midi;

    use super::transform::SoundTransformer;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_sound_playback() {
        let sound = Sound::from_file(FileDescriptor::WavFile(
            String::from("./bin/FS_001/BASS/bass_triangle_1.wav")
        ));
        let player = PlaybackManager::new();
        let sound_buffer = sound.clone_buffer();
        player.append(sound_buffer);
        //player.play();
    }

    #[test]
    fn test_crop_transform() {
        let player = PlaybackManager::new();

        let sound_1 = Sound::from_file(FileDescriptor::WavFile(
            String::from("./bin/FS_001/SYNTH/synth_pedal_0.wav")
        ));
        let sound_2 = Sound::from_file(FileDescriptor::WavFile(
            String::from("./bin/FS_001/SYNTH/synth_tape_loop_0.wav")
        ));

        let crop_transform = CropTransform::from_start_end(
                                std::time::Duration::from_millis(500),
                                std::time::Duration::from_millis(3500),
                                44100
                            );
        let cropped_sound_1 = crop_transform.transform(sound_1.clone());
        let cropped_sound_2 = crop_transform.transform(sound_2.clone());

        let lerp_transform = LerpTransform::from(cropped_sound_2.clone(), 1, 2);
        let lerped_sound = lerp_transform.transform(cropped_sound_1.clone());

        player.append(cropped_sound_1.clone_buffer());
        player.append(cropped_sound_2.clone_buffer());
        player.append(lerped_sound.clone_buffer());
        player.play()
    }

    #[test]
    fn test_pitch_transform() {

        let player = PlaybackManager::new();

        let sound_1 = Sound::from_file(FileDescriptor::WavFile(
            String::from("./bin/FS_001/BASS/bass_triangle_1.wav")
        ));

        let pitch_transform = PitchTransform::from_freq(230);

        player.append(sound_1.clone_buffer());
        let pitched_sound= pitch_transform.transform(sound_1.clone());
        player.append(pitched_sound.clone_buffer());
        player.play()
    }

}

