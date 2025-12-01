# Quick Start Guide

## Installation

1. **Clone the repository:**
   ```bash
   git clone https://github.com/mercyuuu/xfrm.git
   cd xfrm
   ```

2. **Run the install script:**
   ```bash
   ./install.sh
   ```

   Or manually:
   ```bash
   cargo build --release
   sudo cp target/release/xfrm /usr/local/bin/
   ```

3. **Verify installation:**
   ```bash
   xfrm --version
   ```

## Dependencies

PLEASE make sure you have the following installed:

### Arch Linux
```bash
sudo pacman -S ffmpeg imagemagick pandoc
```

### Ubuntu/Debian
```bash
sudo apt update
sudo apt install ffmpeg imagemagick pandoc
```

### macOS
```bash
brew install ffmpeg imagemagick pandoc
```

## Basic Usage

the simplest form:
```bash
xfrm input.ext output.ext
```

## Examples by Category

### Video Conversions

#### Basic video conversion
```bash
xfrm video.mp4 video.mkv
```

#### Create animated GIF from video
```bash
xfrm video.mov animation.gif
```

#### High quality conversion
```bash
xfrm input.avi output.mp4 -q 18
```

#### Custom bitrate
```bash
xfrm input.mov output.webm -b 2M
```

#### Compress video heavily
```bash
xfrm large.mp4 small.mp4 -q 35 -b 500K
```

### Audio Conversions

#### Extract audio from video
```bash
xfrm music-video.mp4 audio.mp3
```

#### Convert lossless
```bash
xfrm song.wav song.flac
```

#### High quality MP3
```bash
xfrm audio.wav audio.mp3 -a 320k
```

#### Multiple format conversions
```bash
xfrm podcast.m4a podcast.mp3
xfrm music.flac music.ogg
xfrm audio.wav audio.opus
```

### Image Conversions

#### Basic image conversion
```bash
xfrm photo.png photo.jpg
```

#### Maximum quality JPEG
```bash
xfrm image.png image.jpg --image-quality 100
```

#### Convert RAW photos
```bash
xfrm IMG_1234.CR2 IMG_1234.jpg
xfrm photo.NEF photo.png
```

#### Modern formats
```bash
xfrm old.jpg new.webp
xfrm screenshot.png screenshot.avif
```

#### Batch conversion (manual loop)
```bash
for file in *.png; do
    xfrm "$file" "${file%.png}.jpg"
done
```

### Document Conversions

#### MD to PDF
```bash
xfrm README.md README.pdf
```

#### Word to HTML
```bash
xfrm document.docx document.html
```

#### Text to EPUB
```bash
xfrm book.txt book.epub
```

#### MD to Word
```bash
xfrm notes.md notes.docx
```

## Advanced Options

### Preserve metadata
```bash
xfrm input.jpg output.png -m
```

### Overwrite existing files
```bash
xfrm input.mp4 output.mp4 -y
```

### Verbose output (allows you to see ffmpeg commands)
```bash
xfrm input.mov output.mp4 -v
```

### Option Combinations
```bash
xfrm input.avi output.mp4 -q 20 -b 1M -a 192k -y -v
```

## Troubleshooting Steps

### Error: ffmpeg not found
```bash
# Arch
sudo pacman -S ffmpeg

# Ubuntu/Debian
sudo apt install ffmpeg

# macOS
brew install ffmpeg
```

### Error: ImageMagick not found
```bash
# Arch
sudo pacman -S imagemagick

# Ubuntu/Debian
sudo apt install imagemagick

# macOS
brew install imagemagick
```

### Error: Pandoc not found
```bash
# Arch
sudo pacman -S pandoc

# Ubuntu/Debian
sudo apt install pandoc

# macOS
brew install pandoc
```

### Did the conversion fail?
Use `-v` flag to see detailed error messages:
```bash
xfrm input.mp4 output.gif -v
```

### If the outputted file already exists
Use `-y` flag to overwrite:
```bash
xfrm input.mov output.mp4 -y
```