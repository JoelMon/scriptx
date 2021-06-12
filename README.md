# ScriptX
ScriptX is a command line program designed to cut specific verses out of the American Sign Language version of the [New World Translation](https://www.jw.org/ase/library/bible/nwt/books/), NWT, Bible published by [Watch Tower Bible and Tract Society of Pennsylvania](https://www.JW.org).

ScriptX is still in early stages and may change dramatically in future versions.

## Installation

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
 
 
