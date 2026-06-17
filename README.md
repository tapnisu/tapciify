<p align="center"><img alt="ASCII Rin Shima" width="300" src="assets/logo.webp"/></p>
<h1 align="center">Tapciify</h1>
<p align="center">CLI tool for converting your images into ASCII art</p>

## Requirements

1. CC linker (Windows - Microsoft Visual Studio with C++ Support) (Linux - gcc)

2. [Rust](https://www.rust-lang.org/tools/install)
   > **Important**:
   > Current minimal Rust version is 1.87!

## Installation

Using cargo:

```bash
cargo install tapciify
```

Using cargo-binstall:

```bash
cargo binstall tapciify -y
```

## Converting image

1. Run: `tapciify -i imagePath -w imageWidth` for image.

2. Run: `tapciify -i imagePath -w imageWidth -r` for reversed colors.

## Converting video

In this example I set framerate to 24 (but you can use any another)

> Requires ffmpeg

1. Make frames from video into dir:

   ```bash
   mkdir frames; ffmpeg -i bad_apple.mkv frames/%08d.jpeg
   ```

2. Run:

   ```bash
   tapciify -i frames/* -w videoWidth -f 24
   ```

## Examples

| Type   | Image     | Command
| ----------|-----------|------------
| Original  | <img alt="Original Image" src="assets/examples/ferris.webp" width=150> | `Original Image` 
| ASCII  | <img alt="ASCII art" src="assets/examples/ascii.webp" width=150> | `tapciify -i ./assets/examples/ferris.webp -w 64` 
| ASCII colored  | <img alt="ASCII colored art" src="assets/examples/ascii-colored.webp" width=150> | `tapciify -i ./assets/examples/ferris.webp -w 64 -c` 
| Pixels   |<img alt="ASCII art using pixels (█ symbol)" src="assets/examples/ascii-pixels.webp" width=150> | `tapciify -i ./assets/examples/ferris.webp -w 64 --pixels` 
| Braille  | <img alt="Braille" src="assets/examples/braille.webp" width=150> | `tapciify -i ./assets/examples/ferris.webp -w 64 -b` 
| Braille colored  | <img alt="Braille colored" src="assets/examples/braille-colored.webp" width=150> | `tapciify -i ./assets/examples/ferris.webp -w 64 -bc` 
| Background string | <img alt="Background string" src="assets/examples/background-string.webp" width=150> | `tapciify -i ./assets/examples/bad-apple.webp -w 64 --background-string badapple` 
