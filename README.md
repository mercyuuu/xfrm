# xfrm - A (hopefully) quick file conversion tool
## Features

- **250 (or so) Formats** - Convert between video, audio, image, and document formats
- **Smart Conversion** - Automatically detects conversion type from file extensions
- **Quality Control** - Fine-tune output quality, bitrate, and compression settings
- **SIMPLE Metadata Handling** - Option to preserve or strip metadata

## Installation

### Prerequisites

`xfrm` uses popular well known tools for its conversions:

- **ffmpeg** - For video/audio conversions
- **ImageMagick** - For image conversions
- **Pandoc** - For document conversions

#### Arch Linux
```bash
sudo pacman -S ffmpeg imagemagick pandoc
```

#### Ubuntu/Debian
```bash
sudo apt update
sudo apt install ffmpeg imagemagick pandoc
```

#### macOS
# Just build from source, or use a compiled binary from the [releases](https://github.com/mercyuuu/xfrm/releases/), As i do not wish to go on and add to brew

### Installing xfrm

#### From Source
```bash
# Clone the repository
git clone https://github.com/mercyuuu/xfrm.git
cd xfrm

# Build and install
cargo install --path .
```

## Usage

Basic syntax:
```bash
xfrm <input> <output>
```

the output format is automatically determined from the file extension.

### Examples

#### Video Conversions
```bash
# MP4 to MKV
xfrm input.mp4 output.mkv

# GIF from video
xfrm input.mov output.gif

# Convert with custom quality
xfrm input.mp4 output.webm -q 30

# Convert with custom bitrate
xfrm input.avi output.mp4 -b 2M
```

#### Audio Conversions
```bash
# Extract audio from video
xfrm input.mp4 output.mp3

# WAV to FLAC
xfrm input.wav output.flac

# Convert with custom audio bitrate
xfrm input.m4a output.mp3 -a 320k
```

#### Image Conversions
```bash
# PNG to JPEG
xfrm input.png output.jpg

# Convert with quality controling
xfrm input.png output.jpg --image-quality 95

# RAW to PNG
xfrm input.cr2 output.png

# HEIC to JPEG
xfrm photo.heic photo.jpg
```

#### Document Conversions
```bash
# MD to PDF
xfrm README.md README.pdf

# DOCX to HTML
xfrm document.docx document.html

# Text to EPUB
xfrm book.txt book.epub
```

## Options

```
Options:
  -q, --quality <QUALITY>
          Video quality (0-51, lower is better) [default: 23]

  -b, --bitrate <BITRATE>
          Video bitrate (e.g., 1M, 500K)

  -a, --audio-bitrate <AUDIO_BITRATE>
          Audio bitrate (e.g., 192k, 320k)

      --image-quality <IMAGE_QUALITY>
          Image quality (1-100) [default: 90]

  -y, --overwrite
          Overwrite output file if it exists

  -v, --verbose
          Verbose output (show conversion commands)

  -m, --keep-metadata
          Keep metadata from original file

  -h, --help
          Print help

  -V, --version
          Print version
```

## Supported Formats

### Video (25+)
mp4, avi, mkv, mov, wmv, flv, webm, m4v, mpg, mpeg, 3gp, ogv, mxf, and more

### Audio (25+)
mp3, wav, flac, aac, ogg, opus, m4a, wma, aiff, alac, and more

### Images (50+)
jpg, png, gif, bmp, webp, svg, tiff, ico, heic, avif, raw, cr2, nef, arw, dng, psd, and more

### Documents (30+)
pdf, docx, txt, md, html, epub, odt, rtf, latex, and more

## Performance

`xfrm` is built in Rust with pretty aggressive optimizations:
- **Stripped binaries** - Minimal binary size
- **Attempte Zero-copy where possible** - Efficient memory usage
- **Parallel processing** - Which utilizes all CPU cores via 

## Benchmarks

When compared to manual ffmpeg/ImageMagick commands, `xfrm` adds a very slight overhead (<5ms) while providing a much simpler interface.

## Inspiration

inspired by [VERT](https://github.com/VERT-sh/VERT), the next-generation web-based file converter.
<3333

## License

AGPLv3 License - see [LICENSE](LICENSE) for details

## Contributing

contributions are welcome!!! please feel free to submit a pr

## Roadmap
- [ ] Batch conversion support
- [ ] Configuration file support
- [ ] Custom ffmpeg/ImageMagick presets
- [ ] Progress bar for long conversions
- [ ] Parallel batch processing
- [ ] Web UI (optional)
- [ ] Container format detection
- [ ] Format validation before conversion

## FAQ

**Q: Why use xfrm instead of ffmpeg directly?**  
A: xfrm provides a simpler, more easy to work with interface with smart defaults while being just as fast.

**Q: Does xfrm work offline?**  
A: yup, so long as you've got all of the deps installed it should work fine.

**Q: What if a conversion fails?**  
A: use the `-v` flag for verbose output to see the raw command and error messages.

## Support
If you encounter any issues or have questions:
- Open an issue (https://github.com/mercyuuu/xfrm/issues)
- Check existing issues for solutions
- Use `-v` flag for detailed error messages
