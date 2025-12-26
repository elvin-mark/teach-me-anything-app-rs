build:
	@cargo clippy
	@cargo fmt
	@cargo build --release

docker:
	@docker build -t tools/teach-me-anything-app .

run: build
	@RUST_LOG=debug ./target/release/teach-me-anything-app-rs