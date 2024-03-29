/*!
ffprobe wrapper

This module is a wrapper for the [ffprobe](https://ffmpeg.org/ffprobe.html) tool.
It gathers relevant information needed for ScriptX function, for example, it is charged
with collecting and returning chapter information such as the *start* and *end* time
stamps of each verse that is later used by [ffmpeg](https://ffmpeg.org/) to cut out
specific verses from the video file.
*/

use crate::ffwrappers::errors::Errors;
use core::{f64, str};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{path::Path, process::Command};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("The verse `{verse:?}` was not found")]
    VerseNotFound { verse: String },
    #[error("The prefix was not found")]
    PrefixNotMatch,
    #[error("Unable to parse verse number from {item:?} to i32")]
    VerseFromTitle { item: String },
}
#[derive(PartialEq, Debug)]
enum VerseKind {
    SingleVerse,
    RangeVerse,
}

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
/**
Contains the title field for each chapter.

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
    pub fn new(path: &Path) -> Result<Root, Errors> {
        let probe = Command::new("ffprobe")
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
            eprint!(
                "The file, {}, was not found by ffprobe. Check path and try again. ",
                path.to_str().unwrap()
            );
            return Err(Errors::FileError);
        }

        let c: Root =
            serde_json::from_slice(&probe.stdout).expect("Error during JSON parsing of file.");
        Ok(c)
    }

    /**
    Returns the *start* and *end* time for the verse passed in.

    The `verse_times` method takes a verse and returns the *start* and *end* times
    for the verse passed in.

    ## Example

    ```rust,
    use ffwrappers::probe;
    # use ffwrappers::probe::{Root, Chapter, Tags};
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
    pub fn verse(&self, verse: &str) -> Result<(f64, f64), Error> {
        let kind: VerseKind = verse_kind(verse);
        match kind {
            VerseKind::SingleVerse => Ok(self.return_single_verse(verse)?),
            VerseKind::RangeVerse => Ok(self.return_range_verse(verse)?),
        }
    }

    /// Returns a tuple with the _start_ and _end_ time for a singe verse.
    fn return_single_verse(&self, verse: &str) -> Result<(f64, f64), Error> {
        Ok(self
            .get_times(self.find_verse_id(format!("{}{}", self.get_prefix()?, verse).as_str())?)?)
    }

    /// Returns a tuple with the _start_ and _end_ time for a range of verses.
    fn return_range_verse(&self, verse: &str) -> Result<(f64, f64), Error> {
        let range: (&str, &str) = range_split(verse)?;
        let start_time: f64 = self.return_single_verse(range.0).unwrap().0;
        let end_time: f64 = self.return_single_verse(range.1).unwrap().1;

        Ok((start_time, end_time))
    }

    /// Searches for the title of the verse and returns the ``id`` if the chapter the title is found.
    fn find_verse_id(&self, verse: &str) -> Result<i64, Error> {
        for i in self.chapters.iter() {
            if i.tags.title == verse {
                return Ok(i.id);
            }
        }

        Err(Error::VerseNotFound {
            verse: verse.to_string(),
        })
    }

    /// Returns a tuple of the *start* and *end* time for the chapter in which the *id* has been provided for.
    fn get_times(&self, id: i64) -> Result<(f64, f64), Error> {
        for i in self.chapters.iter() {
            if i.id == id {
                return Ok((i.start_time.parse().unwrap(), i.end_time.parse().unwrap()));
            }
        }
        unreachable!()
    }

    /// Returns the last chapter in the file.
    fn get_last_chapter(&self) -> Result<&Chapter, Error> {
        Ok(self.chapters.last().unwrap())
    }

    /**
    Returns the prefix needed to complete the title being searched for.

    In order to avoid the user from having to type the complete title of the chapter being
    searched, this method finds the prefix of the chapter so the user doesn't have to add it
    when using the `-v` option.

    The method searches for all text up to and including the ':'. For example, the title
    _Joel 1:2_ would match and return the Joel 1:_ as a &str. The prefix is then combined
    with the verse the user wants.
    */
    fn get_prefix(&self) -> Result<&str, Error> {
        let title = self.get_last_chapter()?.tags.title.as_str();

        let pattern = Regex::new(r"(^[\s\S]*:)").unwrap(); // Regular expression to find the prefix (ig. 'Joel 1:') of a `title`. See https://regexr.com/5vus6
        let prefix = pattern.captures(title);

        match prefix {
            Some(prefix) => Ok(prefix.get(1).unwrap().as_str()),

            None => Err(Error::PrefixNotMatch),
        }
    }

    /// Returns all the verses' start and end times in a vector.
    pub fn get_all_verses(&self) -> Result<Vec<(f64, f64)>, Error> {
        // let prefix: &str = self.get_prefix(); // Example of prefix: 'John 3:'.
        let last_verse: i32 = get_verse_from_title(self.get_last_chapter()?.tags.title.as_str())?;
        let mut all_verses: Vec<(f64, f64)> = Vec::new(); // tuple of all the start/end times for the entire chapter.

        for i in 1..last_verse {
            all_verses.push(self.verse(&i.to_string())?);
        }

        Ok(all_verses)
    }
}

