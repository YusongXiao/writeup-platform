use anyhow::Result;
use std::io::Write;
use std::path::{Path, PathBuf};

const PDF_MAGIC: &[u8] = b"%PDF";
const ZIP_MAGIC: &[u8] = b"PK\x03\x04";
const CONTENT_TYPES_XML: &str = "[Content_Types].xml";

pub enum FileType {
    Pdf,
    Docx,
}

impl FileType {
    pub fn as_str(&self) -> &'static str {
        match self {
            FileType::Pdf => "pdf",
            FileType::Docx => "docx",
        }
    }

    #[allow(dead_code)]
    pub fn mime(&self) -> &'static str {
        match self {
            FileType::Pdf => "application/pdf",
            FileType::Docx => {
                "application/vnd.openxmlformats-officedocument.wordprocessingml.document"
            }
        }
    }
}

/// Validate file magic bytes and return the detected FileType.
pub fn detect_file_type(data: &[u8]) -> Result<FileType, &'static str> {
    if data.len() < 4 {
        return Err("file too small");
    }
    if data.starts_with(PDF_MAGIC) {
        return Ok(FileType::Pdf);
    }
    if data.starts_with(ZIP_MAGIC) {
        // Verify it's a docx by checking for [Content_Types].xml inside the ZIP
        if is_docx(data) {
            return Ok(FileType::Docx);
        }
        return Err("ZIP file is not a valid docx (missing [Content_Types].xml)");
    }
    Err("unsupported file type: only PDF and docx are accepted")
}

fn is_docx(data: &[u8]) -> bool {
    use std::io::Cursor;
    let cursor = Cursor::new(data);
    let Ok(mut archive) = zip::ZipArchive::new(cursor) else {
        return false;
    };
    archive.by_name(CONTENT_TYPES_XML).is_ok()
}

/// Write bytes to a temp file, then atomically rename to dest.
pub async fn write_file(dest: &Path, data: &[u8]) -> Result<()> {
    let tmp_path = dest.with_extension("tmp");
    if let Some(parent) = dest.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }
    tokio::fs::write(&tmp_path, data).await?;
    tokio::fs::rename(&tmp_path, dest).await?;
    Ok(())
}

/// Delete a file, ignoring not-found errors.
pub async fn delete_file(path: &Path) -> Result<()> {
    match tokio::fs::remove_file(path).await {
        Ok(()) => Ok(()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(e) => Err(e.into()),
    }
}

/// Build the storage path for a writeup file.
#[allow(dead_code)]
pub fn storage_path(root: &str, game_id: i64, user_id: i64, id: &str, ext: &str) -> PathBuf {
    PathBuf::from(root)
        .join(game_id.to_string())
        .join(format!("{}-{}.{}", user_id, id, ext))
}

/// Pack all files for a game into a ZIP archive in memory.
pub async fn zip_game_writeups(
    root: &str,
    writeups: &[database::writeup::Writeup],
) -> Result<Vec<u8>> {
    use std::io::Cursor;
    let buf = Vec::new();
    let cursor = Cursor::new(buf);
    let mut zip = zip::ZipWriter::new(cursor);
    let options = zip::write::SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated);

    for w in writeups {
        let path = PathBuf::from(root).join(&w.file_path);
        let data = match tokio::fs::read(&path).await {
            Ok(d) => d,
            Err(e) => {
                tracing::warn!("skipping missing file {}: {}", path.display(), e);
                continue;
            }
        };
        let safe_name = w.team_name.as_deref().unwrap_or(&w.user_account);
        let safe_slug: String = safe_name.chars().map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' | '\0'..='\x1f' => '_',
            _ => c,
        }).collect();
        let safe_slug = safe_slug.trim_matches('.').trim();
        let safe_slug = if safe_slug.is_empty() { "unknown" } else { safe_slug };
        let entry_name = format!(
            "writeups-{}/{}-{}.{}",
            w.game_id, safe_slug, w.user_id, w.file_type
        );
        zip.start_file(&entry_name, options)?;
        zip.write_all(&data)?;
    }

    let cursor = zip.finish()?;
    Ok(cursor.into_inner())
}
