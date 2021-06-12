// #![warn(missing_docs)]

use clap::{crate_authors, crate_version, App, Arg};
mod ff;
use ff::{mpeg, probe};

/// ScriptX - A Sign Language Bible verse slicer.
fn main() {
    let matches = App::new("ScriptX")
        .author(crate_authors!())
        .about("This application cuts scriptures")
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

    let path = matches.value_of("file").unwrap();
    let verse = matches.value_of("verse").unwrap();
    let output_path = matches.value_of("output_path").unwrap();
    let (start_time, end_time) = probe::Root::new(path).verse(verse);

    mpeg::cut(start_time, end_time, path, output_path);
}
