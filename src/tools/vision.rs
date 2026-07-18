//! Vision tool — basic image file analysis (metadata, format detection).
//!
//! Extracts file metadata (size, format, dimensions) from common image
//! formats.  Full AI vision analysis (object detection, scene description)
//! would require an external model integration.

use anyhow::{bail, Context};
use std::fs;
use std::path::Path;

use crate::core::tool_registry::{ToolHandler, ToolSchema};

pub struct VisionTool;

impl VisionTool {
    pub fn new() -> Self {
        Self
    }
}

impl Default for VisionTool {
    fn default() -> Self {
        Self::new()
    }
}

impl ToolHandler for VisionTool {
    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: "vision".into(),
            description: "Analyze images — file metadata, format detection, dimension extraction"
                .into(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "action": {
                        "type": "string",
                        "enum": ["analyze_image", "describe_scene"]
                    },
                    "path": {
                        "type": "string",
                        "description": "Path to image file"
                    },
                    "prompt": {
                        "type": "string",
                        "description": "Optional prompt to guide analysis"
                    }
                },
                "required": ["action", "path"]
            }),
        }
    }

    fn execute(&self, arguments: serde_json::Value) -> anyhow::Result<String> {
        let action = arguments
            .get("action")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let path_str = arguments
            .get("path")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let prompt = arguments
            .get("prompt")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        match action {
            "analyze_image" => analyze_image(path_str, prompt),
            "describe_scene" => describe_scene(path_str, prompt),
            _ => bail!("unknown vision action: '{}'", action),
        }
    }
}

// ── helpers ───────────────────────────────────────────────────────

/// Known image format with its typical extensions.
#[derive(Debug)]
struct ImageInfo {
    format: &'static str,
    mime: &'static str,
    width: Option<u32>,
    height: Option<u32>,
    file_size: u64,
}

/// Detect image format from magic bytes and extract dimensions for
/// common formats (PNG, JPEG, GIF, BMP).
fn read_image_info(path_str: &str) -> anyhow::Result<ImageInfo> {
    let p = Path::new(path_str);
    let meta = fs::metadata(p)
        .with_context(|| format!("cannot access file: {}", path_str))?;
    let file_size = meta.len();

    if file_size < 8 {
        bail!("file too small to be a valid image ({} bytes)", file_size);
    }

    let data = fs::read(p)
        .with_context(|| format!("cannot read file: {}", path_str))?;

    // ── PNG ────────────────────────────────────────────────
    if data.len() >= 24
        && data[0] == 0x89
        && data[1] == b'P'
        && data[2] == b'N'
        && data[3] == b'G'
    {
        // PNG: width at offset 16 (4 bytes big-endian), height at offset 20
        let w = u32::from_be_bytes([data[16], data[17], data[18], data[19]]);
        let h = u32::from_be_bytes([data[20], data[21], data[22], data[23]]);
        return Ok(ImageInfo {
            format: "PNG",
            mime: "image/png",
            width: Some(w),
            height: Some(h),
            file_size,
        });
    }

    // ── JPEG ───────────────────────────────────────────────
    if data[0] == 0xFF && data[1] == 0xD8 && data[2] == 0xFF {
        // Scan JPEG markers for SOF0 (0xC0) / SOF2 (0xC2) to get dimensions.
        let (w, h) = jpeg_dimensions(&data);
        return Ok(ImageInfo {
            format: "JPEG",
            mime: "image/jpeg",
            width: w,
            height: h,
            file_size,
        });
    }

    // ── GIF ────────────────────────────────────────────────
    if data.len() >= 10
        && (data[0] == b'G' && data[1] == b'I' && data[2] == b'F' && data[3] == b'8'
            && (data[4] == b'7' || data[4] == b'9') && data[5] == b'a')
    {
        let w = u16::from_le_bytes([data[6], data[7]]) as u32;
        let h = u16::from_le_bytes([data[8], data[9]]) as u32;
        return Ok(ImageInfo {
            format: "GIF",
            mime: "image/gif",
            width: Some(w),
            height: Some(h),
            file_size,
        });
    }

    // ── BMP ────────────────────────────────────────────────
    if data[0] == b'B' && data[1] == b'M' && data.len() >= 26 {
        let w = u32::from_le_bytes([data[18], data[19], data[20], data[21]]);
        let h = u32::from_le_bytes([data[22], data[23], data[24], data[25]]);
        return Ok(ImageInfo {
            format: "BMP",
            mime: "image/bmp",
            width: Some(w),
            height: Some(h),
            file_size,
        });
    }

    // ── WebP ───────────────────────────────────────────────
    if data.len() >= 12
        && data[0] == b'R' && data[1] == b'I' && data[2] == b'F' && data[3] == b'F'
        && data[8] == b'W' && data[9] == b'E' && data[10] == b'B' && data[11] == b'P'
    {
        let w = u16::from_le_bytes([data[24], data[25]]) as u32;
        let h = u16::from_le_bytes([data[26], data[27]]) as u32;
        return Ok(ImageInfo {
            format: "WebP (RIFF)",
            mime: "image/webp",
            width: Some(w),
            height: Some(h),
            file_size,
        });
    }

    // ── TIFF ───────────────────────────────────────────────
    if (data[0] == b'I' && data[1] == b'I' && data[2] == 0x2A && data[3] == 0x00)
        || (data[0] == b'M' && data[1] == b'M' && data[2] == 0x00 && data[3] == 0x2A)
    {
        return Ok(ImageInfo {
            format: "TIFF",
            mime: "image/tiff",
            width: None,
            height: None,
            file_size,
        });
    }

    Ok(ImageInfo {
        format: "unknown",
        mime: "application/octet-stream",
        width: None,
        height: None,
        file_size,
    })
}

