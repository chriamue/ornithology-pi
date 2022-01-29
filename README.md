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

## Build Debian package

```
cargo install cargo-deb
cargo deb
```

### Build for Raspberry Pi

```
cargo install cross
cargo install cargo-deb
docker build -t crossbuild:local -f crossbuild.Dockerfile .
cross build --release --target armv7-unknown-linux-gnueabihf
cargo deb --no-build --target armv7-unknown-linux-gnueabihf
```

### Install on Raspberry Pi

You need some rerequirements:

```
sudo apt install gstreamer1.0-plugins-good
```

Now download and install.

```
wget https://github.com/chriamue/ornithology-pi/releases/latest/download/ornithology-pi.deb
sudo apt install ./ornithology-pi.deb
```

#### Iptables

```
sudo iptables -t nat -A PREROUTING -s 10.42.0.0/24 -p tcp --dport 80 -j DNAT --to-destination 127.0.0.1:8000
sudo iptables -t nat -A POSTROUTING -s 10.42.0.0/24 -j MASQUERADE
```

## Debug

### Bluetooth

For a bluetooth services overview visit chrome://bluetooth-internals/#devices in the chrome browser.
