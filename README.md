# ScriptX
[![Project Status: Active – The project has reached a stable, usable state and is being actively developed.](https://www.repostatus.org/badges/latest/active.svg)](https://www.repostatus.org/#active)

ScriptX is a command line tool designed to extract scriptures out of the American Sign Language version of the [New World Translation](https://www.jw.org/ase/library/bible/nwt/books/), NWT, Bible published by the [Watch Tower Bible and Tract Society of Pennsylvania](https://www.JW.org). It is useful when you need specific scripture(s) in a standalone file. A quick example would be if you need to splice scriptures into another video. 

All the meta data stored within the original file is retained even after being extracted, so when extracting a range you will still have access to chapter markers within your extracted video.

## Features
- Extract single or a range of scriptures eg. `-v 5-10` extracts verses ranging from 5 to 10
- Maintains all metadata from the original file
- Doesn't transcode so blazing :fire: fast!

![scriptx-demo](https://user-images.githubusercontent.com/6587811/121826488-e6cf6400-cc85-11eb-8604-39dc87910e08.gif)


## Dependencies 
[ffmpeg](https://ffmpeg.org/) is required and has to be installed.


## Installation
ScriptX hasn't been tested on Windows but it should work. If it doesn't create an [issue](https://github.com/JoelMon/scriptx/issues) and I will look into it.

Use [Cargo](https://doc.rust-lang.org/cargo/) to install ScriptX.

```bash
$ cargo install scriptx
```


## Usage


```text
ScriptX cuts scriptures from the American Sign Language version of the New World Translation Bible.

USAGE:
    scriptx [OPTIONS] --file <file> <--all|--verse <verse>>

FLAGS:
    -a, --all        Extracts all scriptures from the file.
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --file <file>             The input video file to process.
    -o, --output <output_path>    The path were to save the output file. [default: output.m4v]
    -v, --verse <verse>           The verse to be cut out. A single verse or a range of verses can be cut, eg. 2-5.

There's a known bug where some books of the Bible are not extracted correctly. Check Github's issue #17:
https://github.com/JoelMon/scriptx/issues/17
```
 
## Contributing
Pull requests are welcome. For major changes or if unsure about a contribution, please open an [issue](https://github.com/JoelMon/scriptx/issues) first to discuss what you would like to change.

Please, also remember to run tests to make sure everything is okay.

## License
[MIT](https://choosealicense.com/licenses/mit/)

## Note
This project, ScriptX, is not made by nor is it associated with the [Watch Tower Bible and Tract Society of Pennsylvania](https://www.JW.org).
 
 
