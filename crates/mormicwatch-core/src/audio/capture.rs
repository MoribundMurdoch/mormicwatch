use anyhow::{anyhow, Result};
use cpal::traits::{
    DeviceTrait,
    HostTrait,
    StreamTrait,
};
use crossbeam_channel::Sender;

use crate::audio::levels::{
    linear_to_db,
    peak,
    rms,
};

#[derive(Debug, Clone)]
pub struct AudioFrame {
    pub peak_db: f32,
    pub rms_db: f32,
}

pub fn start_capture(
    tx: Sender<AudioFrame>,
) -> Result<cpal::Stream> {
    let host = cpal::default_host();

    let device = host
        .default_input_device()
        .ok_or_else(|| anyhow!("No input device found"))?;

    let config = device.default_input_config()?;

    let stream = match config.sample_format() {
        cpal::SampleFormat::F32 => {
            build_stream::<f32>(
                &device,
                &config.into(),
                tx,
            )?
        }
        _ => {
            return Err(anyhow!(
                "Unsupported sample format"
            ));
        }
    };

    stream.play()?;

    Ok(stream)
}

fn build_stream<T>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    tx: Sender<AudioFrame>,
) -> Result<cpal::Stream>
where
    T: cpal::SizedSample + Into<f32>,
{
    let stream = device.build_input_stream(
        config,
        move |data: &[T], _| {
            let samples: Vec<f32> =
                data.iter().map(|s| (*s).into()).collect();

            let peak_db =
                linear_to_db(peak(&samples));

            let rms_db =
                linear_to_db(rms(&samples));

            let _ = tx.send(AudioFrame {
                peak_db,
                rms_db,
            });
        },
        move |err| {
            eprintln!("Audio error: {err}");
        },
        None,
    )?;

    Ok(stream)
}