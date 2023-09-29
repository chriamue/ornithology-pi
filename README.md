# OrnithologyPi: Bird Detection and Streaming on Raspberry Pi

OrnithologyPi is a Rust-based project that captures and analyzes a video stream to detect and identify birds. It runs on a Raspberry Pi equipped with a webcam, providing a seamless interface for observing and identifying birds in your garden or any outdoor setting.

![Overview](https://www.plantuml.com/plantuml/proxy?cache=no&src=https://raw.githubusercontent.com/chriamue/ornithology-pi/main/docs/overview.puml)

## 🎥 How It Works

1. **Capture Stream**: Utilizes a webcam to capture a live video stream.
2. **Analyze Stream**: Analyzes the stream to detect the presence of birds.
3. **Identify Birds**: Identifies the detected birds and labels them.
4. **Web App Interface**: Offers a web app for viewing the live stream and observed birds.

## 🚀 Quickstart

### Prerequisites

Ensure the installation of gstreamer development libraries for proper camera support:

```sh
apt install libgstreamer1.0-dev libgstreamer-plugins-base1.0-dev v4l-utils
```

## 🖥️ Running the Application and CLI Parameters

OrnithologyPi provides a command-line interface for easy configuration and operation. You can control the features like server, hotspot, bluetooth, and detect through command-line arguments. Below is the information on how to use these parameters:

### Command Line Parameters:

- `--server`: Activate server mode. This is enabled by default if the server feature is compiled.
- `--hotspot`: Activate hotspot mode.
- `--bluetooth`: Activate bluetooth mode.
- `--detect`: Activate detect mode.

### Example:

To run the application with server mode, use the following command:

```sh
cargo run --release --features=server -- --server=true
```

This command will compile and run the OrnithologyPi application with the server feature enabled and the server mode activated.

To disable the server mode (which is enabled by default), use:

```sh
cargo run --release --features=server -- --server=false
```

### Error Handling:

If a feature is set on, but not compiled with the application, an error message will be displayed indicating that the feature is not enabled.

### Running the Application:

1. Open a terminal in the project's root directory.
2. Run the application with the desired parameters. For example:

```sh
cargo run --release --features=server,bluetooth -- --server=true --bluetooth=true
```

This command will run the application with server and bluetooth features enabled.

### Note:

Ensure that the features you are trying to use are enabled during compilation. If a feature is not compiled and you try to use it, an error message will be displayed.

By using the command-line interface, you can easily control the features and functionality of the OrnithologyPi application, tailoring it to your specific needs and environment.


### Examples

- **Webcam Example**: Save output at `frame.jpg`.

  ```sh
  cargo run --example webcam
  ```

- **Crop Example**: Save the cropped bird image at `crop.jpg`.

  ```sh
  cargo run --example crop
  ```

- **Label Example**: Print the detected bird on the image.

  ```sh
  cargo run --example label
  ```

- **Window Example**: Navigate through frames and exit as needed.

  ```sh
  cargo run --features="window" example window
  ```

- **Web App with yolov8 backend**: Start the web app and navigate to `http://localhost:8080`.

  ```sh
  cargo run --features="server,detect,yolov8" --no-default-features --release
  ```

## 📦 Building and Installation

### Build Debian Package

```sh
cargo install cargo-deb
cargo deb
```

### Build App

![App](docs/app_screenshot.png)

```sh
mkdir build
cd build
cmake ../ornithology-app
```

### Build for Raspberry Pi

Utilize crossbuild for building for Raspberry Pi:

```sh
cargo install cross
cargo install cargo-deb
docker build -t crossbuild:local -f crossbuild.Dockerfile .
cross build --release --target armv7-unknown-linux-gnueabihf
cargo deb --no-build --target armv7-unknown-linux-gnueabihf
```

### Install on Raspberry Pi

Install the necessary plugins and the application:

```sh
sudo apt install gstreamer1.0-plugins-good
wget https://github.com/chriamue/ornithology-pi/releases/latest/download/ornithology-pi.deb
sudo apt install ./ornithology-pi.deb
```

To start the service:

```sh
sudo service ornithology-pi start
```

To enable the service on boot:

```sh
sudo systemctl enable ornithology-pi
```

## 🛠️ Using the Makefile

For ease of use, OrnithologyPi includes a `Makefile` that automates various tasks, allowing you to focus on development without worrying about the underlying commands. Below are the available make commands and their descriptions:

- `make server`: This command runs the server with specific features enabled: `server`, `detect`, and `yolov8`. It runs the server in release mode with no default features.
- `make build`: This command builds the project.
- `make test`: This command runs all the tests in the project.
- `make app`: This command serves the app, allowing you to access it from a web browser.
- `make mobile`: This command runs mobile development for Android.

### How to Use the Makefile

1. Open a terminal in the project's root directory.
2. Type the make command you want to use, for example, `make server` to run the server, and press `Enter`.

This will execute the corresponding command as defined in the `Makefile`, saving you time and ensuring consistency in development tasks.

### Example

To run the server, simply type the following command in your terminal and press `Enter`:

```sh
make server
```

This will execute the `cargo run --features="server,detect,yolov8" --no-default-features --release` command as defined in the `Makefile`.

By using the `Makefile`, you can easily and consistently manage and run tasks without having to remember or type out the full commands each time.

## 🐞 Debugging

### Bluetooth Debugging

For a Bluetooth services overview, visit `chrome://bluetooth-internals/#devices` in the Chrome browser.

### Known Issues

- **Bluetooth Issue**: Run the app with `BLUETOOTH_FORCE_DBUS_LE_VERSION=1` to resolve Bluetooth issues.

## 📺 [Demo Video](https://www.youtube.com/watch?v=OJHczE3-kko)

Watch the OrnithologyPi in action:

[![Video](https://img.youtube.com/vi/OJHczE3-kko/0.jpg)](https://www.youtube.com/watch?v=OJHczE3-kko)
