

# Learnings

## Pipeline elements

### v4l2 element
- Video for linux 2 driver which allows for video input

### videoconvert element
- Ensures video frames are received and sent in the format that is expected by other pipeline elements.
- Best practice to include right after a source and right before a sink.

### autovideosink element
- Used when you simply want to stream the video source without having access to each frame.

### appsrc
- Used when you want access to the frame by frame data.


### appsink
- Used for pushing your own frame data down the pipeline.
