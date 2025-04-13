# Demo
This demo streams from my webcam and draws on each frame which is then output to the screen.

The video pipelines follows the order shown below:

[v4l2src] -> [videoconvert] -> [appsink] -> [appsrc] -> [videoconvert] -> [autovideosink] 



# My Learnings

While building this demo I took some notes along the way as I learnt more about how GStreamer works.


## Overview of Pipeline Elements

### v4l2 element
- Video for linux 2 driver which allows for video input

### videoconvert element
- Ensures video frames are received and sent in the format that is expected by other pipeline elements.
- Best practice to include right after a source and right before a sink.

### autovideosink element
- Used when you simply want to stream the video source without having access to each frame.

### appsrc element
- Used when you want access to the frame by frame data.

### appsink element
- Used for pushing your own frame data down the pipeline.
