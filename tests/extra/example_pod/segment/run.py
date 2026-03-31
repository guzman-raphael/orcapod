import os
from pathlib import Path
from PIL import Image
import numpy as np
from time import sleep
import csv


def configure_runtime():
    if os.getenv("RAISE_ERROR", "false").upper() == "TRUE":
        raise Exception("Raising requested error...")

    delay = int(os.getenv("DELAY", "0"))
    print(f"Waiting for {delay}[s]...", flush=True)
    sleep(delay)


def setup_io():
    with open("/input/config1.csv") as c1, open("/extra_configs/config2.csv") as c2:
        return {
            "input_path": Path("/input/subject.jpeg"),
            "output_configs": {
                Path("/output/overlay_image1.png"): list(
                    csv.reader(c1, quoting=csv.QUOTE_NONNUMERIC)
                ),
                Path("/output/overlay_image2.png"): list(
                    csv.reader(c2, quoting=csv.QUOTE_NONNUMERIC)
                ),
            },
        }


def segment(subject_image_path):
    # Normalize intensity to [0, 1]
    normalized_image = (
        np.array(Image.open(subject_image_path).convert("L")).astype("float32") / 255.0
    )

    # Threshold-based segmentation into 4 intensity classes
    mask = np.zeros_like(normalized_image, dtype=np.uint8)
    mask[normalized_image < 0.05] = 0
    mask[(normalized_image >= 0.05) & (normalized_image < 0.25)] = 1
    mask[(normalized_image >= 0.25) & (normalized_image < 0.6)] = 2
    mask[normalized_image >= 0.6] = 3

    return mask


def save_overlay(mask, overlay_image_path, overlay_rgb_config):
    # Create overlay for visualization
    palette = np.array(overlay_rgb_config, dtype=np.uint8)
    overlay_image = palette[mask]

    Image.fromarray(overlay_image).save(overlay_image_path, quality=95)


configure_runtime()
io_config = setup_io()
mask = segment(io_config["input_path"])
for output_path, output_config in io_config["output_configs"].items():
    save_overlay(mask, output_path, output_config)

print("done")