/**
Returns the verse number from a full title.
## Example
```rust
let verse: i32 = get_verse_from_title("John 3:16")
assert_eq!(verse, 16i32));

```
*/
fn get_verse_from_title(title: &str) -> Result<i32, Error> {
    let v: Vec<&str> = title.split(':').collect();

    match v[1].parse::<i32>() {
        Ok(p) => Ok(p),
        Err(_) => Err(Error::VerseFromTitle {
            item: String::from(v[1]),
        }),
    }
}

/// Determines whether the verse is single or part of a range.
fn verse_kind(verse: &str) -> VerseKind {
    if verse.contains('-') {
        VerseKind::RangeVerse
    } else {
        VerseKind::SingleVerse
    }
}

/// Splits the verse range into two parts and returns a tuple (starting_verse, ending_verse).
///
/// ## Example
///
/// ```rust
///
/// let verse = "12-15";
/// let s = range_split(verse).unwrap();
/// assert_eq!(s, (12, 15));
///
/// ```
fn range_split(verse: &str) -> Result<(&str, &str), Error> {
    let s: Vec<&str> = verse.split('-').collect();
    Ok((s[0], s[1]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_verse_from_title() {
        let title_0 = "Joel 5:1";
        assert_eq!(get_verse_from_title(title_0).unwrap(), 1);

        let title_0 = "John 3:16";
        assert_eq!(get_verse_from_title(title_0).unwrap(), 16);

        let title_0 = "Ps. 18:32";
        assert_eq!(get_verse_from_title(title_0).unwrap(), 32);
    }

    #[test]
    fn test_verse() {
        let root = init_struct_1();
        assert_eq!(root.verse("16").unwrap(), (197.597, 226.259));
        assert_eq!(root.verse("17").unwrap(), (226.259, 241.908));
        assert_eq!(root.verse("25").unwrap(), (358.658, 374.741));
        assert_eq!(root.verse("26").unwrap(), (374.741, 394.561));
    }

    #[test]
    #[should_panic]
    fn test_verse_not_found() {
        let root = init_struct_1();
        assert_eq!(root.verse("27").unwrap(), (197.597, 226.259));
    }

    #[test]
    fn test_range_split_1() {
        assert_eq!(range_split("5-7").unwrap(), ("5", "7"));
    }
    #[test]
    fn test_range_split_2() {
        assert_eq!(range_split("5-17").unwrap(), ("5", "17"));
    }
    #[test]
    fn test_range_split_3() {
        assert_eq!(range_split("15-17").unwrap(), ("15", "17"));
    }

    #[test]
    fn test_get_prefix() {
        let r: Root = init_struct_1();
        assert_eq!(r.get_prefix().unwrap(), "John 3:");
    }
    #[test]
    fn test_find_verse_id() {
        let r: Root = init_struct_1();
        let id = r.find_verse_id("John 3:16").unwrap();
        assert_eq!(id, 16);
    }

    #[test]
    #[should_panic]
    fn test_find_verse_id_fail() {
        let r: Root = init_struct_1();
        let _id = r.find_verse_id("Robert 3:16").unwrap();
    }
    #[test]
    fn test_find_times() {
        let r: Root = init_struct_1();
        assert_eq!(r.get_times(16).unwrap(), (197.597, 226.259));
    }
    #[test]
    fn test_return_range_verse() {
        let r: Root = init_struct_1();
        assert_eq!(r.return_range_verse("16-17").unwrap(), (197.597, 241.908))
    }

    #[test]
    fn test_return_single_verse() {
        let r: Root = init_struct_1();
        assert_eq!(r.return_single_verse("16").unwrap(), (197.597, 226.259))
    }

    #[test]
    fn test_verse_single() {
        let r: Root = init_struct_1();
        assert_eq!(r.verse("16").unwrap(), (197.597, 226.259))
    }
    #[test]
    fn test_verse_range() {
        let r: Root = init_struct_1();
        assert_eq!(r.verse("16-17").unwrap(), (197.597, 241.908))
    }

    #[test]
    fn test_verse_kind() {
        assert_eq!(verse_kind("1"), VerseKind::SingleVerse)
    }

    #[test]
    fn test_last_chapter() {
        let r: Root = init_struct_1();
        let chapter: &Chapter = r.get_last_chapter().unwrap();
        assert_eq!(chapter.id, 26);
    }

    #[test]
    #[ignore = "Need to finish writing the test."]
    fn test_get_all_verses() {
        let f = init_struct_1();
        assert_eq!(f.get_all_verses().unwrap(), vec![(197.597000, 4226.259000)]);
    }

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
// #[test]
// fn test_last_id() {
//     let r: Root = init_struct_1();
//     let id: i64 = r.last_id();
//     assert_eq!(id, 26);
// }

// #[test]
// #[ignore = "Not sure how to test this method yet."]
// fn test_last_chapter() {
//     let r: Root = init_struct_1();
//     let last_chapter: &Chapter = r.last_chapter();
// }

/*
Structure of the structs used in ff::probe

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
