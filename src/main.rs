use printpdf::*;
use std::env;
use std::error::Error;
use std::fs;
use std::io;
use std::path::PathBuf;

const DPI: f32 = 300.0;

fn main() {
    if let Err(error) = run() {
        println!();
        println!("Error: {}", error);
    }

    // Always pause, even on error, so the console
    // window doesn't close before the message is read
    pause();
}

fn run() -> Result<(), Box<dyn Error>> {
    // Current working folder
    let folder = env::current_dir()?;

    // Output PDF in the same folder
    let output_pdf = folder.join("combined.pdf");

    // Find PNG/JPG/JPEG files
    let mut image_files: Vec<PathBuf> = fs::read_dir(&folder)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| {
            path.extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| {
                    matches!(
                        ext.to_ascii_lowercase().as_str(),
                        "png" | "jpg" | "jpeg"
                    )
                })
                .unwrap_or(false)
        })
        .collect();

    // Sort alphabetically by filename
    image_files.sort_by_key(|path| {
        path.file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_ascii_lowercase()
    });

    if image_files.is_empty() {
        println!("No PNG/JPG/JPEG images found in:");
        println!("{}", folder.display());
        return Ok(());
    }

    println!("Folder:");
    println!("{}", folder.display());
    println!();
    println!("Found {} image(s).", image_files.len());
    println!();

    let mut doc = PdfDocument::new("Combined Images");
    let mut pages = Vec::new();
    let mut warnings = Vec::new();

    for (index, image_path) in image_files.iter().enumerate() {
        println!(
            "[{}/{}] Adding: {}",
            index + 1,
            image_files.len(),
            image_path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
        );

        // Read image
        let image_bytes = fs::read(image_path)?;

        // Decode image
        let image = RawImage::decode_from_bytes(
            &image_bytes,
            &mut warnings,
        )
        .map_err(|error| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "Failed to decode {}: {}",
                    image_path.display(),
                    error
                ),
            )
        })?;

        // Get dimensions
        let width_px = image.width as f32;
        let height_px = image.height as f32;

        // Convert pixels to millimeters at 300 DPI
        let width_mm = width_px / DPI * 25.4;
        let height_mm = height_px / DPI * 25.4;

        // Add image to PDF
        let image_id = doc.add_image(&image);

        // Place image on the page
        let operations = vec![
            Op::UseXobject {
                id: image_id,
                transform: XObjectTransform {
                    dpi: Some(DPI),
                    ..Default::default()
                },
            }
        ];

        // Make page size match image size
        let page = PdfPage::new(
            Mm(width_mm),
            Mm(height_mm),
            operations,
        );

        pages.push(page);
    }

    println!();
    println!("Creating PDF...");

    // PDF save options.
    //
    // IMPORTANT: `PdfSaveOptions::default()` silently degrades images!
    // Its default `ImageOptimizationOptions` caps every image at an
    // estimated 2 MB of *raw* pixel data, so anything larger than about
    // 950x700 px gets downscaled with nearest-neighbor resampling and
    // then re-encoded as JPEG at quality 0.85. That is why output
    // quality was bad.
    let save_options = PdfSaveOptions {
        image_optimization: Some(ImageOptimizationOptions {
            // No size cap -> images keep their full resolution
            max_image_size: None,
            // High JPEG quality (default was 0.85).
            // Set to Some(1.0) for maximum quality, or use
            // `format: Some(ImageCompression::Flate)` for lossless
            // (much larger files).
            quality: Some(0.95),
            ..Default::default()
        }),
        ..Default::default()
    };

    // Generate PDF
    let pdf_bytes = doc
        .with_pages(pages)
        .save(
            &save_options,
            &mut warnings,
        );

    // Write PDF
    fs::write(&output_pdf, pdf_bytes)?;

    println!();
    println!("Done!");
    println!("Created:");
    println!("{}", output_pdf.display());

    if !warnings.is_empty() {
        println!();
        println!(
            "PDF library reported {} warning(s).",
            warnings.len()
        );
    }

    Ok(())
}

fn pause() {
    println!();
    println!("Press Enter to exit...");

    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
}