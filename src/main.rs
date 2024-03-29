#![warn(missing_docs)]

//! ScriptX is a command line tool designed to extract scriptures out of the American Sign Language version of the [New World Translation](https://www.jw.org/ase/library/bible/nwt/books/), NWT, Bible published by the [Watch Tower Bible and Tract Society of Pennsylvania](https://www.JW.org). It is useful when you need specific scripture(s) in a standalone file. A quick example would be if you need to splice scriptures into another video.

mod ffwrappers;
use core::str;
use indicatif::ProgressIterator;
use std::path::Path;
use std::process::Command;
use std::u8;

use clap::{crate_authors, crate_description, crate_version, App, Arg, ArgGroup, ArgMatches};

use crate::ffwrappers::mpeg;
use crate::ffwrappers::probe::Root;
use scriptx_errors::ScriptxErrors;

/// ScriptX - A Sign Language Bible verse slicer.
fn main() -> Result<(), ScriptxErrors> {
    let m:ArgMatches = App::new("ScriptX")
        .author(crate_authors!())
        .about(crate_description!())
        .version(crate_version!())
        .arg(
            Arg::with_name("verse")
                .help("The verse to be extracted out. A single verse or a range of verses can be extracted. e.g. 2-5")
                .short("v")
                .long("verse")
                .takes_value(true),
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
        .arg(
            Arg::with_name("all")
                .help("Extracts all scriptures from the file.")
                .short("a")
                .long("all")
                .takes_value(false),
        )
        .group(ArgGroup::with_name("extraction_types")
            .args(&["all", "verse"])
            .multiple(false)
            .required(true),
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

    let path: &Path = Path::new(m.value_of("file").unwrap());
    let output_path: &Path = Path::new(m.value_of("output_path").unwrap());

    match m.is_present("all") {
        true => {
            match all_verses(path, output_path) {
                Ok(_) => (),
                Err(e) => return Err(e),
            };
        }
        false => {
            match some_verses(path, &m, output_path) {
                Ok(_) => (),
                Err(e) => return Err(e),
            };
        }
    };

    fn all_verses(path: &Path, out_path: &Path) -> Result<(), ScriptxErrors> {
        let chapters: Root = match Root::new(path).map_err(|_| ScriptxErrors::FileError) {
            Ok(r) => r,
            Err(e) => return Err(e),
        };
        let chapters_vec: Vec<(f64, f64)> = chapters.get_all_verses().unwrap();
        let mut i: u8 = 1;

        for scripture in chapters_vec.iter().progress() {
            let (start_time, end_time) = *scripture;
            let destination: &Path = out_path.parent().unwrap();
            let file_name = out_path
                .file_name()
                .expect("Expected to find a file name.")
                .to_str()
                .unwrap();

            mpeg::cut(
                start_time,
                end_time,
                path,
                format!("{}/{}-{}", destination.to_str().unwrap(), i, file_name).as_str(),
            );

            i += 1;
        }
        Ok(())
    }

    fn some_verses(path: &Path, m: &ArgMatches, output_path: &Path) -> Result<(), ScriptxErrors> {
        let verse: &str = m.value_of("verse").unwrap();

        let chapters: Root = match Root::new(path).map_err(|_| ScriptxErrors::FileError) {
            Ok(r) => r,
            Err(e) => return Err(e),
        };

        let (start_time, end_time) = chapters.verse(verse).unwrap();

        mpeg::cut(start_time, end_time, path, output_path.to_str().unwrap());
        Ok(())
    }

    Ok(())
}

/// Returns a bool if the ffprobe is installed otherwise an error is returned.
fn check_for_ffprobe() -> Result<bool, ScriptxErrors> {
    let ffprobe = Command::new("ffprobe").arg("-version").output();

    match ffprobe {
        Ok(o) => Ok(o.status.success()),
        _ => Err(ScriptxErrors::DependencyError),
    }
}

/// Returns bool if the ffprobe is installed otherwise an error is returned.
fn check_for_ffmpeg() -> Result<bool, ScriptxErrors> {
    let ffmpeg = Command::new("ffmpeg").arg("-version").output();

    match ffmpeg {
        Ok(o) => Ok(o.status.success()),
        _ => Err(ScriptxErrors::DependencyError),
    }
}

/// Custom errors for the ScriptX project.
pub mod scriptx_errors {
    use core::fmt;

    /// The various errors used within ScriptX.
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
