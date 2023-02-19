clean:
	cargo clean

build:
	cargo build

clean_courses:
	rm -rf generated_courses/

build_courses: build clean_courses
	mkdir generated_courses
	cd generated_courses; cargo run