/// Walk JPEG marker segments to find dimensions in SOF0/SOF2.
fn jpeg_dimensions(data: &[u8]) -> (Option<u32>, Option<u32>) {
    let mut pos = 2usize;
    while pos + 4 <= data.len() {
        if data[pos] != 0xFF {
            break;
        }
        let marker = data[pos + 1];
        pos += 2;
        if marker == 0xD8 || marker == 0xD9 {
            continue; // SOI / EOI — no length field
        }
        if pos + 2 > data.len() {
            break;
        }
        let len = u16::from_be_bytes([data[pos], data[pos + 1]]) as usize;
        if len < 2 || pos + len > data.len() {
            break;
        }
        if (marker == 0xC0 || marker == 0xC2) && len >= 7 {
            let h = u16::from_be_bytes([data[pos + 3], data[pos + 4]]) as u32;
            let w = u16::from_be_bytes([data[pos + 5], data[pos + 6]]) as u32;
            return (Some(w), Some(h));
        }
        pos += len;
    }
    (None, None)
}

/// Format bytes as human-readable size.
fn fmt_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
    let mut s = bytes as f64;
    let mut idx = 0;
    while s >= 1024.0 && idx + 1 < UNITS.len() {
        s /= 1024.0;
        idx += 1;
    }
    format!("{:.1} {}", s, UNITS[idx])
}

// ── action functions ──────────────────────────────────────────────

fn analyze_image(path_str: &str, prompt: &str) -> anyhow::Result<String> {
    let info = read_image_info(path_str)?;

    let dims = match (info.width, info.height) {
        (Some(w), Some(h)) => format!("{} × {} px", w, h),
        _ => "unknown".into(),
    };

    let mut out = format!(
        "## Image Analysis\n\n\
         File: {}\n\
         Format: {}\n\
         MIME: {}\n\
         Size: {} ({} bytes)\n\
         Dimensions: {}",
        path_str, info.format, info.mime, fmt_size(info.file_size), info.file_size, dims
    );

    if info.format == "unknown" {
        out.push_str("\n\n> ⚠ Format not recognized from magic bytes. First 32 hex bytes:\n> ");
        let data = fs::read(path_str)?;
        let hex: String = data.iter().take(32).map(|b| format!("{:02X}", b)).collect::<Vec<_>>().join(" ");
        out.push_str(&hex);
    }

    if !prompt.is_empty() {
        out.push_str(&format!(
            "\n\n> Prompt: {}\n> (full AI vision analysis requires external model integration)",
            prompt
        ));
    }

    Ok(out)
}

fn describe_scene(path_str: &str, prompt: &str) -> anyhow::Result<String> {
    let info = read_image_info(path_str)?;

    let dims = match (info.width, info.height) {
        (Some(w), Some(h)) => format!("{} × {} px", w, h),
        _ => "unknown dimensions".into(),
    };

    let mut out = format!(
        "## Scene Description\n\n\
         File: {}\n\
         Format: {} ({})\n\
         Size: {}\n\
         Dimensions: {}\n\n\
         > Scene description requires an AI vision model (e.g., CLIP, LLaVA, GPT-4o).\n\
         > Currently only file-level metadata is available.\n\
         > This is a placeholder — full vision integration planned for a future phase.",
        path_str, info.format, info.mime, fmt_size(info.file_size), dims
    );

    if !prompt.is_empty() {
        out.push_str(&format!("\n> Prompt: {}", prompt));
    }

    Ok(out)
}
