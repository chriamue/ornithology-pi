# OrnithologyPi

Capture birds in your garden, running on raspberry pi.

[![Wichteln](https://img.youtube.com/vi/OJHczE3-kko/0.jpg)](https://www.youtube.com/watch?v=OJHczE3-kko)

![Overview](https://www.plantuml.com/plantuml/proxy?cache=no&src=https://raw.githubusercontent.com/chriamue/ornithology-pi/main/docs/overview.puml)

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

```sh
cargo install cargo-deb
cargo deb
```

## Build App

```sh
mkdir build
cd build
cmake ../ornithology-app
```

There seems to be an issue with bluetooth.
A solution is to run the app with

`BLUETOOTH_FORCE_DBUS_LE_VERSION=1`

### Build for Raspberry Pi

#### Crossbuild

```sh
cargo install cross
cargo install cargo-deb
docker build -t crossbuild:local -f crossbuild.Dockerfile .
cross build --release --target armv7-unknown-linux-gnueabihf
cargo deb --no-build --target armv7-unknown-linux-gnueabihf
```

### Install on Raspberry Pi

You need some requirements:

```sh
sudo apt install gstreamer1.0-plugins-good
```

Now download and install.

```sh
wget https://github.com/chriamue/ornithology-pi/releases/latest/download/ornithology-pi.deb
sudo apt install ./ornithology-pi.deb
```

To start the service run

```sh
sudo service ornithology-pi start
```

If you want to start the service on boot, run

```sh
sudo systemctl enable ornithology-pi
```

#### Iptables

```sh
sudo iptables -t nat -A PREROUTING -s 10.42.0.0/24 -p tcp --dport 80 -j DNAT --to-destination 127.0.0.1:8000
sudo iptables -t nat -A POSTROUTING -s 10.42.0.0/24 -j MASQUERADE
```

## Debug

### Bluetooth

For a bluetooth services overview visit chrome://bluetooth-internals/#devices in the chrome browser.
