use anyhow::{Context, Result};
use std::path::Path;
use std::process::{Command, Stdio};
use colored::Colorize;

use crate::formats;
use crate::Cli;

#[derive(Debug)]
pub enum ConversionType {
    Video,
    Audio,
    Image,
    Document,
    Unknown,
}

pub struct Converter {
    pub conversion_type: ConversionType,
    input_format: String,
    output_format: String,
}

impl Converter {
    pub fn new(input: &Path, output: &Path) -> Result<Self> {
        let input_ext = input
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();

        let output_ext = output
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();

        let conversion_type = Self::determine_type(&input_ext, &output_ext);

        Ok(Self {
            conversion_type,
            input_format: input_ext,
            output_format: output_ext,
        })
    }

    fn determine_type(input_ext: &str, output_ext: &str) -> ConversionType {
        if (formats::is_video_format(input_ext) && output_ext == "gif")
            || (input_ext == "gif" && formats::is_video_format(output_ext))
        {
            return ConversionType::Video;
        }

        if formats::is_video_format(input_ext) && formats::is_video_format(output_ext) {
            return ConversionType::Video;
        }

        if (formats::is_audio_format(input_ext) && formats::is_audio_format(output_ext))
            || (formats::is_video_format(input_ext) && formats::is_audio_format(output_ext))
        {
            return ConversionType::Audio;
        }
        
        if formats::is_image_format(input_ext) && formats::is_image_format(output_ext) {
            return ConversionType::Image;
        }

        if formats::is_document_format(input_ext) && formats::is_document_format(output_ext) {
            return ConversionType::Document;
        }

        ConversionType::Unknown
    }

    pub fn convert(&self, cli: &Cli) -> Result<()> {
        match self.conversion_type {
            ConversionType::Video => self.convert_video(cli),
            ConversionType::Audio => self.convert_audio(cli),
            ConversionType::Image => self.convert_image(cli),
            ConversionType::Document => self.convert_document(cli),
            ConversionType::Unknown => {
                anyhow::bail!("Unsupported conversion: {} → {}", self.input_format, self.output_format)
            }
        }
    }

    fn convert_video(&self, cli: &Cli) -> Result<()> {
        let mut args = vec![
            "-i".to_string(),
            cli.input.to_str().unwrap().to_string(),
        ];
        if cli.output.extension().and_then(|e| e.to_str()) == Some("gif") {
            args.extend([
                "-vf".to_string(),
                "fps=15,scale=480:-1:flags=lanczos,split[s0][s1];[s0]palettegen[p];[s1][p]paletteuse".to_string(),
            ]);
        } else {
            args.extend([
                "-c:v".to_string(),
                "libx264".to_string(),
                "-crf".to_string(),
                cli.quality.to_string(),
                "-preset".to_string(),
                "medium".to_string(),
            ]);

            if let Some(ref bitrate) = cli.bitrate {
                args.extend(["-b:v".to_string(), bitrate.clone()]);
            }

            args.extend([
                "-c:a".to_string(),
                "aac".to_string(),
            ]);

            if let Some(ref audio_bitrate) = cli.audio_bitrate {
                args.extend(["-b:a".to_string(), audio_bitrate.clone()]);
            }
        }

        // handliong metadata
        if !cli.keep_metadata {
            args.push("-map_metadata".to_string());
            args.push("-1".to_string());
        }

        // overwrite flag
        if cli.overwrite {
            args.push("-y".to_string());
        }

        args.push(cli.output.to_str().unwrap().to_string());

        if cli.verbose {
            println!("{} ffmpeg {}", "Running:".bright_blue(), args.join(" "));
        }

        let status = Command::new("ffmpeg")
            .args(&args)
            .stdout(if cli.verbose { Stdio::inherit() } else { Stdio::null() })
            .stderr(if cli.verbose { Stdio::inherit() } else { Stdio::null() })
            .status()
            .context("Failed to execute FFmpeg")?;

        if !status.success() {
            anyhow::bail!("FFmpeg conversion failed");
        }

        Ok(())
    }

    fn convert_audio(&self, cli: &Cli) -> Result<()> {
        let mut args = vec![
            "-i".to_string(),
            cli.input.to_str().unwrap().to_string(),
        ];

        // determining audio codec based on output format
        let codec = match self.output_format.as_str() {
            "mp3" => "libmp3lame",
            "aac" | "m4a" => "aac",
            "ogg" => "libvorbis",
            "opus" => "libopus",
            "flac" => "flac",
            "wav" => "pcm_s16le",
            _ => "copy",
        };

        args.extend([
            "-c:a".to_string(),
            codec.to_string(),
        ]);

        if let Some(ref audio_bitrate) = cli.audio_bitrate {
            args.extend(["-b:a".to_string(), audio_bitrate.clone()]);
        } else if codec != "copy" && codec != "flac" {
            args.extend(["-b:a".to_string(), "192k".to_string()]);
        }

        args.extend(["-vn".to_string()]);

        if !cli.keep_metadata {
            args.push("-map_metadata".to_string());
            args.push("-1".to_string());
        }

        if cli.overwrite {
            args.push("-y".to_string());
        }

        args.push(cli.output.to_str().unwrap().to_string());

        if cli.verbose {
            println!("{} ffmpeg {}", "Running:".bright_blue(), args.join(" "));
        }

        let status = Command::new("ffmpeg")
            .args(&args)
            .stdout(if cli.verbose { Stdio::inherit() } else { Stdio::null() })
            .stderr(if cli.verbose { Stdio::inherit() } else { Stdio::null() })
            .status()
            .context("Failed to execute FFmpeg")?;

        if !status.success() {
            anyhow::bail!("FFmpeg audio conversion failed");
        }

        Ok(())
    }

    fn convert_image(&self, cli: &Cli) -> Result<()> {
        let mut args = vec![
            cli.input.to_str().unwrap().to_string(),
        ];

        // quality settings stuff
        args.extend([
            "-quality".to_string(),
            cli.image_quality.to_string(),
        ]);

        if !cli.keep_metadata {
            args.push("-strip".to_string());
        }

        args.push(cli.output.to_str().unwrap().to_string());

        if cli.verbose {
            println!("{} magick {}", "Running:".bright_blue(), args.join(" "));
        }

        // tries to use magick first (if its available), falls back to convert
        let cmd = which::which("magick")
            .map(|_| "magick")
            .unwrap_or("convert");

        let status = Command::new(cmd)
            .args(&args)
            .stdout(if cli.verbose { Stdio::inherit() } else { Stdio::null() })
            .stderr(if cli.verbose { Stdio::inherit() } else { Stdio::null() })
            .status()
            .context("Failed to execute ImageMagick")?;

        if !status.success() {
            anyhow::bail!("ImageMagick conversion failed");
        }

        Ok(())
    }

    fn convert_document(&self, cli: &Cli) -> Result<()> {
        let args = vec![
            cli.input.to_str().unwrap().to_string(),
            "-o".to_string(),
            cli.output.to_str().unwrap().to_string(),
        ];

        if cli.verbose {
            println!("{} pandoc {}", "Running:".bright_blue(), args.join(" "));
        }

        let status = Command::new("pandoc")
            .args(&args)
            .stdout(if cli.verbose { Stdio::inherit() } else { Stdio::null() })
            .stderr(if cli.verbose { Stdio::inherit() } else { Stdio::null() })
            .status()
            .context("Failed to execute Pandoc")?;

        if !status.success() {
            anyhow::bail!("Pandoc conversion failed");
        }

        Ok(())
    }
}
