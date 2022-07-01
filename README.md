# trane-music

This repository contains courses in music designed to be used with
[Trane](https://github.com/trane-project/trane). It is in active development and new material is
being added.

## Building

Building is not needed for most users. All the course materials are kept under the `courses/`
directory and can be used immediately.

These course materials are built by running the binary under the `src/` directory. If you want to
rebuild the courses, use the Makefile provided along by executing `make build_courses`. This command
requires that a stable version of the Rust toolchain is installed.

## Usage

Install [trane-cli](https://github.com/trane-project/trane-cli) in your computer. Then, you can
download this repository, start the `trane` command, and open a new Trane library with the `open
<DIRECTORY>` command inside the CLI.

You can also copy only the directories for the courses you want to practice to another directory.
It's also perfectly OK to mix and match courses from multiple sources. They should work seamlessly
with each other as long as there are no issues with the dependencies (e.g. cycles) or repeated IDs.
All official courses have IDs with the prefix `trane::` to help avoid those issues.

## Contributing

`trane-music` is open for contributions, ranging from course requests to fully built courses. See
the `CONTRIBUTING.md` file for more details.
