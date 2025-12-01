use anyhow::{Context, Result};
use clap::Parser;
use colored::Colorize;
use std::path::PathBuf;
use std::time::Instant;
use which::which;

mod converter;
mod formats;

use converter::{ConversionType, Converter};

#[derive(Parser)]
#[command(
    name = "xfrm",
    version = "0.1.0",
    about = "a pretty quick conversion tool supporting 250+ formats",
    long_about = "converts files between formats using FFmpeg, ImageMagick, and Pandoc.\n\nExamples:\n  xfrm input.mp4 output.mp3\n  xfrm input.mov output.gif\n  xfrm input.png output.jpg\n  xfrm input.docx output.pdf"
)]
struct Cli {
    input: PathBuf,
    output: PathBuf,

    #[arg(short = 'q', long, default_value = "23")]
    quality: u8,

    #[arg(short = 'b', long)]
    bitrate: Option<String>,

    /// bitrate settings for audio related stuff
    #[arg(short = 'a', long)]
    audio_bitrate: Option<String>,

    /// image quality flag/argment wtv
    #[arg(long, default_value = "90")]
    image_quality: u8,

    #[arg(short = 'y', long)]
    overwrite: bool,

    #[arg(short = 'v', long)]
    verbose: bool,

    /// this makes sure to keeps metadata and stuff during the converison
    #[arg(short = 'm', long)]
    keep_metadata: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if !cli.input.exists() {
        anyhow::bail!("Input file does not exist: {}", cli.input.display());
    }

    if cli.output.exists() && !cli.overwrite {
        anyhow::bail!(
            "Output file already exists: {}. Use -y to overwrite.",
            cli.output.display()
        );
    }

    let converter = Converter::new(&cli.input, &cli.output)?;
    
    println!(
        "{} {} {} {}",
        "Converting".bright_blue().bold(),
        cli.input.display().to_string().bright_white(),
        "→".bright_blue(),
        cli.output.display().to_string().bright_green()
    );

    if cli.verbose {
        println!("Conversion type: {:?}", converter.conversion_type);
    }

    check_dependencies(&converter.conversion_type)?;
    let start = Instant::now();

    converter.convert(&cli)?;
    
    let duration = start.elapsed();

    println!(
        "Conversion completed in {:.2}s",
        duration.as_secs_f64()
    );

    // Display output file size
    if let Ok(metadata) = std::fs::metadata(&cli.output) {
        let size_mb = metadata.len() as f64 / 1_048_576.0;
        println!("Output size: {:.2} MB", size_mb);
    }

    Ok(())
}

fn check_dependencies(conversion_type: &ConversionType) -> Result<()> {
    match conversion_type {
        ConversionType::Video | ConversionType::Audio => {
            which("ffmpeg").context("FFmpeg is not installed. Please install FFmpeg to convert video/audio files.")?;
        }
        ConversionType::Image => {
            which("magick")
                .or_else(|_| which("convert"))
                .context("ImageMagick is not installed. Please install ImageMagick to convert image files.")?;
        }
        ConversionType::Document => {
            which("pandoc").context("Pandoc is not installed. Please install Pandoc to convert document files.")?;
        }
        ConversionType::Unknown => {
            anyhow::bail!("Unable to choose a conversion type for these file formats");
        }
    }
    Ok(())
}
