# Standard Operating Procedures (SOPs) Documentation

This document outlines the procedures and tools used in the `fixed_point_experiments` project, covering setup, refactoring, visualization, and quality assurance.

## 1. Project Setup

### 1.1 Prerequisites

Before starting, ensure the following tools are installed on your system:

*   **Rust Toolchain:** Follow the official Rust installation guide: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
*   **FFmpeg:** A powerful multimedia framework. Installation varies by OS (e.g., `sudo apt install ffmpeg` on Debian/Ubuntu, `brew install ffmpeg` on macOS).
*   **jq:** A lightweight and flexible command-line JSON processor. (e.g., `sudo apt install jq` or `brew install jq`).

### 1.2 Repository Cloning (Implied)

Ensure you have cloned the project repository to your local machine.

### 1.3 Initial Build

Navigate to the project root (`~/storage/github/rustc/`) and perform an initial build to download dependencies and compile the project:

```bash
cargo build -p fixed_point_experiments
```

## 2. Refactoring `fixed_point_experiments` Crate

The `fixed_point_experiments` crate was refactored to improve modularity and maintainability. The monolithic `src/main.rs` was split into specialized modules, and a configuration management system was introduced.

### 2.1 Module Structure

*   `src/main.rs`: Main application entry point, orchestrating the process.
*   `src/image_utils.rs`: Contains utilities for image manipulation, ASCII art rendering, and image analysis.
*   `src/slime_mold.rs`: Implements the core slime mold simulation logic.
*   `src/sat_solver.rs`: Handles SAT problem solving and vertex cover calculations.
*   `src/config.rs`: Defines data structures for application configuration.

### 2.2 Dependency Changes (`Cargo.toml`)

The following dependencies were added to `crates/introspector/fixed_point_experiments/Cargo.toml` to support configuration parsing:

```toml
serde_yaml = "0.9"
clap = { version = "4.0", features = ["derive"] }
toml = "0.8"
```

### 2.3 Function Signature and Configuration Integration

Global constants for image dimensions and other parameters were removed and replaced with an `ImageConfig` struct. Functions in `image_utils.rs` and `slime_mold.rs` were updated to accept `&ImageConfig` as a parameter, promoting better parameter management and flexibility.

## 3. Slime Concentration Debug Visualization

To better understand the slime mold simulation, a debug visualization mode was implemented. This generates ASCII art and PNG images for each simulation step, and then compiles them into animated GIFs.

### 3.1 `main.rs` Logic

The `main.rs` file was modified to:

*   Iterate through a predefined set of prime numbers (`zos_primes`), using each as a grid size and number of iterations for the slime simulation.
*   For each iteration, calculate the slime concentration at the center of the grid.
*   Generate an ASCII art representation of the current slime grid.
*   Render this ASCII art onto a fixed-size screen image (576x1024 pixels) and center it.
*   Save the screen image as a PNG file in a dedicated debug directory (`debug_frames/grid_XxY/`).
*   Print `ffmpeg` commands to the console for later GIF generation.

### 3.2 `run_and_log.sh` Script

This script automates the build and execution of the Rust program, and logs all output. It also handles the extraction of audio and video frames from a source MP4 file, which can be used for more advanced visualizations.

```bash
#!/bin/bash

LOG_FILE="build_and_run.log"
INPUT_VIDEO="/data/data/com.termux/files/home/downloads/a65d4a9780457d4f14629415016964c4.mp4"
EXTRACTED_MEDIA_DIR="extracted_media"
EXTRACTED_FRAMES_DIR="${EXTRACTED_MEDIA_DIR}/frames"
EXTRACTED_AUDIO_FILE="${EXTRACTED_MEDIA_DIR}/audio.aac"

echo "Starting build and run process. Output will be logged to $LOG_FILE" | tee -a "$LOG_FILE"
echo "------------------------------------------------------------------" | tee -a "$LOG_FILE"

# Clean and re-create extracted media directories
echo "Cleaning and re-creating extracted media directories..." | tee -a "$LOG_FILE"
rm -rf "$EXTRACTED_MEDIA_DIR" 2>&1 | tee -a "$LOG_FILE"
mkdir -p "$EXTRACTED_FRAMES_DIR" 2>&1 | tee -a "$LOG_FILE"

# Extract audio
echo "Extracting audio from video..." | tee -a "$LOG_FILE"
ffmpeg -y -i "$INPUT_VIDEO" -vn "$EXTRACTED_AUDIO_FILE" 2>&1 | tee -a "$LOG_FILE"
if [ $? -ne 0 ]; then
    echo "Audio extraction failed. Check $LOG_FILE for details." | tee -a "$LOG_FILE"
    exit 1
fi

# Extract video frames
echo "Extracting video frames..." | tee -a "$LOG_FILE"
ffmpeg -y -i "$INPUT_VIDEO" "${EXTRACTED_FRAMES_DIR}/frame_%05d.png" 2>&1 | tee -a "$LOG_FILE"
if [ $? -ne 0 ]; then
    echo "Frame extraction failed. Check $LOG_FILE for details." | tee -a "$LOG_FILE"
    exit 1
fi

echo "Media extraction complete. Building and running Rust executable..." | tee -a "$LOG_FILE"

# Build the project
cargo build -p fixed_point_experiments 2>&1 | tee -a "$LOG_FILE"

# Check if build was successful
if [ $? -ne 0 ]; then
    echo "Build failed. Check $LOG_FILE for details." | tee -a "$LOG_FILE"
    exit 1
fi

echo "Build successful. Running executable..." | tee -a "$LOG_FILE"

# Run the executable
/data/data/com.termux/files/home/storage/github/rustc/crates/introspector/target/debug/fixed_point_experiments 2>&1 | tee -a "$LOG_FILE"

# Check if execution was successful
if [ $? -ne 0 ]; then
    echo "Execution failed. Check $LOG_FILE for details." | tee -a "$LOG_FILE"
    exit 1
fi

echo "------------------------------------------------------------------" | tee -a "$LOG_FILE"
echo "Process completed. All output saved to $LOG_FILE" | tee -a "$LOG_FILE"
```

