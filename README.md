# Sensible Web Defaults For Rust

## Pre-requisites
 - Docker
 - Docker-compose

## Getting started
To get started, run `docker-compose up`
Development with such tools as `cargo-watch` is possible, as the `src` directory is two-way shared into the image.

## Deployment

Deployment should always be with the tagged image that `docker build . -t myapp` builds, as it has been stripped down to just a bare `alpine` os and the project's rust-built binary.

![dockerbuild.png]()
