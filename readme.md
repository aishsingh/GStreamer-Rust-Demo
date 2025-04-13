# Demo
This demo streams from my webcam, processes the format of each frame and displays this info as well as the original frame to the screen.

![Demo](Screencast.gif)

The video pipelines follows the order shown below:
[v4l2src] -> [videoconvert] -> [appsink] -> [appsrc] -> [videoconvert] -> [textoverlay] -> [autovideosink] 


# Optimisations that are possible
- Create a seperate thread to handle the frame processing from data received in each callback from the input pipeline. This would avoid blocking the main GStreamer thread if processing was causing high latency. 
- ensure no copying of buffers
- adjust frame size and framerate to be minimum absolutely neccessary.


# My learnings

While building this demo I took some notes along the way as I learnt more about how GStreamer works.


## Overview of Pipeline Elements

### v4l2 element
- Video for linux 2 driver which acts as a video source

### videoconvert element
- Ensures video frames are received and sent in the format that is expected by other pipeline elements.
- Best practice to include right after a source and right before a sink.
- Setting the format in the cap does not guarantee videoconvert element will follow the specified format.

### autovideosink element
- Used when you simply want to stream the video source without having access to each frame.

### appsrc element
- Used when you want access to the frame by frame data.

### appsink element
- Used for pushing your own frame data down the pipeline.
