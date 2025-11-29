use anyhow::{Context, Result};
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};

const BUFFER_SIZE: usize = 8 * 1024 * 1024; // 8MB buffer for fast I/O

pub struct FileJoiner {
    part_files: Vec<PathBuf>,
    output_path: PathBuf,
}

impl FileJoiner {
    pub fn new<P: AsRef<Path>>(first_part: P, output_path: P) -> Result<Self> {
        let first_part = first_part.as_ref().to_path_buf();
        let output_path = output_path.as_ref().to_path_buf();

        if !first_part.exists() {
            anyhow::bail!("First part file does not exist: {:?}", first_part);
        }

        let part_files = Self::find_all_parts(&first_part)?;

        if part_files.is_empty() {
            anyhow::bail!("No part files found");
        }

        Ok(Self {
            part_files,
            output_path,
        })
    }

    pub fn join(&self) -> Result<PathBuf> {
        let total_size: u64 = self.part_files.iter()
            .map(|p| std::fs::metadata(p).map(|m| m.len()).unwrap_or(0))
            .sum();

        println!("\n{}", "═══════════════════════════════════════".bright_magenta());
        println!("{}", "           FILE JOINER".bright_magenta().bold());
        println!("{}", "═══════════════════════════════════════".bright_magenta());

        println!("\n{} {}", "Number of parts:".green().bold(), self.part_files.len().to_string().cyan());
        println!("{} {}", "Total size:".green().bold(), format_bytes(total_size).yellow());
        println!("{} {:?}\n", "Output file:".green().bold(), self.output_path);

        let pb = ProgressBar::new(total_size);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:50.magenta/blue}] {bytes}/{total_bytes} ({percent}%) {msg} ETA: {eta}")
                .unwrap()
                .progress_chars("█▓▒░ "),
        );

        let output_file = File::create(&self.output_path)
            .context("Failed to create output file")?;

        let mut writer = BufWriter::with_capacity(BUFFER_SIZE, output_file);
        let mut total_bytes = 0u64;

        for (index, part_path) in self.part_files.iter().enumerate() {
            pb.set_message(format!("Joining part {}/{}", index + 1, self.part_files.len()));

            let part_file = File::open(part_path)
                .context(format!("Failed to open part file: {:?}", part_path))?;

            let mut reader = BufReader::with_capacity(BUFFER_SIZE, part_file);
            let mut buffer = vec![0u8; BUFFER_SIZE];

            loop {
                let bytes_read = reader.read(&mut buffer)
                    .context("Failed to read from part file")?;

                if bytes_read == 0 {
                    break;
                }

                writer.write_all(&buffer[..bytes_read])
                    .context("Failed to write to output file")?;

                total_bytes += bytes_read as u64;
                pb.set_position(total_bytes);
            }
        }

        writer.flush()
            .context("Failed to flush output file")?;

        pb.finish_with_message("Join complete!".green().to_string());

        println!("\n{}", "═══════════════════════════════════════".bright_magenta());
        println!("{} {}", "✓ Successfully joined".green().bold(), format_bytes(total_bytes).cyan().bold());
        println!("{}", "═══════════════════════════════════════".bright_magenta());

        println!("\n{} {:?}\n", "Output file:".yellow().bold(), self.output_path);

        Ok(self.output_path.clone())
    }

    fn find_all_parts(first_part: &Path) -> Result<Vec<PathBuf>> {
        let file_name = first_part.file_name()
            .context("Invalid file name")?
            .to_string_lossy();

        // Extract base name (remove .partXXX)
        let base_name = if let Some(pos) = file_name.rfind(".part") {
            &file_name[..pos]
        } else {
            anyhow::bail!("First part file must have .partXXX extension");
        };

        let parent_dir = first_part.parent()
            .unwrap_or_else(|| Path::new("."));

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
            anyhow::bail!("No sequential part files found starting from: {:?}", first_part);
        }

        Ok(parts)
    }
}

fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    format!("{:.2} {}", size, UNITS[unit_index])
}
