<p align="center"><img width="300" src="./assets/logo.png"/></p>

<h1 align="center">Tapciify</h1>

<p align="center">CLI tool that can show images in your terminal.</p>

## How to use:

### Images

1. Install via cargo from repo:
   `cargo install --git https://github.com/tapnisu/tapciify`

2. Run: `tapciify -f imageDir -w imageWeight` for image.

3. Run: `tapciify -f imageDir -w imageWeight -r` for reversed colors.

### Videos

> Requires ffmpeg

1. Make frames from video into dir:
   `mkdir frames; ffmpeg -i badapple.mkv -r 24 frames/%08d.jpeg`.
