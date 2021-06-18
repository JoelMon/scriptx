// #![warn(missing_docs)]
/*!
ffprpbe & ffmpeg wrapper for the ScriptX project

ScriptX is a program to cut out verses from the [American Sign Language Bible](https://www.jw.org/ase/library/bible/nwt/books/)
published by Watch Tower Bible and Tract Society of Pennsylvania in order to aid in showing specific scripture verses without
having to work with an entire chapter.

An example of use would be to create a playlist for the use during a congregation meeting or for study.

This module is a simple wrapper for [ffprobe](https://ffmpeg.org/ffprobe.html) and [ffmpeg](https://ffmpeg.org/).
These two tools must be installed on the system in order for ScriptX to work.
*/

pub mod errors;
pub mod mpeg;
pub mod probe;
