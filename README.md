# renjpg
A command line tool for automatically renaming JPEG files.

If available, *renjpg* prepends the file name with the original date/time from the file's EXIF metadata to the existing filename:

Given the EXIF tag `DateTimeOriginal` of `my photo.jpg` was `2019:08:17 11:59:22`, then calling *renjpg* would rename the file to `2019-08-17_115922_my photo.jpg`.

## Usage
To rename all JPEG files with EXIF data in the current folder execute

    ./renjpg

It is also possible to specify particular files that should be renamed:

    ./renjpg Foo.jpg Bar.jpg

The programme identifies JPEG files by the file extension `jpg` and `jpeg`. The case of the extension characters does not matter: e. g. `jPeG` would also be recognised.

## Build
Building the tool for your local environment is pretty straightforward: Simply execute

    cargo build --release

in the project root folder.
The binary `renjpg` can be found in folder `target/release`.

`Cargo.toml` defines a profile for release builds to minify the resulting binary as much as possible.

## Notes
Coming from Java, this is my very first Rust coding attempt.
I appreciate comments and hints about how the code could be improved.