# trane-music

This repository contains courses in general musicianship designed to be used with
[Trane](https://github.com/trane-project/trane). It is in active development and new material is
being added.

Courses for specific instruments are not included in this repository so that it is easier to pick
and choose which courses you want to use.

## Building

Building is not needed for most users. All the course materials are kept under the `courses/`
directory and can be used immediately.

These course materials are built by running the binary under the `src/` directory. If you want to
rebuild the courses, use the Makefile provided along by executing `make build_courses`. This command
requires that a stable version of the Rust toolchain is installed.

## Usage

For instructions on how to use Trane, please refer to the [quick start
guide](https://trane-project.github.io/quick_start.html).

## Contributing

`trane-music` is open for contributions, ranging from course requests to fully built courses. See
the `CONTRIBUTING.md` file for more details.
