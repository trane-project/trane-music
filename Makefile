clean:
	cargo clean

build:
	cargo build

clean_courses:
	rm -rf old_courses/

build_courses: build clean_courses
	mkdir old_courses
	cd old_courses; cargo run
