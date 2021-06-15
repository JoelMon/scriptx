#![warn(missing_docs)]
/*!
ffprpbe & ffmpeg wrapper for the ScriptX project

ScriptX is a program to cut out verses from the [American Sign Language Bible](https://www.jw.org/ase/library/bible/nwt/books/)
published by Watch Tower Bible and Tract Society of Pennsylvania in order to aid in showing specific scripture verses without
having to work with an entire chapter.

An example of use would be to create a playlist for the use during a congregation meeting or for study.

This module is a simple wrapper for [ffprobe](https://ffmpeg.org/ffprobe.html) and [ffmpeg](https://ffmpeg.org/).
These two tools must be installed on the system in order for ScriptX to work.
*/

use regex::Regex;
use std::process::Command;

/** ffprobe wrapper

This module is a wrapper for the [ffprobe](https://ffmpeg.org/ffprobe.html) tool.
It gathers relevant information needed for ScriptX function, for example, it is charged
with collecting and returning chapter information such as the *start* and *end* time
stamps of each verse that is later used by [ffmpeg](https://ffmpeg.org/) to cut out
specific verses from the video file.
*/
pub mod probe {
    use core::str;

    use serde::{Deserialize, Serialize};
    /**
    The top most, or *root*, level of the struct

    It contains a single struct of vector type which is a collection of all the chapters contained within the video file.
     */
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Root {
        /// The `chapters` fields contains a vector of the type `Chapter` struct.
        pub chapters: Vec<Chapter>,
    }
    /**
    The struct for the chapter

    Contains information for each chapter within a video file. Also contains the `Tags` struct. Specifically the
    _start_time_ and _end_time_ are used from this struct.
    */
    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Chapter {
        /// The `id` field that uniquely identifies each chapter.
        pub id: i64,
        /// The `time_base` field.
        #[serde(rename = "time_base")]
        pub time_base: String,
        /// The `start` field.
        pub start: i64,
        /**
        The `start_time` field.
        This field contains the start time for the chapter.
        */
        #[serde(rename = "start_time")]
        pub start_time: String,
        /// The `end` field.
        pub end: i64,
        #[serde(rename = "end_time")]
        /**
        The `end_time` field.
        This field contains the end time for the chapter.
        */
        pub end_time: String,
        ///The `Tags` field.
        pub tags: Tags,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    /** Contains the title field for each chapter.

    The *title* field is used to determine the chapter that contains the verse(s) being searched for.

    For example, if the verse *John 3:16* needs to be found, searching the title fields will result in
    the correct chapter struct which also contains additional information needed to cut the verse.
    */
    pub struct Tags {
        /**
        The title field for each chapter.

        The `verse` function searches this field when looking for
        the verse, for example `Ps. 83:13`, needed.
        */
        pub title: String,
    }

    impl Root {
        /**
        Returns the Root struct when given a path to a video file.

        The `new` function initializes a new chapter video to be worked with.
        The function parses the meta data of the video with [ffprobe](https://ffmpeg.org/ffprobe.html)
        and returns the `Root` struct which contains all the chapter information needed
        for cutting the scriptures.

        ## Example
        For example, if you want to load chapter 3 of the book of John
        to process it, you would do the following:

        ```rust, ignore
        use ff::probe;
        let chapter:Root = probe::new("nwt_43_Joh_ASL_03_r720P.mp4");
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

        /**
        Returns the *start* and *end* time for the verse passed in.

        The `verse_times` method takes a verse and returns the *start* and *end* times
        for the verse passed in.

        ## Example

        ```rust,
        use ff::probe;
        # use ff::probe::{Root, Chapter, Tags};
        # let chapter: Root = Root {
        #     chapters: {
        #         vec![
        #             Chapter {
        #                id: 16,
        #                 time_base: String::from("1/1000"),
        #                 start: 197597,
        #                 start_time: String::from("197.597000"),
        #                 end: 226259,
        #                 end_time: String::from("226.259000"),
        #                 tags: Tags {
        #                     title: String::from("John 3:16"),
        #                 },
        #             },
        #             Chapter {
        #                 id: 17,
        #                 time_base: String::from("1/1000"),
        #                 start: 197597,
        #                 start_time: String::from("226.259000"),
        #                 end: 241908,
        #                 end_time: String::from("241.908000"),
        #                 tags: Tags {
        #                     title: String::from("John 3:17"),
        #                 },
        #             },
        #         ]
        #     }
        # };

        // let chapter:Root = probe::new("nwt_43_Joh_ASL_03_r720P.mp4");
        assert_eq!(chapter.verse("16"), (197.597, 226.259)); // Returned the `start_time` and `end_time`.
        ```

         */
        pub fn verse(&self, verse: &str) -> (f64, f64) {
            self.find_times(self.find_verse_id(format!("{}{}", self.get_prefix(), verse).as_str()))
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

        /** Returns the prefix needed to complete the title being searched for.
         *
         * In order to avoid the user from having to type the complete title of the chapter being
         * searched, this method finds the prefix of the chapter so the user doesn't have to add it
         * when using the `-v` option.
         *
         * The method searches for all text up to and including the ':'. For example, the title
         * _Joel 1:2_ would match and return the Joel 1:_ as a &str. The prefix is then combined
         * with the verse the user wants.
         */
        fn get_prefix(&self) -> &str {
            let last_chapter = *&self.chapters.last().unwrap();
            let title = last_chapter.tags.title.as_str();

            let pattern = super::Regex::new(r"(^[\s\S]*:)").unwrap(); // Regular expression to find the prefix (ig. 'Joel 1:') of a `title`. See https://regexr.com/5vus6
            let prefix = pattern.captures(title);

            match prefix {
                Some(prefix) => {
                    return prefix.get(1).unwrap().as_str();
                }
                None => panic!("The prefix pattern was not found."),
            };
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
    ```rust, ignore
    use ff:mpeg;
    mpeg::cut(23.32342, 50.234234, "nwt_43_Joh_ASL_03_r720P.mp4", "John_3-5.mp4"); // Slices the given video at the given time stamp and outputs to *John_3-5.mp4*.
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

#[cfg(test)]
mod test {
    use super::probe::*;

    #[test]
    fn test_verse() {
        let root = init_struct_1();
        assert_eq!(root.verse("16"), (197.597, 226.259));
        assert_eq!(root.verse("17"), (226.259, 241.908));
        assert_eq!(root.verse("25"), (358.658, 374.741));
        assert_eq!(root.verse("26"), (374.741, 394.561));
    }

    #[test]
    #[should_panic]
    fn test_verse_not_found() {
        let root = init_struct_1();
        assert_eq!(root.verse("27"), (197.597, 226.259));
    }

    /* Structure of the structs used in ff::probe

    struct Root {
        chapters: Vec<Chapter>,
    }
    struct Chapter {
        id: i64,
        time_base: String,
        start: i64,
        start_time: String,
        end: i64,
        end_time: String,
        tags: Tags,
    }
    pub struct Tags {
        title: String, // The verse(s) being search for is compared to this field.
    }
    */
    fn init_struct_1() -> Root {
        let root_struct: Root = Root {
            chapters: {
                vec![
                    Chapter {
                        id: 16,
                        time_base: String::from("1/1000"),
                        start: 197597,
                        start_time: String::from("197.597000"),
                        end: 226259,
                        end_time: String::from("226.259000"),
                        tags: Tags {
                            title: String::from("John 3:16"),
                        },
                    },
                    Chapter {
                        id: 17,
                        time_base: String::from("1/1000"),
                        start: 197597,
                        start_time: String::from("226.259000"),
                        end: 241908,
                        end_time: String::from("241.908000"),
                        tags: Tags {
                            title: String::from("John 3:17"),
                        },
                    },
                    Chapter {
                        id: 25,
                        time_base: String::from("1/1000"),
                        start: 358658,
                        start_time: String::from("358.658000"),
                        end: 374741,
                        end_time: String::from("374.741000"),
                        tags: Tags {
                            title: String::from("John 3:25"),
                        },
                    },
                    Chapter {
                        id: 26,
                        time_base: String::from("1/1000"),
                        start: 374741,
                        start_time: String::from("374.741000"),
                        end: 394561,
                        end_time: String::from("394.561000"),
                        tags: Tags {
                            title: String::from("John 3:26"),
                        },
                    },
                ]
            },
        };

        root_struct
    }
}
