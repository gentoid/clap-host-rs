use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Stream, StreamConfig,
};

use crate::plugin_host::PluginHost;

pub struct Audio {
    output_stream: Option<Stream>,
    output_stream_config: StreamConfig,
}

impl Audio {
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

        let stream = output_device.build_output_stream(
            &output_stream_config,
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                println!("OUTPUT DATA LENGTH: {}", data.len());
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
        };

        Self {
            output_stream,
            output_stream_config,
        }
    }

    pub fn play(&mut self) {
        if let Some(stream) = &self.output_stream {
            match stream.play() {
                Ok(_) => {
                    println!("SUCCESSFULLY STARTED PLAYING");
                }
                Err(err) => {
                    println!("PLAY ERROR: {:?}", err);
                }
            }
        }
    }
}
