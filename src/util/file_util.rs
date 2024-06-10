use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

pub fn read_file(path: &Path) -> anyhow::Result<String> {
    Ok(fs::read_to_string(path)?)
}

#[allow(dead_code)]
pub fn create_file_if_not_exist<P: AsRef<Path>>(path: P) -> io::Result<()> {
    let path = path.as_ref();
    let parent = path.parent().unwrap();
    if !parent.exists() {
        fs::create_dir_all(&parent)?;
    }
    if !path.exists() {
        File::create(path)?;
    }
    Ok(())
}

#[allow(dead_code)]
pub fn mkdir<P: AsRef<Path>>(path: P) -> io::Result<()> {
    fs::create_dir_all(path)
}

#[allow(dead_code)]
pub fn write_file<P: AsRef<Path>>(path: P, text: &str) {
    let mut file = File::create(path).unwrap_or_else(|e| {
        panic!("Could not create file: {:?}", e);
    });
    file.write_all(text.as_bytes()).unwrap_or_else(|e| {
        panic!("Write file: {:?}", e);
    });
}

#[allow(dead_code)]
pub fn delete_dir<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
    fs::remove_dir_all(path)?;
    Ok(())
}

#[allow(dead_code)]
pub fn rename<P: AsRef<Path>>(src: P, dst: P) -> anyhow::Result<()> {
    fs::rename(src, dst)?;
    Ok(())
}

pub fn get_current_dir() -> PathBuf {
    std::env::current_dir().unwrap_or_else(|e| {
        panic!("Failed to get current path: {:?}", e);
    })
}

#[allow(dead_code)]
pub fn get_exe_dir() -> PathBuf {
    std::env::current_exe().unwrap_or_else(|e| {
        panic!("Failed to get exe path: {:?}", e);
    }).parent().unwrap().to_path_buf()
}
