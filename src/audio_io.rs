use cpal::{
    traits::{DeviceTrait, HostTrait},
    Stream, StreamConfig,
};
use rtrb::{Producer, RingBuffer};

use crate::audio::Audio;

enum AudioIOMsg {
    NewAudio(Audio),
    NoAudio,
}

pub struct AudioIO {
    output_stream: Stream,
    io_tx: Producer<AudioIOMsg>,
    output_stream_config: StreamConfig,
}

impl AudioIO {
    pub fn init() -> Self {
        let host = cpal::default_host();
        let output_device = host.default_output_device().unwrap();
        let output_stream_config = output_device
            .supported_output_configs()
            .unwrap()
            .next()
            .unwrap()
            .with_max_sample_rate()
            .into();

        let (io_tx, mut io_rx) = RingBuffer::new(32);

        let stream = output_device.build_output_stream(
            &output_stream_config,
            move |output: &mut [f32], _: &cpal::OutputCallbackInfo| {
                println!("OUTPUT DATA LENGTH: {}", output.len());

                let mut audio = None;

                while let Ok(msg) = io_rx.pop() {
                    audio = match msg {
                        AudioIOMsg::NewAudio(new_audio) => Some(new_audio),
                        AudioIOMsg::NoAudio => None,
                    }
                }

                if let Some(audio) = &mut audio {
                    audio.process(output);
                }
            },
            move |err| {
                println!("STREAM ERROR: {:?}", err);
            },
            None,
        );

        let output_stream = match stream {
            Ok(stream) => Some(stream),
            Err(err) => {
                println!("BUILD STREAM ERROR: {:?}", err);
                None
            }
        }
        .unwrap();

        Self {
            output_stream,
            output_stream_config,
            io_tx,
        }
    }

    pub fn new_audio(&mut self, audio: Audio) {
        self.io_tx.push(AudioIOMsg::NewAudio(audio)).unwrap();
    }

    pub fn drop_audio(&mut self) {
        self.io_tx.push(AudioIOMsg::NoAudio).unwrap();
    }
}
