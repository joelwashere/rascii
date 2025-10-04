## Rascii
Author: Joel Williams

Convert images to ASCII art in the terminal.

### Features
- **Simple CLI**: `rascii <image> [--width N] [--height N]`
- **Format support**: Uses the Rust `image` crate (PNG, JPEG/JPG, GIF\*, BMP, TIFF, WebP, etc.)
- **Adjustable size**: Control output width and height

\* GIFs are decoded as a single frame.

### Requirements
- **Rust** (stable) and **Cargo**

### Build
```bash
cargo build --release
```

The binary will be at `target/release/rascii`.

### Install (local project)
```bash
cargo install --path .
```

This makes `rascii` available on your `$PATH` (typically at `$HOME/.cargo/bin`).

### Usage
```bash
rascii <image_path> [--width N] [--height N]
```

#### Options
- `--width N`  Output width in characters. Default: `100`.
- `--height N` Output height in characters. Default: `100`.

Note: Terminal characters are not square. The program compensates by sampling rows at roughly half the vertical density to better preserve aspect ratio. You may need to experiment with `--width`/`--height` for your font and terminal.

### Examples
Run directly with Cargo:
```bash
cargo run -- angel.png --width 120 --height 60
```

Using the installed binary:
```bash
rascii girl.png --width 100 --height 50
```

Save output to a file:
```bash
rascii nemo.png --width 140 --height 70 > nemo.txt
```

Preview a file (macOS):
```bash
rascii test.png > out.txt && open -a TextEdit out.txt
```

### Error messages
- `Issue parsing arguments: missing file_path` — Provide an image path as the first argument.
- `Issue parsing arguments: --width requires a value` — Supply a number after `--width`.
- `Issue parsing arguments: --height requires a value` — Supply a number after `--height`.
- `Issue parsing arguments: unrecognized argument` — Remove or correct the flag.
- `Failed to convert image: ...` — The image could not be opened or decoded.

### Notes
- Large images can take longer to process; start with a smaller `--width`.
- Supported formats depend on what the `image` crate is built with in your environment.

### Author
Joel Williams


