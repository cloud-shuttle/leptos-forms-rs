//! File data module - File data representation and handling
//!
//! This module provides the FileData struct for representing file uploads,
//! including file metadata, validation, and utility methods.

use serde::{Deserialize, Serialize};
use std::fmt;

/// File data representation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileData {
    pub name: String,
    pub size: usize,
    pub mime_type: String,
    pub data: Vec<u8>, // In practice, this would be a URL or handle
}

impl FileData {
    /// Create a new file data instance
    pub fn new(name: impl Into<String>, mime_type: impl Into<String>, data: Vec<u8>) -> Self {
        let name = name.into();
        let mime_type = mime_type.into();
        let size = data.len();

        Self {
            name,
            size,
            mime_type,
            data,
        }
    }

    /// Get the file name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the file size in bytes
    pub fn size(&self) -> usize {
        self.size
    }

    /// Get the file size in KB
    pub fn size_kb(&self) -> f64 {
        self.size as f64 / 1024.0
    }

    /// Get the file size in MB
    pub fn size_mb(&self) -> f64 {
        self.size as f64 / (1024.0 * 1024.0)
    }

    /// Get the MIME type
    pub fn mime_type(&self) -> &str {
        &self.mime_type
    }

    /// Get the file data
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    /// Get the file extension
    pub fn extension(&self) -> Option<&str> {
        self.name.rfind('.').map(|pos| &self.name[pos + 1..])
    }

    /// Check if the file is an image
    pub fn is_image(&self) -> bool {
        self.mime_type.starts_with("image/")
    }

    /// Check if the file is a video
    pub fn is_video(&self) -> bool {
        self.mime_type.starts_with("video/")
    }

    /// Check if the file is an audio file
    pub fn is_audio(&self) -> bool {
        self.mime_type.starts_with("audio/")
    }

    /// Check if the file is a document
    pub fn is_document(&self) -> bool {
        matches!(
            self.mime_type.as_str(),
            "application/pdf"
                | "application/msword"
                | "application/vnd.openxmlformats-officedocument.wordprocessingml.document"
                | "application/vnd.ms-excel"
                | "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"
                | "application/vnd.ms-powerpoint"
                | "application/vnd.openxmlformats-officedocument.presentationml.presentation"
                | "text/plain"
                | "text/csv"
        )
    }

    /// Check if the file is an archive
    pub fn is_archive(&self) -> bool {
        matches!(
            self.mime_type.as_str(),
            "application/zip"
                | "application/x-rar-compressed"
                | "application/x-7z-compressed"
                | "application/gzip"
                | "application/x-tar"
        )
    }

    /// Get a human-readable file size
    pub fn human_readable_size(&self) -> String {
        let size = self.size;
        if size < 1024 {
            format!("{} B", size)
        } else if size < 1024 * 1024 {
            format!("{:.1} KB", size as f64 / 1024.0)
        } else if size < 1024 * 1024 * 1024 {
            format!("{:.1} MB", size as f64 / (1024.0 * 1024.0))
        } else {
            format!("{:.1} GB", size as f64 / (1024.0 * 1024.0 * 1024.0))
        }
    }

    /// Check if the file is empty
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// Get the file category based on MIME type
    pub fn category(&self) -> FileCategory {
        if self.is_image() {
            FileCategory::Image
        } else if self.is_video() {
            FileCategory::Video
        } else if self.is_audio() {
            FileCategory::Audio
        } else if self.is_document() {
            FileCategory::Document
        } else if self.is_archive() {
            FileCategory::Archive
        } else {
            FileCategory::Other
        }
    }

    /// Validate the file against constraints
    pub fn validate(
        &self,
        max_size: Option<usize>,
        allowed_types: &[String],
    ) -> Result<(), String> {
        // Check file size
        if let Some(max) = max_size {
            if self.size > max {
                return Err(format!(
                    "File size {} exceeds maximum allowed size {}",
                    self.human_readable_size(),
                    Self::format_size(max)
                ));
            }
        }

        // Check file type
        if !allowed_types.is_empty() {
            let is_allowed = allowed_types.iter().any(|allowed_type| {
                if allowed_type.ends_with("/*") {
                    let prefix = &allowed_type[..allowed_type.len() - 2];
                    self.mime_type.starts_with(prefix)
                } else {
                    self.mime_type == *allowed_type
                }
            });

            if !is_allowed {
                return Err(format!(
                    "File type {} is not allowed. Allowed types: {}",
                    self.mime_type,
                    allowed_types.join(", ")
                ));
            }
        }

        Ok(())
    }

    /// Format a size in bytes to human-readable format
    fn format_size(size: usize) -> String {
        if size < 1024 {
            format!("{} B", size)
        } else if size < 1024 * 1024 {
            format!("{:.1} KB", size as f64 / 1024.0)
        } else if size < 1024 * 1024 * 1024 {
            format!("{:.1} MB", size as f64 / (1024.0 * 1024.0))
        } else {
            format!("{:.1} GB", size as f64 / (1024.0 * 1024.0 * 1024.0))
        }
    }
}

impl fmt::Display for FileData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.human_readable_size())
    }
}

/// File categories for classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FileCategory {
    Image,
    Video,
    Audio,
    Document,
    Archive,
    Other,
}

impl FileCategory {
    /// Get the display name for the category
    pub fn display_name(&self) -> &'static str {
        match self {
            FileCategory::Image => "Image",
            FileCategory::Video => "Video",
            FileCategory::Audio => "Audio",
            FileCategory::Document => "Document",
            FileCategory::Archive => "Archive",
            FileCategory::Other => "Other",
        }
    }

    /// Get the icon name for the category
    pub fn icon_name(&self) -> &'static str {
        match self {
            FileCategory::Image => "image",
            FileCategory::Video => "video",
            FileCategory::Audio => "audio",
            FileCategory::Document => "document",
            FileCategory::Archive => "archive",
            FileCategory::Other => "file",
        }
    }
}

impl std::fmt::Display for FileCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}
