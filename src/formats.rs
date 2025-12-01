const VIDEO_FORMATS: &[&str] = &[
    "mp4", "avi", "mkv", "mov", "wmv", "flv", "webm", "m4v", "mpg", "mpeg",
    "3gp", "3g2", "mts", "m2ts", "ts", "vob", "ogv", "gifv", "mxf", "roq",
    "nsv", "f4v", "f4p", "f4a", "f4b",
];

const AUDIO_FORMATS: &[&str] = &[
    "mp3", "wav", "flac", "aac", "ogg", "opus", "m4a", "wma", "aiff", "ape",
    "alac", "wv", "tta", "ac3", "dts", "amr", "au", "ra", "rm", "mka",
    "mp2", "mpa", "spx", "oga", "mogg",
];

const IMAGE_FORMATS: &[&str] = &[
    "jpg", "jpeg", "png", "gif", "bmp", "webp", "svg", "tiff", "tif", "ico",
    "raw", "cr2", "nef", "arw", "dng", "orf", "rw2", "pef", "srw", "raf",
    "heic", "heif", "avif", "jxl", "jp2", "j2k", "jpf", "jpx", "jpm",
    "psd", "xcf", "exr", "hdr", "dds", "tga", "pcx", "ppm", "pgm", "pbm",
    "pnm", "sgi", "pic", "pct", "pict", "bpg", "flif", "jfif",
];

const DOCUMENT_FORMATS: &[&str] = &[
    "txt", "md", "markdown", "rst", "tex", "rtf", "org",
    "doc", "docx", "odt", "pdf", "xls", "xlsx", "ods", "ppt", "pptx", "odp",
    "epub", "mobi", "azw3", "fb2",
    "html", "htm", "xhtml", "xml", "json", "yaml", "yml", "toml",
    "csv", "tsv", "latex", "adoc", "asciidoc", "docbook",
];

pub fn is_video_format(ext: &str) -> bool {
    VIDEO_FORMATS.contains(&ext.to_lowercase().as_str())
}

pub fn is_audio_format(ext: &str) -> bool {
    AUDIO_FORMATS.contains(&ext.to_lowercase().as_str())
}

pub fn is_image_format(ext: &str) -> bool {
    IMAGE_FORMATS.contains(&ext.to_lowercase().as_str())
}

pub fn is_document_format(ext: &str) -> bool {
    DOCUMENT_FORMATS.contains(&ext.to_lowercase().as_str())
}

// ignore for now, going to be used later on..
/// i pray that it actually gets used
pub fn get_all_formats() -> Vec<String> {
    let mut formats = Vec::new();
    formats.extend(VIDEO_FORMATS.iter().map(|s| s.to_string()));
    formats.extend(AUDIO_FORMATS.iter().map(|s| s.to_string()));
    formats.extend(IMAGE_FORMATS.iter().map(|s| s.to_string()));
    formats.extend(DOCUMENT_FORMATS.iter().map(|s| s.to_string()));
    formats.sort();
    formats.dedup();
    formats
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_video_formats() {
        assert!(is_video_format("mp4"));
        assert!(is_video_format("MP4"));
        assert!(is_video_format("mkv"));
        assert!(!is_video_format("mp3"));
    }

    #[test]
    fn test_audio_formats() {
        assert!(is_audio_format("mp3"));
        assert!(is_audio_format("wav"));
        assert!(!is_audio_format("mp4"));
    }

    #[test]
    fn test_image_formats() {
        assert!(is_image_format("jpg"));
        assert!(is_image_format("PNG"));
        assert!(!is_image_format("mp4"));
    }

    #[test]
    fn test_document_formats() {
        assert!(is_document_format("pdf"));
        assert!(is_document_format("docx"));
        assert!(!is_document_format("mp4"));
    }
}
