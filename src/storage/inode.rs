use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

const INODE_SIZE: usize = 128;

struct Inode {
    id: u64,
    filename: String,
    size: usize,
    data: [u8; INODE_SIZE],
}

impl Inode {
    fn new(id: u64, filename: String, size: usize) -> Inode {
        Inode {
            id,
            filename,
            size,
            data: [0; INODE_SIZE],
        }
    }

    fn write_to_disk(&self, path: &Path) -> std::io::Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(path)?;

        file.write_all(self.filename.as_bytes())?;
        file.write_all(&self.id.to_le_bytes())?;
        file.write_all(&self.size.to_le_bytes())?;
        file.write_all(&self.data)?;

        Ok(())
    }

    fn read_from_disk(id: u64, path: &Path) -> std::io::Result<Inode> {
        let mut file = File::open(path)?;
        file.seek(SeekFrom::Start(id * INODE_SIZE as u64))?;

        let mut filename_buf = [0; 32];
        file.read_exact(&mut filename_buf)?;

        let filename = String::from_utf8_lossy(&filename_buf).trim_end_matches('\0').to_string();

        let mut id_buf = [0; 8];
        file.read_exact(&mut id_buf)?;
        let id = u64::from_le_bytes(id_buf);

        let mut size_buf = [0; 8];
        file.read_exact(&mut size_buf)?;
        let size = usize::from_le_bytes(size_buf);

        let mut data = [0; INODE_SIZE];
        file.read_exact(&mut data)?;

        Ok(Inode {
            id,
            filename,
            size,
            data,
        })
    }
}
