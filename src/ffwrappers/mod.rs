/*!This module contains wrappers for [ffprobe](https://ffmpeg.org/ffprobe.html) and [ffmpeg](https://ffmpeg.org/) and also custom error types used within the library.

These two tools must be installed on the system in order for ScriptX to work.
*/

pub mod errors;
pub mod mpeg;
pub mod probe;
