use gstreamer as gst;
use gstreamer_app as gst_app;
use gstreamer_video as gst_video;
use gstreamer::prelude::*;
use anyhow::Result;

// The webcam device on my linux system which will be used as the video source
const WEBCAM: &str = "/dev/video2";

fn main() -> Result<()> {
    
    println!("GStreamer Rust DEMO - Aishwarya Singh");
    println!("Initializing GStreamer...");
    gst::init()?;


    // GStreamer input pipeline to access webcam frames from the source
    let input_pipeline = gst::parse_launch(&format!("v4l2src device={} ! videoconvert ! video/x-raw,format=BGR,width=640,height=480 ! appsink name=sink", WEBCAM))?
        .downcast::<gst::Bin>()
        .expect("Pipeline should be a gst::Bin");

    let appsink = input_pipeline
    .by_name("sink").unwrap()
    .downcast::<gst_app::AppSink>().unwrap();


    // GStreamer output pipeline to push webcam frames to the sink
    let output_pipeline = gst::parse_launch("appsrc name=source ! videoconvert ! autovideosink")?
        .downcast::<gst::Bin>()
        .expect("Pipeline should be a gst::Bin");

    let appsrc = output_pipeline
    .by_name("source")
    .expect("appsrc not found")
    .downcast::<gst_app::AppSrc>()
    .expect("Element is expected to be an AppSrc");


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

                //let info = gst_video::VideoInfo::from_caps(sample.caps().unwrap()).unwrap();
                //println!("Frame received! format: {:?}, size: {}x{}", info.format(), info.width(), info.height());


                appsrc.push_buffer(processed_buffer).unwrap();
                Ok(gst::FlowSuccess::Ok)
            })
            .build(),
    );



    // Loop until EOS (end-of-stream) or error
    // using the output pipeline only as it will end when the window is closed
    // the input pipeline would not detect this change
    let bus = output_pipeline.bus().unwrap();
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
    input_pipeline.set_state(gst::State::Null)?;
    output_pipeline.set_state(gst::State::Null)?;
    Ok(())
}

