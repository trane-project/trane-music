# Contributing to trane-music

`trane-music` is in active development and new courses are in the process of being added. All the
courses in this repository are code-generated using the code inside the `src/` directory. While all
the files needed to share courses are plain text (either markdown or JSON files), generating them by
hand quickly becomes a painful process. Trane is also in active development, which makes generating
the materials by code a better choice for keeping the material up to date with the latest changes.

However, there are multiple valuable ways you can contribute to this project, even if you've never
touched a line of code or opened a JSON file in your life.

## Suggesting new material

Easiest way to contribute is by simply opening an issue and suggesting new courses or lessons to be
added. It is especially useful to detail how the material in those units relates to other material
(either in existing courses or ones that have not been added yet) and how you would break the
material into smaller lessons, if necessary. Either links to reference material or example
exercises are also useful.

If you are an experienced musician but can't (or want) to code any material yourself, this is a
great way to contribute. A large part of the task at hand is breaking down skills and knowledge
into smaller units and figuring a sensible order in which they should be introduced to students.

## Planning new material

This is a more involved version of the above. You still don't need to code any material yourself but
if you already know exactly how the material you want to add translates to courses, lessons, and
exercises, you can write them in a simplified format in plain text and propose the course is added
by opening and issue and sharing your work. Once all the details have been ironed out and your
proposal has been accepted, someone else can work into translating it into working code.

## Coding new material

You can also contribute by adding the code to generate new courses and lessons inside the `src/`
folder. The best way to learn is to look at how existing courses are defined. You can either create
courses for material you have designed yourself or generate new material that has been proposed and
approved but still needs to be added as code.

Make sure you have gotten an approval to work on new material before submitting a PR.
