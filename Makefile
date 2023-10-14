.PHONY: server build test app mobile

# Run the server with specific features
server:
	cargo run --features="server,detect,yolov8,webcam" --no-default-features --release

# Build the project
build:
	cargo build

# Run tests
test:
	cargo test

# Serve the app
app:
	cd app && trunk serve

# Run mobile development
mobile:
	cd app && cargo tauri android dev
