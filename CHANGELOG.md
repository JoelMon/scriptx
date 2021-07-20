# Changelog

## ScriptX [v0.4.5](https://github.com/JoelMon/scriptx/releases/tag/v0.4.5) (2021-7-20)

## Changes
- Added Github workflow to auto create deb files for Ubuntu amd64 systems
- Updated the `README.md` file

## ScriptX [v0.4.4](https://github.com/JoelMon/scriptx/releases/tag/v0.4.4) (2021-7-20)

## Fix
- Close #27 - Removed `ffmpeg` from `Cargo.toml`
- Updated the `Readme.md` *Usage* section

## Improvements
- Added the new `-a` feature to the `README.md` *Features* section
- Added instructions for installing ScriptX from the repository using `cargo install --git`
- Close #18 - Started using `std::path::Path` instead of strings
- Made code more idiomatic

## ScriptX [v0.4.3](https://github.com/JoelMon/scriptx/releases/tag/v0.4.3) (2021-7-05)

## Fix
- Close #24 - Reworded cut to extract in the deb `control` file and other places.

## Other
- Instead of modifying a static man file, `make-deb` generates and compresses the man file. The man file created isn't perfect but it's better than having the man page to by out of sync with development. **NOTE:** With [Clap 3](https://github.com/clap-rs/clap/issues/552) man pages generation should be possible.:w
- Replaced clap's description string with the `crate_description` macro. The `--help` application description will be the same as the one in `Cargo.toml`.
- Tweaked the wording in the `-v` option help string.

## ScriptX [v0.4.2](https://github.com/JoelMon/scriptx/releases/tag/v0.4.2) (2021-7-04)
## New Feature
- Close #21 - Added a progress bar to when using the `-a --all` flag.

## Fix
- Fixed #20 - Removed the out of date "known bug" from `-h`.

## Other
- Added man pages for the project
- Updated `hermit-abi` crate
- Close #22 - made a script, `make-deb` to create deb files
  - Added `deb` directory that holds the control file used to create deb file
  - Add `man` directory that holds the man file for creating Linux packages

## ScriptX [v0.4.1](https://github.com/JoelMon/scriptx/releases/tag/v0.4.1) (2021-7-04)
## Fix
- Fixed #17 bug where some Bible books were not being extracted correctly.
- Fixed #12 Now if the output file exists, it overrides it instead of crashing.

## ScriptX [v0.4.0](https://github.com/JoelMon/scriptx/releases/tag/v0.4.0) (2021-7-02)
## Fix
- Fixed the version number. v0.3.3 was supposed to be v0.4.0
- Also updated the `README.md` file.

## ScriptX [v0.3.3](https://github.com/JoelMon/scriptx/releases/tag/v0.3.3) (2021-6-18)

### Fixes
- Removed unused code in `test_verse_kind()` function.
- Fixed #16 - Removed badge.
- Fixed #15 - Changed ig. - eg. in ScriptX's help menu.
- Updated `Cargo.toml` to avoid crates.io from trying to generate documentation.

### New Feature
- Ability to extract all verses with the `-a` flag.

### Other
- Added a note of issue #17 to the menu.

## ScriptX [v0.3.2](https://github.com/JoelMon/scriptx/releases/tag/v0.3.2) (2021-6-18)

### Changes
- Added test for function `verse_kind()`.

## ScriptX [v0.3.1](https://github.com/JoelMon/scriptx/releases/tag/v0.3.1) (2021-6-17)

### Improvements
- Refactored parts of the library.
- Added all unit tests for `probe.rs`.
  
### Changes
- Restructured the modules into files and directories for organization.
- Renamed error module within `main.rs` to _scriptx\_errors_.
- Added `[[bin]]` to the `Cargo.toml` file. 

## ScriptX [v0.3.0](https://github.com/JoelMon/scriptx/releases/tag/v0.3.0) (2021-6-17)

### New Feature
- Close #9 - Added ability to cut a range of scriptures.

### Improvements
- Updated the help menu to include the new range feature and updated other options.

## ScriptX [v0.2.4](https://github.com/JoelMon/scriptx/releases/tag/v0.2.4) (2021-6-17)

### Improvements
- Added doc.rs and repo status badges. [100b78a](https://github.com/JoelMon/scriptx/commit/100b78ae9b6fbf78b1786d28b1ffa8e62d81ef9d#diff-b335630551682c19a781afebcf4d07bf978fb1f8ac04c6bf87428ed5106870f5)
- Added information about dependencies on the `README.md`.

#### Error Handling
- Improved error handling when checking for dependency.
- Improved error handling when file path is incorrect.
- Improved overall error messages.

### Other
- Added `.mp4` video files to the `.gitignore` file to avoid pushing test video files to repository.
- Set the default output file to a `.m4v` container.  
- Removed deprecated badges from `Cargo.toml`.

## ScriptX [v0.2.3](https://github.com/JoelMon/scriptx/releases/tag/v0.2.3) (2021-6-16)

### Fixed
- Fixed the doc test for the method `verse`.
- Fixed the CHANGELOG entry for ScriptX [v0.2.2](https://github.com/JoelMon/scriptx/releases/tag/v0.2.2) (2021-6-14). The link was pointing to the `v0.2.1` instead of `v0.2.2` tag.
- Updated the code example for `ff::mpeg::cut`.
- The `Cargo.toml` file does not need both the `license` _and_ `license_file` fields.

### Improvements
- Improved some the text found in code examples.
- Added the `documentation` field to the `Cargo.toml` file.

### Changes
- Switched from GLP3 license to the MIT license.

## ScriptX [v0.2.2](https://github.com/JoelMon/scriptx/releases/tag/v0.2.2) (2021-6-14)

### Fixed
- Fixed spelling in `CHANGELOG.md`.
- Fixed spelling in `ff.rs`.
- Removed `mod ff` from within `main.sr`.

### Improvements
- Added comment to the regular expression code.
- Close #11 - Added tests to public API in ff module.
- Improved the documentation within the `ff`library.
- Added `[lib]` pointing to `ff.rs`.
  - Now `cargo test` will include doc tests.

## ScriptX [v0.2.1](https://github.com/JoelMon/scriptx/releases/tag/v0.2.1) (2021-6-13)

### Improvements
- Improved the documentation for the new methods created for v0.2.0 and a few others.
- Changed the panic message if the pattern for a title isn't found.
- Close #8 - Added logic to determine if `ffprobe` and `ffmpeg` are installed.

## ScriptX [v0.2.0](https://github.com/JoelMon/scriptx/releases/tag/v0.2.0) (2021-6-13)

### Added
- Added `*.m4v` to the `.gitignore` file to ignore any test videos used during development and test.

### Fixes
- Fixed #7 - Improved the description field within `Cargo.toml`.

### Implemented
- Implemented #2 - ScriptX figures out the book of the Bible and chapter it is parsing automatically. The user no longer has to type `-v "Joel 1:1"`. Now the verse can be included without the name of the book or chapter, for example, the following is now valid when the book of Joel chapter 1 is used: `-v 1` will export _Joel 1:1_.

## ScriptX [v0.1.2](https://github.com/JoelMon/scriptx/releases/tag/v0.1.2) (2021-6-13)

## Fixed
- Fixed #6 - Replaced the application's `--help` short description to:

 > ScriptX cuts scriptures from the American Sign Language version of the New World Translation Bible.

 [[dc72efb]](https://github.com/JoelMon/scriptx/commit/dc72efb79c300895e1987864ad87cdfc9782b565)
 
## ScriptX [v0.1.1](https://github.com/JoelMon/scriptx/releases/tag/v0.1.1) (2021-6-13)

### Added
- Added a CHANGELOG.md document to keep track of changes made to ScriptX. [[ec6600]](https://github.com/JoelMon/scriptx/commit/0422b0bcb605082eca8704f9dddfd0ad85ec6600)

- Added _*.md.backup_ to `.gitignore` to ignore backup files created by [ghostwriter](https://wereturtle.github.io/ghostwriter/	). [[ec6600]](https://github.com/JoelMon/scriptx/commit/0422b0bcb605082eca8704f9dddfd0ad85ec6600)

### Fixed
- Fixed [#4](https://github.com/JoelMon/scriptx/issues/4) - Fixed the *description* string within the `Cargo.toml` file. [[c59bfd]](https://github.com/JoelMon/scriptx/commit/a385adeab40a1742435a9278679ae2c153c59bfd)

## ScriptX [v0.1.0](https://github.com/JoelMon/scriptx/releases/tag/v0.1.0) (2021-6-12)
Initial release of [ScriptX](https://joelmon.github.io/scriptx/).

