use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Stream,
};

pub struct Audio {
    output_stream: Option<Stream>,
}

impl Audio {
    pub fn init() -> Self {
        let host = cpal::default_host();
        let output_device = host.default_output_device().unwrap();
        let output_config = output_device
            .supported_output_configs()
            .unwrap()
            .next()
            .unwrap()
            .with_max_sample_rate();
        let stream = output_device.build_output_stream(
            &output_config.into(),
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

        Self { output_stream }
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
