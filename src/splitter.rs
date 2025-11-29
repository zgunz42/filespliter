use anyhow::{Context, Result};
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};

const BUFFER_SIZE: usize = 8 * 1024 * 1024; // 8MB buffer for fast I/O

pub struct FileSplitter {
    input_path: PathBuf,
    part_size: u64,
}

impl FileSplitter {
    pub fn new<P: AsRef<Path>>(input_path: P, part_size: u64) -> Result<Self> {
        let input_path = input_path.as_ref().to_path_buf();

        if !input_path.exists() {
            anyhow::bail!("Input file does not exist: {:?}", input_path);
        }

        if part_size == 0 {
            anyhow::bail!("Part size must be greater than 0");
        }

        Ok(Self {
            input_path,
            part_size,
        })
    }

    pub fn split(&self) -> Result<Vec<PathBuf>> {
        let file = File::open(&self.input_path)
            .context("Failed to open input file")?;

        let file_size = file.metadata()
            .context("Failed to get file metadata")?
            .len();

        println!("\n{}", "═══════════════════════════════════════".bright_cyan());
        println!("{}", "          FILE SPLITTER".bright_cyan().bold());
        println!("{}", "═══════════════════════════════════════".bright_cyan());

        println!("\n{} {:?}", "Input file:".green().bold(), self.input_path);
        println!("{} {}", "File size:".green().bold(), format_bytes(file_size).yellow());
        println!("{} {}", "Part size:".green().bold(), format_bytes(self.part_size).yellow());

        let num_parts = (file_size + self.part_size - 1) / self.part_size;
        println!("{} {}\n", "Total parts:".green().bold(), num_parts.to_string().cyan());

        let pb = ProgressBar::new(file_size);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:50.cyan/blue}] {bytes}/{total_bytes} ({percent}%) {msg} ETA: {eta}")
                .unwrap()
                .progress_chars("█▓▒░ "),
        );

        let mut reader = BufReader::with_capacity(BUFFER_SIZE, file);
        let mut part_paths = Vec::new();
        let mut part_number = 1;
        let mut total_written = 0u64;

        while total_written < file_size {
            let part_path = self.get_part_path(part_number);
            let part_file = File::create(&part_path)
                .context(format!("Failed to create part file: {:?}", part_path))?;

            let mut writer = BufWriter::with_capacity(BUFFER_SIZE, part_file);
            let mut bytes_written_in_part = 0u64;
            let mut buffer = vec![0u8; BUFFER_SIZE];

            pb.set_message(format!("Splitting part {}/{}", part_number, num_parts));

            while bytes_written_in_part < self.part_size && total_written < file_size {
                let remaining_in_part = self.part_size - bytes_written_in_part;
                let to_read = (BUFFER_SIZE as u64).min(remaining_in_part) as usize;

                let bytes_read = reader.read(&mut buffer[..to_read])
                    .context("Failed to read from input file")?;

                if bytes_read == 0 {
                    break;
                }

                writer.write_all(&buffer[..bytes_read])
                    .context("Failed to write to part file")?;

                bytes_written_in_part += bytes_read as u64;
                total_written += bytes_read as u64;
                pb.set_position(total_written);
            }

            writer.flush()
                .context("Failed to flush part file")?;

            part_paths.push(part_path);
            part_number += 1;
        }

        pb.finish_with_message("Split complete!".green().to_string());

        println!("\n{}", "═══════════════════════════════════════".bright_cyan());
        println!("{} {}", "✓ Successfully created".green().bold(), format!("{} parts", part_paths.len()).cyan().bold());
        println!("{}", "═══════════════════════════════════════".bright_cyan());

        println!("\n{}", "Part files:".yellow().bold());
        for (i, part) in part_paths.iter().enumerate() {
            println!("  {} {:?}", format!("[{}]", i + 1).cyan(), part);
        }
        println!();

        Ok(part_paths)
    }

    fn get_part_path(&self, part_number: u32) -> PathBuf {
        let file_name = self.input_path.file_name()
            .unwrap_or_else(|| std::ffi::OsStr::new("file"));

        let part_name = format!("{}.part{:03}",
                                file_name.to_string_lossy(),
                                part_number);

        self.input_path.with_file_name(part_name)
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
