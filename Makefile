clean:
	cargo clean

build:
	cargo build

clean_courses:
	rm -rf courses/

build_courses: build clean_courses
	mkdir courses
	cd courses; cargo run
