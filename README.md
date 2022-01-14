# OrnithologyPi
Capture birds in your garden, running on raspberry pi.

## Quickstart

### Webcam example

```sh
cargo run --example webcam
```

Output will be saved at frame.jpg.

### Crop example

```sh
cargo run --example crop
```

The cropped bird image will be saved at crop.jpg.

### Label example

```sh
cargo run --example label
```

The bird detected on the image will be printed.

### Window example

```sh
cargo run --features="window" example window
```

Press space for next frame.

Press Escape to exit.
