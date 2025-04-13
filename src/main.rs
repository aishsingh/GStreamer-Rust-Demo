use gstreamer as gst;
use gstreamer::prelude::*;
use anyhow::Result;

// My webcam device on linux
const WEBCAM: &str = "/dev/video2";

fn main() -> Result<()> {
    
    println!("GStreamer Rust DEMO - Aishwarya Singh");


    // Initialize GStreamer
    gst::init()?;

    // Create a GStreamer pipeline to display webcam feed
    let pipeline = gst::parse_launch(
        format!("v4l2src device={} ! videoconvert ! autovideosink", WEBCAM).as_str()
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

