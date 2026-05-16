use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::Arc;
use std::sync::Mutex;

pub struct AudioCapture {
    stream: Option<cpal::Stream>,
    buffer: Arc<Mutex<Vec<f32>>>,
}

impl AudioCapture {
    pub fn new() -> Result<Self, String> {
        let host = cpal::default_host();
        let device = host
            .default_input_device()
            .ok_or("No input device found")?;

        let config = cpal::StreamConfig {
            channels: 1,
            sample_rate: 16000,
            buffer_size: cpal::BufferSize::Default,
        };

        let buffer = Arc::new(Mutex::new(Vec::new()));
        let buffer_clone = Arc::clone(&buffer);

        let stream = device
            .build_input_stream(
                &config,
                move |data: &[f32], _: &cpal::InputCallbackInfo| {
                    let mut buf = buffer_clone.lock().unwrap();
                    buf.extend_from_slice(data);
                },
                |err| eprintln!("Stream error: {}", err),
                None,
            )
            .map_err(|e| e.to_string())?;

        stream.play().map_err(|e| e.to_string())?;

        Ok(AudioCapture {
            stream: Some(stream),
            buffer,
        })
    }

    pub fn stop_and_get_buffer(mut self) -> Vec<f32> {
        drop(self.stream.take());
        match Arc::try_unwrap(self.buffer) {
            Ok(mutex) => mutex.into_inner().unwrap(),
            Err(arc) => arc.lock().unwrap().clone(),
        }
    }
}
