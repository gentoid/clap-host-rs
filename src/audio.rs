use cpal::ChannelCount;
use rtrb::{Consumer, Producer, RingBuffer};

pub struct Audio {
    rb_rx: Consumer<f32>,
}

impl Audio {
    pub fn init(channels: ChannelCount) -> (Self, Producer<f32>) {
        let (rb_tx, rb_rx) = RingBuffer::new(channels as usize * 44100);

        (Self { rb_rx }, rb_tx)
    }

    // pub fn play(&mut self) {
    //     if let Some(stream) = &self.output_stream {
    //         match stream.play() {
    //             Ok(_) => {
    //                 println!("SUCCESSFULLY STARTED PLAYING");
    //             }
    //             Err(err) => {
    //                 println!("PLAY ERROR: {:?}", err);
    //             }
    //         }
    //     }
    // }

    pub fn process(&mut self, output: &mut [f32]) {
        if let Ok(chunk) = self.rb_rx.read_chunk(output.len()) {
            for (index, value) in chunk.into_iter().enumerate() {
                output[index] = value;
            }
        }
    }
}
