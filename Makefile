make run:
	cargo build
	cargo run > image.ppm
	okular image.ppm
