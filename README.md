# Vexide Template

[![Build status](https://github.com/vexide/vexide-template/actions/workflows/build.yml/badge.svg)](https://github.com/vexide/vexide-template/actions/workflows/build.yml)

> Ready-to-use template for developing VEX V5 robots in Rust.

Seasoned Vexide user? Delete README.md and update Cargo.toml as needed.

## Table of Contents

- [Vexide Template](#vexide-template)
  - [Table of Contents](#table-of-contents)
  - [Using This Template](#using-this-template)
  - [Getting Started (Windows)](#getting-started-windows)
  - [Getting Started (macOS)](#getting-started-macos)
  - [Getting Started (NixOS)](#getting-started-nixos)
  - [Getting Started (Debian/Ubuntu Linux)](#getting-started-debianubuntu-linux)
  - [Getting Started (Fedora Linux)](#getting-started-fedora-linux)
  - [Development](#development)
    - [Compiling and uploading to a VEX V5 robot](#compiling-and-uploading-to-a-vex-v5-robot)
    - [Viewing program output](#viewing-program-output)
    - [Using smart editing features](#using-smart-editing-features)
  - [Troubleshooting](#troubleshooting)

## Using This Template

To start a project using this template, click the "Use this template" button in the upper right corner of the GitHub repository. Choose an appropriate name and clone the new repository using Git. Finally, update the package name in `Cargo.toml`:

```toml
[package]
name = "my-vex-robot"
version = "0.1.0"
edition = "2021"
```

## Getting Started (Windows)

Install Rust by following the instructions on <https://rustup.rs/>.

Run the following commands in Powershell to set up your PC for development on Windows.

- Install Python 3.9:

  ```pwsh
  winget install -s msstore "Python 3.9"
  ```

- Close and reopen the terminal, and finish installing Vexide:

  ```console
  pip3.9 install --user pros-cli
  rustup default nightly
  rustup component add rust-src llvm-tools-preview
  cargo install --git "https://github.com/vexide/cargo-pros.git#feat/vexide-support"
  ```

## Getting Started (macOS)

Run the following terminal commands to set up your Mac for development.

- Install Homebrew, a package manager for macOS:

  ```console
  /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
  ```

  - Under the header "Next Steps", Homebrew may prompt you to run commands to complete the installation.

- Install Rust and PROS:

  ```console
  brew install rustup python@3.10
  rustup-init -y --default-toolchain nightly
  pip3.10 install pros-cli
  ```

- Close and reopen the terminal, and finish installing Vexide:

  ```console
  rustup component add rust-src llvm-tools-preview
  cargo install --git "https://github.com/vexide/cargo-pros.git#feat/vexide-support"
  ```

## Getting Started (NixOS)

The Nix flake includes a devshell with every tool you need for building and uploading vexide projects.
You still need to run ``rustup component add llvm-tools-preview`` if you haven't already.

There is a `.envrc` file for Nix + Direnv users.

## Getting Started (Debian/Ubuntu Linux)

Run the following terminal commands to set up your PC for development on Debian or Ubuntu.

- Install Rust:

  ```console
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

- Install Python 3.9 and PROS:

  ```console
  sudo add-apt-repository ppa:deadsnakes/ppa
  sudo apt update
  sudo apt install python3.9 python3-pip python3.9-distutils

  python3.9 -m pip install --user pros
  ```

- Close and reopen the terminal, and finish installing Vexide:

  ```console
  rustup default nightly
  rustup component add rust-src llvm-tools-preview
  cargo install --git "https://github.com/vexide/cargo-pros.git#feat/vexide-support"
  ```

## Getting Started (Fedora Linux)

Run the following terminal commands to set up your PC for development on Fedora.

- Install Rust and PROS:

  ```console
  sudo dnf install rustup python3-pip
  rustup-init -y --default-toolchain nightly
  pip install --user pros-cli
  ```

- Close and reopen the terminal, and finish installing Vexide:

  ```console
  rustup component add rust-src llvm-tools-preview
  cargo install --git "https://github.com/vexide/cargo-pros.git#feat/vexide-support"
  ```

## Development

### Compiling and uploading to a VEX V5 robot

Use the Cargo PROS terminal utility to build and upload this pros-rs project.

```console
cargo pros build
```

Use a USB cable to connect to your robot brain or to your controller before uploading. Make sure to specify a program slot and post-upload action.

```console
cargo pros upload --slot 1 --action none
```

### Viewing program output

You can view panic messages and calls to `println!()` using the PROS terminal.
Use a USB cable to connect to your robot brain or controller, then start the terminal:

```console
pros terminal --raw
```

<!--
### Debugging in the pros-rs simulator

If you have PROS Simulator installed, you can use it to run this project without real VEX hardware for debugging and development purposes. Start by adding the WebAssembly Rust target:

```console
rustup target add wasm32-unknown-unknown
```

Build the project for the simulator by running:

```console
cargo pros build -s
```

Then open this project in PROS Simulator to run and debug the robot code.
-->
### Using smart editing features

Developers using Visual Studio Code with the rust-analyzer extension have access to smart editing features like Intellisense and code analysis. By default, rust-analyzer will check the project for errors when it is saved.

## Troubleshooting

- If you get the error `TypeError: <flag 'BrainFlags'> has no members defined` when using the third party `pros upload` command, you need to downgrade Python to v3.9
