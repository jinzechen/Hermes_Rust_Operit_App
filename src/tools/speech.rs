//! Speech tool — placeholder for speech-to-text and text-to-speech.
//!
//! Both actions return placeholder messages.  Real speech processing
//! requires external models (Whisper, etc.) or cloud APIs (Azure, Google).

use anyhow::bail;

use crate::core::tool_registry::{ToolHandler, ToolSchema};

pub struct SpeechTool;

impl SpeechTool {
    pub fn new() -> Self {
        Self
    }
}

impl Default for SpeechTool {
    fn default() -> Self {
        Self::new()
    }
}

impl ToolHandler for SpeechTool {
    fn schema(&self) -> ToolSchema {
        ToolSchema {
            name: "speech".into(),
            description:
                "Speech-to-text and text-to-speech (placeholder — requires external model/API)"
                    .into(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "action": {
                        "type": "string",
                        "enum": ["speech_to_text", "text_to_speech"]
                    },
                    "path": {
                        "type": "string",
                        "description": "Path to audio file (for speech_to_text) or output path (for text_to_speech)"
                    },
                    "text": {
                        "type": "string",
                        "description": "Text to convert to speech (for text_to_speech)"
                    },
                    "language": {
                        "type": "string",
                        "description": "Language code, e.g. 'en', 'zh' (optional)"
                    }
                },
                "required": ["action"]
            }),
        }
    }

    fn execute(&self, arguments: serde_json::Value) -> anyhow::Result<String> {
        let action = arguments
            .get("action")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        match action {
            "speech_to_text" => speech_to_text_placeholder(&arguments),
            "text_to_speech" => text_to_speech_placeholder(&arguments),
            _ => bail!("unknown speech action: '{}'", action),
        }
    }
}

fn speech_to_text_placeholder(args: &serde_json::Value) -> anyhow::Result<String> {
    let path = args
        .get("path")
        .and_then(|v| v.as_str())
        .unwrap_or("(no path)");
    let language = args
        .get("language")
        .and_then(|v| v.as_str())
        .unwrap_or("auto");

    Ok(format!(
        "## Speech-to-Text (placeholder)\n\n\
         Audio file: {}\n\
         Language: {}\n\n\
         > Speech-to-text transcription requires an external model or cloud API:\n\
         > - Local: OpenAI Whisper (whisper-rs), Vosk, Coqui STT\n\
         > - Cloud: Azure Speech, Google Cloud Speech-to-Text, Deepgram\n\
         >\n\
         > This is a placeholder.  Integrate one of the above for real transcription.\n\
         > Until then, use the `vision` tool for image-based OCR if applicable.",
        path, language
    ))
}

fn text_to_speech_placeholder(args: &serde_json::Value) -> anyhow::Result<String> {
    let text = args.get("text").and_then(|v| v.as_str()).unwrap_or("");
    let path = args
        .get("path")
        .and_then(|v| v.as_str())
        .unwrap_or("(no output path)");
    let language = args
        .get("language")
        .and_then(|v| v.as_str())
        .unwrap_or("en");

    let preview: String = text.chars().take(200).collect();
    let suffix = if text.len() > 200 { "…" } else { "" };

    Ok(format!(
        "## Text-to-Speech (placeholder)\n\n\
         Text: \"{}{}\" ({} chars)\n\
         Output: {}\n\
         Language: {}\n\n\
         > Text-to-speech synthesis requires an external model or cloud API:\n\
         > - Local: Piper TTS, eSpeak, Coqui TTS\n\
         > - Cloud: Azure Speech, Google Cloud Text-to-Speech, ElevenLabs\n\
         >\n\
         > This is a placeholder.  Integrate one of the above for real speech synthesis.",
        preview,
        suffix,
        text.len(),
        path,
        language
    ))
}
