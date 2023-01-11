<p align="center"><img width="300" src="./assets/logo.png"/></p>

<h1 align="center">Tapciify</h1>

<p align="center">CLI tool that can show images in your terminal.</p>

## How to use

### Install

1. Install [rustup](https://rustup.rs/)

2. Run:

```bash
rustup default stable 
```

2. Install via cargo from repo:
   `cargo install --git https://github.com/tapnisu/tapciify`

3. Windows: Install Microsoft Visual Studio with C++ support.

3. Linux: Install cc-linker (Ubuntu) `apt install build-essential`

### Images

1. Run: `tapciify -i imageDir -w imageWidth` for image.

2. Run: `tapciify -i imageDir -w imageWidth -r` for reversed colors.

### Videos

> Requires ffmpeg

1. Make frames from video into dir:
   `mkdir frames; ffmpeg -i badapple.mkv -r 24 frames/%08d.jpeg`.

2. Run: `tapciify -i dir -w videoWidth -d -f 24`

`-f 24` - fps
