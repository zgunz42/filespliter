use anyhow::{Context, Result};
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};

const BUFFER_SIZE: usize = 8 * 1024 * 1024; // 8MB buffer for fast I/O

#[allow(dead_code)]
pub struct ProgressInfo {
    pub current_bytes: u64,
    pub total_bytes: u64,
    pub current_part: usize,
    pub total_parts: usize,
    pub message: String,
}

impl ProgressInfo {
    pub fn percentage(&self) -> f32 {
        if self.total_bytes == 0 {
            0.0
        } else {
            (self.current_bytes as f32 / self.total_bytes as f32) * 100.0
        }
    }
}

pub fn split_file<F>(
    input_path: &Path,
    part_size: u64,
    mut progress_callback: F,
) -> Result<Vec<PathBuf>>
where
    F: FnMut(ProgressInfo),
{
    if !input_path.exists() {
        anyhow::bail!("Input file does not exist: {:?}", input_path);
    }

    if part_size == 0 {
        anyhow::bail!("Part size must be greater than 0");
    }

    let file = File::open(input_path).context("Failed to open input file")?;

    let file_size = file
        .metadata()
        .context("Failed to get file metadata")?
        .len();

    let total_parts: usize = file_size.div_ceil(part_size) as usize;

    let mut reader = BufReader::with_capacity(BUFFER_SIZE, file);
    let mut part_paths = Vec::new();
    let mut part_number: u32 = 1;
    let mut total_written = 0u64;

    while total_written < file_size {
        let part_path = get_part_path(input_path, part_number);
        let part_file = File::create(&part_path)
            .context(format!("Failed to create part file: {:?}", part_path))?;

        let mut writer = BufWriter::with_capacity(BUFFER_SIZE, part_file);
        let mut bytes_written_in_part = 0u64;
        let mut buffer = vec![0u8; BUFFER_SIZE];

        progress_callback(ProgressInfo {
            current_bytes: total_written,
            total_bytes: file_size,
            current_part: part_number as usize,
            total_parts,
            message: format!("Splitting part {}/{}", part_number, total_parts),
        });

        while bytes_written_in_part < part_size && total_written < file_size {
            let remaining_in_part = part_size - bytes_written_in_part;
            let to_read = (BUFFER_SIZE as u64).min(remaining_in_part) as usize;

            let bytes_read = reader
                .read(&mut buffer[..to_read])
                .context("Failed to read from input file")?;

            if bytes_read == 0 {
                break;
            }

            writer
                .write_all(&buffer[..bytes_read])
                .context("Failed to write to part file")?;

            bytes_written_in_part += bytes_read as u64;
            total_written += bytes_read as u64;

            progress_callback(ProgressInfo {
                current_bytes: total_written,
                total_bytes: file_size,
                current_part: part_number as usize,
                total_parts,
                message: format!("Splitting part {}/{}", part_number, total_parts),
            });
        }

        writer.flush().context("Failed to flush part file")?;

        part_paths.push(part_path);
        part_number += 1;
    }

    progress_callback(ProgressInfo {
        current_bytes: file_size,
        total_bytes: file_size,
        current_part: total_parts,
        total_parts,
        message: "Split complete!".to_string(),
    });

    Ok(part_paths)
}

pub fn join_files<F>(
    first_part: &Path,
    output_path: &Path,
    mut progress_callback: F,
) -> Result<PathBuf>
where
    F: FnMut(ProgressInfo),
{
    if !first_part.exists() {
        anyhow::bail!("First part file does not exist: {:?}", first_part);
    }

    let part_files = find_all_parts(first_part)?;

    if part_files.is_empty() {
        anyhow::bail!("No part files found");
    }

    let total_size: u64 = part_files
        .iter()
        .map(|p| std::fs::metadata(p).map(|m| m.len()).unwrap_or(0))
        .sum();

    let output_file = File::create(output_path).context("Failed to create output file")?;

    let mut writer = BufWriter::with_capacity(BUFFER_SIZE, output_file);
    let mut total_bytes = 0u64;
    let total_parts = part_files.len();

    for (index, part_path) in part_files.iter().enumerate() {
        progress_callback(ProgressInfo {
            current_bytes: total_bytes,
            total_bytes: total_size,
            current_part: index + 1,
            total_parts,
            message: format!("Joining part {}/{}", index + 1, total_parts),
        });

        let part_file =
            File::open(part_path).context(format!("Failed to open part file: {:?}", part_path))?;

        let mut reader = BufReader::with_capacity(BUFFER_SIZE, part_file);
        let mut buffer = vec![0u8; BUFFER_SIZE];

        loop {
            let bytes_read = reader
                .read(&mut buffer)
                .context("Failed to read from part file")?;

            if bytes_read == 0 {
                break;
            }

            writer
                .write_all(&buffer[..bytes_read])
                .context("Failed to write to output file")?;

            total_bytes += bytes_read as u64;

            progress_callback(ProgressInfo {
                current_bytes: total_bytes,
                total_bytes: total_size,
                current_part: index + 1,
                total_parts,
                message: format!("Joining part {}/{}", index + 1, total_parts),
            });
        }
    }

    writer.flush().context("Failed to flush output file")?;

    progress_callback(ProgressInfo {
        current_bytes: total_size,
        total_bytes: total_size,
        current_part: total_parts,
        total_parts,
        message: "Join complete!".to_string(),
    });

    Ok(output_path.to_path_buf())
}

fn get_part_path(input_path: &Path, part_number: u32) -> PathBuf {
    let file_name = input_path
        .file_name()
        .unwrap_or_else(|| std::ffi::OsStr::new("file"));

    let part_name = format!("{}.part{:03}", file_name.to_string_lossy(), part_number);

    input_path.with_file_name(part_name)
}

pub fn find_all_parts(first_part: &Path) -> Result<Vec<PathBuf>> {
    let file_name = first_part
        .file_name()
        .context("Invalid file name")?
        .to_string_lossy();

    let base_name = if let Some(pos) = file_name.rfind(".part") {
        &file_name[..pos]
    } else {
        anyhow::bail!("First part file must have .partXXX extension");
    };

    let parent_dir = first_part.parent().unwrap_or_else(|| Path::new("."));

    let mut parts = Vec::new();
    let mut part_number = 1;

    loop {
        let part_name = format!("{}.part{:03}", base_name, part_number);
        let part_path = parent_dir.join(&part_name);

        if !part_path.exists() {
            break;
        }

        parts.push(part_path);
        part_number += 1;
    }

    if parts.is_empty() {
        anyhow::bail!(
            "No sequential part files found starting from: {:?}",
            first_part
        );
    }

    Ok(parts)
}

pub fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    format!("{:.2} {}", size, UNITS[unit_index])
}
