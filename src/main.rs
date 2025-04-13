use gstreamer as gst;
use gstreamer::prelude::*;
use anyhow::Result;

// The webcam device on my linux system which will be used as the video source
const WEBCAM: &str = "/dev/video2";

fn main() -> Result<()> {
    
    println!("GStreamer Rust DEMO - Aishwarya Singh");
    println!("Initializing GStreamer...");
    gst::init()?;


    // GStreamer input pipeline to access webcam frames from the source
    let input_pipeline = gst::parse_launch(&format!("v4l2src device={} ! videoconvert ! video/x-raw,format=BGR ! appsink name=sink", WEBCAM)).unwrap();

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



    // Set caps on appsrc to follow a defined format
    // BGR 640x480 30fps
    let caps = gst::Caps::builder("video/x-raw")
        .field("format", "BGR")
        .field("width", 640)
        .field("height", 480)
        .field("framerate", gst::Fraction::new(30, 1))
        .build();
    appsrc.set_caps(Some(&caps));


    // Implement callbacks to access frame by frame data
    appsink.set_callbacks(
        gst_app::AppSinkCallbacks::builder()
            .new_sample(move |sink| {
                let sample = sink.pull_sample().unwrap();
                let buffer = sample.buffer().unwrap();

                // Copy buffer and do ML / drawing
                let mut processed_buffer = buffer.copy(); 
                // ⬆️ Replace this with actual frame processing

                appsrc.push_buffer(processed_buffer).unwrap();
                Ok(gst::FlowSuccess::Ok)
            })
            .build(),
    );


    // Clean up
    input_pipeline.set_state(gst::State::Null)?;
    output_pipeline.set_state(gst::State::Null)?;
    Ok(())
}