### 3.3 GIF Generation

After `run_and_log.sh` completes, it prints `ffmpeg` commands to the console (and log file). These commands can be executed to generate animated GIFs for each grid size. Example command:

```bash
ffmpeg -y -i debug_frames/grid_XxY/frame_%05d.png -start_number 0 -vf "fps=10,scale=576:1024" debug_frames/grid_XxY.gif
```

## 4. Original Meme Frame ASCII Art Conversion

This process converts the original video frames into ASCII art representations, saved as individual text files.

### 4.1 `convert_meme_frames_to_ascii.sh` Script

```bash
#!/bin/bash

INPUT_DIR="/data/data/com.termux/files/home/storage/github/rustc/crates/introspector/extracted_media/frames/"
OUTPUT_DIR="/data/data/com.termux/files/home/storage/github/rustc/crates/introspector/extracted_media/ascii_frames_original_meme/"
MONOCHORA_PATH="/data/data/com.termux/files/home/.cargo/bin/monochora"

mkdir -p "$OUTPUT_DIR"

echo "Converting original meme frames to ASCII art..."

for i in $(seq -f "%05g" 1 624); do
    INPUT_FILE="${INPUT_DIR}/frame_${i}.png"
    OUTPUT_FILE="${OUTPUT_DIR}/frame_${i}.txt"
    echo "Processing ${INPUT_FILE} -> ${OUTPUT_FILE}"
    "$MONOCHORA_PATH" -i "$INPUT_FILE" -o "$OUTPUT_FILE"
done

echo "ASCII art conversion complete."
```

## 5. QA SOP (Initial Draft)

### 5.1 Purpose

To ensure the slime mold simulation functions correctly across various grid sizes (1-23), and to collect performance and output data for analysis.

### 5.2 Scope

Testing the `fixed_point_experiments` Rust crate.

### 5.3 Prerequisites

*   Rust toolchain installed.
*   `ffmpeg` installed.
*   `jq` installed.
*   The `fixed_point_experiments` project set up and built.
*   `monochora` installed (`cargo install monochora`).

### 5.4 Procedure

1.  **Setup:**
    *   Ensure all prerequisites are met.
    *   Navigate to the project root: `~/storage/github/rustc/crates/introspector/`.
    *   Ensure the input video file (`a65d4a9780457d4f14629415016964c4.mp4`) is present in `~/storage/downloads/`.

2.  **Execution:**
    *   Run the main QA script:
        ```bash
        bash ../../run_qa.sh
        ```
    *   This script will:
        *   Clean and re-extract audio and video frames.
        *   Build and run the Rust program for each prime size (1, 2, 3, 5, ..., 23).
        *   Log detailed introspection for each step to `qa_logs/qa_run_prime_XxY.log`.
        *   Generate debug PNG frames in `debug_frames/grid_XxY/`.
        *   Print `ffmpeg` commands to generate GIFs from these debug frames.

3.  **Data Collection:**
    *   Review the `qa_logs/qa_run_prime_XxY.log` files for detailed output and timing information.
    *   Inspect the generated debug PNGs in `debug_frames/grid_XxY/`.
    *   Execute the `ffmpeg` commands (printed in the logs) to generate the animated GIFs in `debug_frames/`.
    *   Run the `convert_meme_frames_to_ascii.sh` script to generate ASCII art files from the original meme frames.

4.  **Inspection:**
    *   Visually inspect the generated GIFs for correctness and visual quality.
    *   Review the slime concentration values in the logs for expected behavior.
    *   Examine the ASCII art files for proper conversion.

5.  **Reporting:**
    *   Summarize findings, including any anomalies or unexpected behavior.
    *   Provide performance metrics (e.g., execution times from logs).
    *   Attach relevant generated media (GIFs, sample ASCII art files) to the report.
