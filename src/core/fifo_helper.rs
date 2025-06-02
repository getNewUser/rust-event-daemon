use nix::sys::stat::Mode;
use nix::unistd::mkfifo;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::io::{self, BufRead};
use std::os::unix::fs::FileTypeExt;
use std::path::Path;

pub fn ensure_fifo_exists(path: &str) {
    let system_path = Path::new(path);
    if system_path.exists() {
        match fs::metadata(system_path) {
            Ok(metadata) => {
                if !metadata.file_type().is_fifo() {
                    eprintln!(
                        "⚠️  File at '{}' exists but is not a FIFO. Replacing it...",
                        path
                    );
                    if let Err(e) = fs::remove_file(system_path) {
                        eprintln!("🚫 Failed to remove non-FIFO file '{}': {}", path, e);
                        return;
                    }
                    if let Err(e) = mkfifo(path, Mode::S_IRWXU) {
                        eprintln!("🚫 Failed to create FIFO at '{}': {}", path, e);
                    } else {
                        eprintln!("✅ Created FIFO at '{}'", path);
                    }
                }
            }
            Err(e) => {
                eprintln!("🚫 Failed to read metadata for '{}': {}", path, e);
            }
        }
    } else {
        match mkfifo(path, Mode::S_IRWXU) {
            Ok(_) => eprintln!("✅ Created FIFO at '{}'", path),
            Err(e) => eprintln!("🚫 Failed to create FIFO at '{}': {}", path, e),
        }
    }
}

pub fn read_lines(path: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    ensure_fifo_exists(path);
    let file = File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn write(text: &str, fifo: FifoFile) {
    ensure_fifo_exists(fifo.file_path());

    match OpenOptions::new().write(true).open(fifo.file_path()) {
        Ok(mut fifo_file) => {
            if let Err(e) = writeln!(fifo_file, "{}", text) {
                eprintln!("🚫 Failed to write to FIFO '{}': {}", fifo.file_path(), e);
            } else {
                eprintln!("✅ Wrote to FIFO '{}': {}", fifo.file_path(), text);
            }
        }
        Err(e) => {
            eprintln!(
                "🚫 Failed to open FIFO for writing '{}': {}",
                fifo.file_path(),
                e
            );
        }
    }
}

#[derive(Debug)]
pub enum FifoFile {
    VolumeStatus,
}

impl FifoFile {
    pub fn file_path(&self) -> &'static str {
        match self {
            Self::VolumeStatus => "/tmp/volume-status.fifo",
        }
    }
}
