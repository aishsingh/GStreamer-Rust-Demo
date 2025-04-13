use gstreamer as gst;
use gstreamer::prelude::*;

use gstreamer_app as gst_app;
use gstreamer_video as gst_video;

use anyhow::Result;


// The webcam device on a Linux system used as the video source
// Modify to point to your camera
const WEBCAM: &str = "/dev/video2";

fn main() -> Result<()> {
    println!("GStreamer Rust Demo - Aishwarya Singh");
    println!("Initializing GStreamer...");
    gst::init()?;

    // Create input and output pipelines
    let (input_pipeline, appsink) = create_input_pipeline()?;
    let (output_pipeline, appsrc, overlay) = create_output_pipeline()?;

    // Configure text overlay and caps
    overlay.set_property("font-desc", "Monospace, 16");

    // Set caps on appsrc to follow a defined format: 
    // BGR 640x480 30fps
    let caps = gst::Caps::builder("video/x-raw")
        .field("format", "BGR")
        .field("width", 640)
        .field("height", 480)
        .field("framerate", gst::Fraction::new(30, 1))
        .build();
    appsrc.set_caps(Some(&caps));

    // Implement callbacks to access frame by frame data
    setup_frame_processing(&appsink, &appsrc, &overlay);

    // Loop until end-of-stream or error
    run_main_loop(&output_pipeline)?;

    cleanup(&input_pipeline, &output_pipeline);
    Ok(())
}

// GStreamer input pipeline to access webcam frames from the source
fn create_input_pipeline() -> Result<(gst::Pipeline, gst_app::AppSink)> {
    let pipeline = gst::parse_launch(&format!(
        "v4l2src device={} ! videoconvert ! video/x-raw,format=BGR,width=640,height=480 ! appsink name=sink",
        WEBCAM
    ))?
    .downcast::<gst::Pipeline>()
    .expect("Expected input pipeline to be a Pipeline");

    let appsink = pipeline
        .by_name("sink")
        .unwrap()
        .downcast::<gst_app::AppSink>()
        .unwrap();

    // Start playing the input pipeline
    pipeline.set_state(gst::State::Playing)?;

    Ok((pipeline, appsink))
}

// GStreamer output pipeline to push webcam frames to the sink
fn create_output_pipeline() -> Result<(gst::Pipeline, gst_app::AppSrc, gst::Element)> {
    let pipeline = gst::parse_launch(
        "appsrc name=source ! videoconvert ! textoverlay name=overlay ! autovideosink"
    )?
    .downcast::<gst::Pipeline>()
    .expect("Expected output pipeline to be a Pipeline");

    let appsrc = pipeline
        .by_name("source")
        .unwrap()
        .downcast::<gst_app::AppSrc>()
        .unwrap();

    let overlay = pipeline
        .by_name("overlay")
        .expect("textoverlay not found");

    // Start playing the output pipeline
    pipeline.set_state(gst::State::Playing)?;

    Ok((pipeline, appsrc, overlay))
}

// Implement callbacks to access frame-by-frame data and forward it to the display
fn setup_frame_processing(
    appsink: &gst_app::AppSink,
    appsrc: &gst_app::AppSrc,
    overlay: &gst::Element,
) {
    let appsrc = appsrc.clone();
    let overlay = overlay.clone();

    // keep track of frames
    let mut frame_count = 0;

    appsink.set_callbacks(
        gst_app::AppSinkCallbacks::builder()
            .new_sample(move |sink| {
                let sample = sink.pull_sample().unwrap();
                let buffer = sample.buffer().unwrap();

                // TODO do ML / drawing with this buffer
                let processed_buffer = buffer.copy_deep().unwrap();

                // Update text overlay with format, resolution, and frame count
                let info = gst_video::VideoInfo::from_caps(sample.caps().unwrap()).unwrap();
                frame_count += 1;
                let text = format!(
                    "format: {}, size: {}x{}, frame: {}",
                    info.format(),
                    info.width(),
                    info.height(),
                    frame_count
                );
                overlay.set_property("text", text);

                // Push the processed frame into the display pipeline
                appsrc.push_buffer(processed_buffer).unwrap();
                Ok(gst::FlowSuccess::Ok)
            })
            .build(),
    );
}

// Loop until the output window is closed or an error occurs
// using the output pipeline only as it will end when the window is closed
// the input pipeline would not detect this change
fn run_main_loop(pipeline: &gst::Pipeline) -> Result<()> {
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
    Ok(())
}

// Shut down both pipelines
fn cleanup(input_pipeline: &gst::Pipeline, output_pipeline: &gst::Pipeline) {
    input_pipeline.set_state(gst::State::Null).unwrap();
    output_pipeline.set_state(gst::State::Null).unwrap();
}

