use gstreamer as gst;
use gstreamer::prelude::*;
use anyhow::Result;

fn main() -> Result<()> {
    // Initialize GStreamer
    gst::init()?;

    // Create a simple GStreamer pipeline to display webcam feed
    // /dev/video2 is my webcam
    let pipeline = gst::parse_launch(
        "v4l2src device=/dev/video2 ! videoconvert ! autovideosink"
    )?;

    // Start playing the pipeline
    pipeline.set_state(gst::State::Playing)?;

    // Wait until EOS (end-of-stream) or error
    let bus = pipeline.bus().unwrap();
    for msg in bus.iter_timed(gst::ClockTime::NONE) {
        match msg.view() {
            gst::MessageView::Eos(..) => {
                println!("Stream ended.");
                break;
            }
            gst::MessageView::Error(err) => {
                eprintln!("Error: {}", err.error());
                break;
            }
            _ => {}
        }
    }

    // Clean up
    pipeline.set_state(gst::State::Null)?;
    Ok(())
}

