# Flatpak Software Installer (FSI)

Flatpak Software Installer or originally called Flatpak Search and Install (FSI) is a simple/small script that allows you to search and install Flatpak applications easier.

## Table of contents

- [Flatpak Software Installer (FSI)](#flatpak-software-installer-fsi)
  - [Table of contents](#table-of-contents)
  - [Flags](#flags)
  - [How to use](#how-to-use)
    - [Cargo run](#cargo-run)
    - [Cargo build](#cargo-build)
    - [Script installation](#script-installation)
  - [TODO](#todo)

## Flags

| Long       | Short | Description                                     |
| ---------- | :---: | ----------------------------------------------- |
| `--update` | `-u`  | Also update the current installed applications. |
| `--help`   | `-h`  | Show the help message.                          |

## How to use

### Cargo run

There are two ways to run the script with cargo:

- One is to run the script with the release flag.
- The other is to run the script without the release flag.

Difference between the two:

- The release flag will compile the script with optimizations.
- The other will compile the script without optimizations.

```bash
git clone --depth 1 https://github.com/StandUp2001/fsi.git
cd fsi
cargo run --release -- <arg> [flags]
```

```bash
git clone --depth 1 https://github.com/StandUp2001/fsi.git
cd fsi
cargo run -- <arg> [flags]
```

### Cargo build

Same as the cargo run but with the build command.
So with the release it will compile the script with optimizations and without it will compile the script without optimizations.

Here we will also copy the compiled script to the /usr/local/bin/ directory. This is so that you can run the script from anywhere.

```bash
git clone --depth 1 https://github.com/StandUp2001/fsi.git
cd fsi
cargo build --release
cp target/release/fsi /usr/local/bin/
```

```bash
git clone --depth 1 https://github.com/StandUp2001/fsi.git
cd fsi
cargo build
cp target/debug/fsi /usr/local/bin/
```

With cp we copy the compiled script to the /usr/local/bin/ directory and with that we can run the script from anywhere.

```bash
fsi <arg> [flags]
```

Otherwise we can also run the script from the directory where the script is located.
In the folder `fsi` we can run the script with the following command:

```bash
./target/release/fsi <arg> [flags]
```

```bash
./target/debug/fsi <arg> [flags]
```

### Script installation

TODO!

## TODO

- [ ] Check if flatpak is installed.
- [ ] Add a way to install the application by: github or script.
- [ ] Check for updates: Check if --update has been passed.
