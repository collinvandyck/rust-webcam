use anyhow::{Context, Result};
use nokhwa::{
    Camera,
    backends::capture::AVFoundationCaptureDevice,
    native_api_backend,
    pixel_format::RgbFormat,
    utils::{CameraIndex, RequestedFormat, RequestedFormatType},
};

fn main() -> Result<()> {
    let be = native_api_backend().context("backend")?;
    println!("got backend");
    let index = CameraIndex::Index(0);
    let requested = RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestFrameRate);
    println!("got requested");
    let mut camera = Camera::new(index, requested).context("build new camera")?;
    println!("got camera");
    Ok(())
}
