// #![warn(missing_docs)]

use core::str;
use std::process::Command;

use clap::{crate_authors, crate_version, App, Arg};

use ff::{
    mpeg,
    probe::{self, Root},
};

use crate::custom_errors::ScriptxErrors;

/// ScriptX - A Sign Language Bible verse slicer.
fn main() -> Result<(), ScriptxErrors> {
    let matches = App::new("ScriptX")
        .author(crate_authors!())
        .about("ScriptX cuts scriptures from the American Sign Language version of the New World Translation Bible.")
        .version(crate_version!())
        .arg(
            Arg::with_name("verse")
                .help("The verse to be cut out. A single verse or a range of verses can be cut, ig. 2-5.")
                .short("v")
                .long("verse")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("file")
                .help("The input video file to process.")
                .short("f")
                .long("file")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("output_path")
                .help("The path were to save the output file.")
                .short("o")
                .long("output")
                .takes_value(true)
                .default_value("output.m4v"),
        )
        .get_matches();

    // Todo: Find a better solution to this error message.
    let dependency_error_msg: &str = r#"ScriptX Error: 
        ffprobe and/or ffmpeg was not found on your system. Make sure it is installed.
                    
        Installing with apt:
            $ sudo apt install ffmpeg
                        
"#;

    // Check if ffprobe && ffmpeg are installed.
    match check_for_ffprobe() {
        Ok(_) => (),
        Err(e) => {
            eprint!("{}", dependency_error_msg);
            return Err(e);
        }
    };

    match check_for_ffmpeg() {
        Ok(_) => (),
        Err(e) => {
            eprint!("{}", dependency_error_msg);
            return Err(e);
        }
    };

    let path: &str = matches.value_of("file").unwrap();
    let verse: &str = matches.value_of("verse").unwrap();
    let output_path: &str = matches.value_of("output_path").unwrap();

    let chapters: Root = match probe::Root::new(path).map_err(|_| ScriptxErrors::FileError) {
        Ok(r) => r,
        Err(e) => return Err(e),
    };

    let (start_time, end_time) = chapters.verse(verse);

    mpeg::cut(start_time, end_time, path, output_path);

    Ok(())
}

/// Returns a bool if the ffprobe is installed otherwise an error is returned.
fn check_for_ffprobe() -> Result<bool, ScriptxErrors> {
    let ffprobe = Command::new("ffprobe").arg("-version").output();

    match ffprobe {
        Ok(o) => Ok(o.status.success()),
        _ => {
            return Err(ScriptxErrors::DependencyError);
        }
    }
}

/// Returns bool if the ffprobe is installed otherwise an error is returned.
fn check_for_ffmpeg() -> Result<bool, ScriptxErrors> {
    let ffmpeg = Command::new("ffmpeg").arg("-version").output();

    match ffmpeg {
        Ok(o) => Ok(o.status.success()),
        _ => {
            return Err(ScriptxErrors::DependencyError);
        }
    }
}

/// Custom errors for the ScriptX project.
pub mod custom_errors {
    use core::fmt;

    #[derive(Debug)]
    pub enum ScriptxErrors {
        /// Errors dealing with dependency errors.
        DependencyError,
        /// Errors dealing with file read or write.
        FileError,
    }

    impl std::error::Error for ScriptxErrors {}

    impl std::fmt::Display for ScriptxErrors {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                ScriptxErrors::DependencyError => {
                    write!(f, "DependencyError:")
                }
                ScriptxErrors::FileError => {
                    write!(f, "LibraryError:")
                }
            }
        }
    }
}
