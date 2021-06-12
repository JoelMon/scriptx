#![warn(missing_docs)]
/*!
ffprpbe & ffmpeg wrapper for the ScriptX project

ScriptX is a program to cut out verses from the [American Sign Language Bible](https://www.jw.org/ase/library/bible/nwt/books/) published by Watch Tower Bible and Tract Society of Pennsylvania in order to aid in showing specific scripture verses without having to work with an entire chapter.

An example of use would be to create a playlist for the use during a congregation meeting or for study.

This module is a simple wrapper for [ffprobe](https://ffmpeg.org/ffprobe.html) and [ffmpeg](https://ffmpeg.org/). These two tools must be installed on the system in order for ScriptX to work.
*/

use std::process::Command;

/**
ffprobe wrapper

This module is a wrapper for the [ffprobe](https://ffmpeg.org/ffprobe.html) tool. It gathers relevant information needed for ScriptX function, for example, it is charged with collecting and returning chapter information such as the *start* and *end* time stamps of each verse that is later used by [ffmpeg](https://ffmpeg.org/) to cut out specific verses from the video file.
*/
pub mod probe {
    use serde::{Deserialize, Serialize};
    /**
    The top most, or *root*, level of the scruct

    It contains a single scruct of vector type which is a collection of all the chapters contained within the video file.
     */
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Root {
        pub chapters: Vec<Chapter>,
    }
    /**
    The struct for the chapter

    Contains information for each chapter within a video file. Also contains the `Tags` scruct.
    */
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Chapter {
        pub id: i64,
        #[serde(rename = "time_base")]
        pub time_base: String,
        pub start: i64,
        #[serde(rename = "start_time")]
        pub start_time: String,
        pub end: i64,
        #[serde(rename = "end_time")]
        pub end_time: String,
        pub tags: Tags,
    }
    /** Contains the title for each chapter

    The *title* field is used to determine the chapter that contains the verse(s) being searched for.

    For example, if the verse *John 3:16* needs to be found, searching the title fields will result in the correct chapter struct which also contains additional information needed to cut the verse.
    */
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Tags {
        pub title: String,
    }

    impl Root {
        /** Returns the Root sctruct when given a path

        # Example
        ```rust
        use ff::probe;
        probe::probe(String::from("video-file.m4v"));
        ```
        */
        pub fn new(path: &str) -> Root {
            let probe = super::Command::new("ffprobe")
                .arg("-v")
                .arg("quiet")
                .arg("-print_format")
                .arg("json")
                .arg("-show_chapters")
                .arg("-i")
                .arg(path)
                .output()
                .unwrap();

            if !probe.status.success() {
                panic!("ffprobe's exit status was FAILURE");
            }

            let c: Root = serde_json::from_slice(&probe.stdout).expect("Something went wrong");
            c
        }

        // Returns the *start* and *end* time for the verse passed in.
        pub fn verse(&self, verse: &str) -> (f64, f64) {
            self.find_times(self.find_verse_id(verse))
        }

        /** Searches for the title of the verse and returns the ``id`` of the chapter the title is found

        */
        fn find_verse_id(&self, verse: &str) -> i64 {
            for i in self.chapters.iter() {
                if i.tags.title == verse {
                    return i.id;
                }
            }
            panic!("The verse was not found.");
        }

        /** Returns a tuple of the *start* and *end* time for the chapter in which the *id* has been provided for

        */
        fn find_times(&self, id: i64) -> (f64, f64) {
            for i in self.chapters.iter() {
                if i.id == id {
                    return (i.start_time.parse().unwrap(), i.end_time.parse().unwrap());
                }
            }
            unreachable!()
        }
    }
}
/**
ffmpeg wrapper

A simple wrapper used to slice specific verses out of video files.

*/
pub mod mpeg {
    /**
    Cuts the video out at specific *start* and *end* times.

    # Example
    ```rust
    use ff:mpeg;
    mpeg::cut(23.32342, 50.234234, String::from("video-file.m4v")); // Slices the given video at the given time stamp and outputs to *output.mp4*.
    ```
    */
    pub fn cut(start_time: f64, end_time: f64, path: &str, output: &str) {
        let command = super::Command::new("ffmpeg")
            .arg("-v")
            .arg("quiet")
            .arg("-i")
            .arg(path)
            .arg("-ss")
            .arg(start_time.to_string())
            .arg("-to")
            .arg(end_time.to_string())
            .arg("-c")
            .arg("copy")
            .arg(output)
            .output()
            .unwrap();

        if !command.status.success() {
            panic!("ffmpeg's exit status was FAILURE");
        }
        println!(
            "Start time: {}, End time: {}",
            start_time.to_string(),
            end_time.to_string()
        );
    }
}
