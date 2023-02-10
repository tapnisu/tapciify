<p align="center"><img width="300" src="./assets/logo.png"/></p>

<h1 align="center">Tapciify</h1>

<p align="center">CLI tool that can let you view images in terminal</p>

## How to use

### Install

1. Install [rustup](https://rustup.rs/)

2. Run:

```bash
rustup default stable
```

3. Install via cargo from crates.io:

```bash
cargo install tapciify
```

4. Windows: Install Microsoft Visual Studio with C++ support.

4. Linux: Install cc-linker (Ubuntu)

```bash
apt install build-essential
```

### Images

1. Run: `tapciify -i imageDir -w imageWidth` for image.

2. Run: `tapciify -i imageDir -w imageWidth -r` for reversed colors.

### Videos

> Requires ffmpeg

1. Make frames from video into dir:

```bash
mkdir frames; ffmpeg -i badapple.mkv -r 24 frames/%08d.jpeg
```

2. Run:

```bash
tapciify -i dir -w videoWidth -d -f 24
```

- `-d` set mode to video
- `-f 24` - set fps for video

## Examples

| Original                                               | Ascii                                      | Ascii colored                                    |
| ------------------------------------------------------ | ------------------------------------------ | ------------------------------------------------ |
| ![Original Image](assets/original.png)                 | ![Ascii image](assets/ascii.png)           | ![Ascii colored image](assets/ascii_colored.png) |
| `Original image (Shima Rin from Laid-Back Camp manga)` | `tapciify -i ./assets/original.png -w 100` | `tapciify -i ./assets/original.png -w 100 -c`    |
