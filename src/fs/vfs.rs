use crate::filesystem::{FileSystem, DirectoryEntry, FileSystemError};

#[derive(Debug)]
pub struct Vfs {
    file_systems: Vec<Box<dyn FileSystem>>,
}

impl Vfs {
    pub fn new() -> Vfs {
        Vfs {
            file_systems: Vec::new(),
        }
    }

    pub fn mount_filesystem(&mut self, filesystem: Box<dyn FileSystem>) {
        self.file_systems.push(filesystem);
    }

    pub fn read_directory(&mut self, path: &str) -> Result<Vec<DirectoryEntry>, FileSystemError> {
        for fs in &mut self.file_systems {
            match fs.read_directory(path) {
                Ok(entries) => {
                    return Ok(entries);
                },
                Err(_) => continue,
            }
        }
        Err(FileSystemError::FileNotFound)
    }

    pub fn read_file(&mut self, path: &str) -> Result<Vec<u8>, FileSystemError> {
        for fs in &mut self.file_systems {
            match fs.read_file(path) {
                Ok(contents) => {
                    return Ok(contents);
                },
                Err(_) => continue,
            }
        }
        Err(FileSystemError::FileNotFound)
    }
}
