FROM rust:bullseye

RUN rustup target add armv7-unknown-linux-gnueabihf
WORKDIR /build

RUN dpkg --add-architecture armhf && \
    apt-get update && \
    apt-get install -y libc6-armhf-cross libc6-dev-armhf-cross gcc-arm-linux-gnueabihf libclang-dev
RUN apt-get install --assume-yes crossbuild-essential-armhf libdbus-1-dev:armhf libgstreamer1.0-dev:armhf libgstreamer-plugins-base1.0-dev:armhf

ENV PKG_CONFIG_LIBDIR_armv7_unknown_linux_gnueabihf=/usr/lib/arm-linux-gnueabihf/pkgconfig
