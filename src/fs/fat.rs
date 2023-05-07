use crate::filesystem::{FileSystem, FileSystemError, DirectoryEntry};

#[derive(Debug)]
pub struct FatFileSystem {
    // filesystem-specific data
}

impl FileSystem for FatFileSystem {
    fn read_directory(&mut self, path: &str) -> Result<Vec<DirectoryEntry>, FileSystemError> {
        // implementation for reading a directory in FAT filesystem
        // returns a vector of directory entries on success, or a FileSystemError on failure
        unimplemented!()
    }

    fn read_file(&mut self, path: &str) -> Result<Vec<u8>, FileSystemError> {
        // implementation for reading a file in FAT filesystem
        // returns the contents of the file as a byte vector on success, or a FileSystemError on failure
        unimplemented!()
    }
}
