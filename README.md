# Versionbump - a version bump tool for BaT deployments

<img src="/docs/screenshot.png" alt="screenshot" width="400">

Versionbump is a simple version bumping tool built in Rust. It allows you to search and replace across different hardcoded file paths for BaT releases. Created to simplify version bumping and speed up deployments.

## Features

* Hardcoded paths for BaT deployment version bumps
* Install one executable
* Cross-platform support for OS X, Windows, and Linux

## Prioritization

Keep it simple and fast.

## Installation

### Install manually from source

1. [Install Rust 1.40 or later](https://www.rust-lang.org/tools/install)
1. Run `git clone https://github.com/renerodr/versionbump` and `cd versionbump`
1. Run `cargo install --path .`
1. Ensure `~/.cargo/bin` is in your `$PATH`

## Usage

1. Run `cd bringatrailer-docker/wordpress`
1. Run `versionbump 5.3.13 5.3.14`
