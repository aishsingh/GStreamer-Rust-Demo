use gstreamer as gst;
use gstreamer::prelude::*;
use anyhow::Result;

// The webcam device on my linux system which will be used as the video source
const WEBCAM: &str = "/dev/video2";

fn main() -> Result<()> {
    
    println!("GStreamer Rust DEMO - Aishwarya Singh");

    // Initialize GStreamer
    println!("Initializing GStreamer...");
    gst::init()?;


    // GStreamer input pipeline to access webcam frames from the source
    let input_pipeline = gst::parse_launch(&format!("v4l2src device={} ! videoconvert ! video/x-raw,format=BGR ! appsink name=sink"), WEBCAM).unwrap();

    let appsink = input_pipeline
    .by_name("sink").unwrap()
    .downcast::<gst_app::AppSink>().unwrap();


    // GStreamer output pipeline to push webcam frames to the sink
    let output_pipeline = gst::parse_launch("appsrc name=source ! videoconvert ! autovideosink").unwrap();

    let appsrc = output_pipeline
    .by_name("source").unwrap()
    .downcast::<gst_app::AppSrc>().unwrap();


    // Start playing the pipelines
    input_pipeline.set_state(gst::State::Playing)?;
    output_pipeline.set_state(gst::State::Playing)?;

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

