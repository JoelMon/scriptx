# ScriptX
[![doc.rs badge](https://docs.rs/scriptx/badge.svg)](https://docs.rs/scriptx) [![Project Status: Active – The project has reached a stable, usable state and is being actively developed.](https://www.repostatus.org/badges/latest/active.svg)](https://www.repostatus.org/#active)


ScriptX is a command line program designed to cut specific verses out of the American Sign Language version of the [New World Translation](https://www.jw.org/ase/library/bible/nwt/books/), NWT, Bible published by [Watch Tower Bible and Tract Society of Pennsylvania](https://www.JW.org).

ScriptX is still in early stages and may change dramatically in future versions.

![scriptx-demo](https://user-images.githubusercontent.com/6587811/121826488-e6cf6400-cc85-11eb-8604-39dc87910e08.gif)

## Installation

### Dependencies 
Both `ffmpeg` and `ffprobe` has to be installed on the system.

### Using Cargo
`$ cargo install scriptx`

## Usage


```bash
USAGE:
    scriptx [OPTIONS] --file <file> --verse <verse>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --file <file>             The video file to process
    -o, --output <output_path>    The path were to save the output [default: output.mp4]
    -v, --verse <verse>           The verse to be cut out
```
 
## Note
This project, ScriptX, is not made nor is it associated with the [Watch Tower Bible and Tract Society of Pennsylvania](https://www.JW.org).
 
 
