/**
ffmpeg wrapper

A simple wrapper used to slice specific verses out of video files.

*/

/**
Cuts the video out at specific *start* and *end* times.

# Example
```rust, ignore
use ff:mpeg;
mpeg::cut(23.32342, 50.234234, "nwt_43_Joh_ASL_03_r720P.mp4", "John_3-5.mp4"); // Slices the given video at the given time stamp and outputs to *John_3-5.mp4*.
```
*/
use std::{path::Path, process::Command};

pub fn cut(start_time: f64, end_time: f64, path: &Path, output: &str) {
    dbg!("{}", start_time);
    dbg!("{}", end_time);

    let command = Command::new("ffmpeg")
        .arg("-v")
        .arg("quiet")
        .arg("-ss")
        .arg(start_time.to_string())
        .arg("-i")
        .arg(path)
        .arg("-to")
        .arg(end_time.to_string())
        .arg("-c")
        .arg("copy")
        .arg("-copyts")
        .arg("-y")
        .arg("-avoid_negative_ts")
        .arg("1")
        .arg(output)
        .output()
        .unwrap();

    if !command.status.success() {
        panic!("ffmpeg's exit status was FAILURE");
    }
}
