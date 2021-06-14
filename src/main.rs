// #![warn(missing_docs)]

use core::str;
use std::process::Command;

use clap::{crate_authors, crate_version, App, Arg};

use ff::{mpeg, probe};

/// ScriptX - A Sign Language Bible verse slicer.
fn main() {
    let matches = App::new("ScriptX")
        .author(crate_authors!())
        .about("ScriptX cuts scriptures from the American Sign Language version of the New World Translation Bible.")
        .version(crate_version!())
        .arg(
            Arg::with_name("verse")
                .help("The verse to be cut out")
                .short("v")
                .long("verse")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("file")
                .help("The video file to process")
                .short("f")
                .long("file")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("output_path")
                .help("The path were to save the output")
                .short("o")
                .long("output")
                .takes_value(true)
                .default_value("output.mp4"),
        )
        .get_matches();

    // Check if ffprobe && ffmpeg are installed.
    check_for_ffprobe().expect("Dependency not found:");
    check_for_ffmpeg().expect("Dependency not found");

    let path = matches.value_of("file").unwrap();
    let verse = matches.value_of("verse").unwrap();
    let output_path = matches.value_of("output_path").unwrap();

    let (start_time, end_time) = probe::Root::new(path).verse(verse);

    mpeg::cut(start_time, end_time, path, output_path);
}

/// Returns Ok(()) if the ffprobe is installed otherwise an error is returned.
fn check_for_ffprobe() -> Result<(), &'static str> {
    let ffprobe = Command::new("ffprobe").arg("-version").output().unwrap();
    if ffprobe.status.success() == true {
        Ok(())
    } else {
        Err("ffprobe seems not to be installed on the system. Install ffprobe before continuing.")
    }
}

/// Returns Ok(()) if the ffprobe is installed otherwise an error is returned.
fn check_for_ffmpeg() -> Result<(), &'static str> {
    let ffmpeg = Command::new("ffmpeg").arg("-version").output().unwrap();
    if ffmpeg.status.success() == true {
        Ok(())
    } else {
        Err("ffmpeg seems not to be installed on the system. Install ffmpeg before continuing.")
    }
}
