# Mobile App

## Introduction

The mobile app is a Yew app that is compiled to WebAssembly and runs in Tauri.

## Development

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Tauri](https://next--tauri.netlify.app/next/mobile/)

```bash
cargo install create-tauri-app
cargo create-tauri-app
cargo install tauri-cli@2.0.0-alpha.14
```

### Build

```bash
cd app
cargo tauri android init
cargo tauri build
cargo tauri android build
```

### Run

```bash
cargo tauri android dev
```

### Deploy

```bash
cargo tauri android build
keytool -genkey -v -keystore release.keystore -alias app -keyalg RSA -keysize 2048 -validity 10000
$ANDROID_HOME/build-tools/33.0.1/apksigner sign --ks release.keystore --out app.apk src-tauri/gen/android/app/build/outputs/apk/universal/release/app-universal-release-unsigned.apk
adb install -r app.apk
```
