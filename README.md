<p align="center"><img alt="ASCII Rin Shima" width="300" src="assets/logo.webp"/></p>
<h1 align="center">Tapciify</h1>
<p align="center">CLI tool for converting your images into ASCII art</p>

## Requirements

1. CC linker (Windows - Microsoft Visual Studio with C++ Support) (Linux - gcc)

2. [Rust](https://www.rust-lang.org/tools/install)
   > **Important**:
   > Current minimal Rust version is 1.85!

## Installation

Using cargo:

```bash
cargo install tapciify --locked
```

Using cargo-binstall:

```bash
cargo binstall tapciify --locked -y
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

| Original                                       | ASCII                                             | ASCII colored                                            | Pixels                                                                  | Braille                                              | Braille colored                                          | Background string                                                                 |
| ---------------------------------------------- | ------------------------------------------------- | -------------------------------------------------------- | ----------------------------------------------------------------------- | ---------------------------------------------------- | -------------------------------------------------------- | --------------------------------------------------------------------------------- |
| ![Original Image](assets/examples/ferris.webp) | ![ASCII art](assets/examples/ascii.webp)          | ![ASCII colored art](assets/examples/ascii-colored.webp) | ![ASCII art using pixels (█ symbol)](assets/examples/ascii-pixels.webp) | ![Braille](assets/examples/braille.webp)             | ![Braille colored](assets/examples/braille-colored.webp) | ![Background string](assets/examples/background-string.webp)                      |
| `Original image`                               | `tapciify -i ./assets/examples/ferris.webp -w 64` | `tapciify -i ./assets/examples/ferris.webp -w 64 -c`     | `tapciify -i ./assets/examples/ferris.webp -w 64 --pixels`              | `tapciify -i ./assets/examples/ferris.webp -w 64 -b` | `tapciify -i ./assets/examples/ferris.webp -w 64 -bc`    | `tapciify -i ./assets/examples/bad-apple.webp -w 64 --background-string badapple` |
