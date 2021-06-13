# Changelog

## ScriptX [v0.2.0](https://github.com/JoelMon/scriptx/releases/tag/v0.2.0) (2021-6-13)

### Added
- Added `*.m4v` to the `.gitignore` file to ignore any test videos used during development and test.

### Fixes
- Fixed #7 - Improved the description field within `Cargo.toml`.

### Implemented
- Implemented #2 - ScriptX figures out the book of the Bible and chapter it is parsing automaticly. The user no longer has to type `-v "Joel 1:1"`. Now the verse can be included without the name of the book or chapter, for example, the following is now valid when the book of Joel chapter 1 is used: `-v 1` will export _Joel 1:1_.


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
Inital release of [ScriptX](https://joelmon.github.io/scriptx/).

