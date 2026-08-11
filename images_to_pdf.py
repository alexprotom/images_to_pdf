from pathlib import Path
from PIL import Image

# Current folder
folder = Path.cwd()

# Output PDF in the same folder
output_pdf = folder / "combined.pdf"

# Supported image extensions
extensions = {".png", ".jpg", ".jpeg"}

# Find and sort images
image_files = sorted(
    [f for f in folder.iterdir() if f.suffix.lower() in extensions],
    key=lambda x: x.name.lower()
)

if not image_files:
    raise RuntimeError("No PNG/JPG images found in the current folder.")

images = []

for image_file in image_files:
    print(f"Adding: {image_file.name}")

    with Image.open(image_file) as img:
        # Handle transparency and convert to RGB
        if img.mode in ("RGBA", "LA"):
            background = Image.new("RGB", img.size, "white")
            background.paste(img, mask=img.getchannel("A"))
            img = background
        else:
            img = img.convert("RGB")

        images.append(img.copy())

# Create PDF
images[0].save(
    output_pdf,
    "PDF",
    save_all=True,
    append_images=images[1:]
)

print(f"\nPDF created: {output_pdf}")