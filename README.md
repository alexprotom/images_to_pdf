# images_to_pdf

Combine all images in a folder into a single PDF.

Drop the tool into a folder with `.png` / `.jpg` / `.jpeg` files and run it. It collects the images, sorts them alphabetically by filename (case-insensitive), and writes them as one `combined.pdf` in the same folder, one image per page. Each page is sized to match its image at 300 DPI, and images keep their full resolution.

Two implementations are included: the original Python script and a Rust rewrite that builds into a single self-contained `.exe` with no runtime dependencies.

## Rust version (recommended)

A standalone executable, no Python or any other runtime needed on the machine where it runs.

### Dependencies

| Dependency | Version | Purpose |
|---|---|---|
| [Rust toolchain](https://rustup.rs) | 1.85+ (edition 2024) | build only |
| [printpdf](https://crates.io/crates/printpdf) | 0.12.5 (features `png`, `jpeg`) | PDF generation and image decoding |

`printpdf` is fetched automatically by Cargo; you only need the Rust toolchain installed to build.

### Build

```bash
git clone https://github.com/alexprotom/images_to_pdf
cd images_to_pdf
cargo build --release
```

The finished binary is at `target/release/images_to_pdf.exe` (about 2 MB, statically linked).

### Use

Copy `images_to_pdf.exe` into the folder with your images and double-click it (or run it from a terminal in that folder). It prints the images as it adds them, writes `combined.pdf` next to them, and waits for Enter before closing so you can read the output.

### Note on image quality

The code deliberately overrides `printpdf`'s default save options. The library's `PdfSaveOptions::default()` caps every image at ~2 MB of raw pixel data, downscaling anything larger than roughly 950×700 px with nearest-neighbor resampling before re-encoding at JPEG quality 0.85, which visibly degrades photos and scans. This tool instead saves with no size cap and JPEG quality 0.95, so images keep their original resolution. For fully lossless (but much larger) output, see the comment in `src/main.rs` about `ImageCompression::Flate`.

## Python version

The original script, kept for reference. Requires an installed Python and one package.

### Dependencies

| Dependency | Version | Purpose |
|---|---|---|
| [Python](https://www.python.org) | 3.8+ | runtime |
| [Pillow](https://pypi.org/project/pillow/) | any recent | image decoding and PDF writing |

### Install

```bash
pip install Pillow
```

### Use

Run the script from the folder with your images:

```bash
cd path/to/your/images
python path/to/images_to_pdf.py
```

It writes `combined.pdf` into the current folder. Images with transparency (RGBA/LA) are flattened onto a white background before saving.

## Differences between the two versions

Both versions find the same files, sort them the same way, and produce one page per image. The Rust version sizes pages at 300 DPI (a 3000×2400 px image becomes a 254×203 mm page) and keeps transparency as a PDF soft mask rendered over the white page; the Python version uses Pillow's default 72 DPI page sizing and flattens transparency onto white before embedding.
## License

[MIT](LICENSE) © 2026 Alexander Pryanichnikov
