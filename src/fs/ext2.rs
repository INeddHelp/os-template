// Ext2 file system driver implementation for x86_64 architecture
// Requires a compatible storage device

use crate::storage::Storage;
use core::mem::size_of;

// File system constants
const EXT2_BLOCK_SIZE: usize = 1024;
const EXT2_INODE_SIZE: usize = 128;
const EXT2_SUPERBLOCK_OFFSET: u64 = 1024;
const EXT2_BLOCK_GROUP_OFFSET: u64 = 2048;

// File system driver struct
pub struct Ext2 {
    storage: Storage,
    block_size: usize,
    block_group_size: usize,
    inode_size: usize,
}

impl Ext2 {
    // Initialize the file system driver
    pub fn new(storage: Storage) -> Ext2 {
        let superblock = read_superblock(&storage);
        let block_size = EXT2_BLOCK_SIZE << superblock.log_block_size;
        let block_group_size = block_size * superblock.blocks_per_group as usize;
        let inode_size = EXT2_INODE_SIZE;

        Ext2 {
            storage,
            block_size,
            block_group_size,
            inode_size,
        }
    }

    // Read data from a file in the file system
    pub fn read_file(&mut self, path: &str, buffer: &mut [u8]) {
        // Find the inode for the file
        let inode = find_inode(path, &self.storage, &self.block_group_size);

        // Read the data blocks for the inode
        let mut block_buffer = [0; EXT2_BLOCK_SIZE];
        let mut position = 0;
        for block_index in &inode.block_pointers {
            if *block_index == 0 {
                break;
            }
            self.storage.read_block(*block_index as u64, &mut block_buffer);
            let bytes_to_read = buffer.len() - position;
            let bytes_from_block = bytes_to_read.min(EXT2_BLOCK_SIZE);
            buffer[position..position + bytes_from_block].copy_from_slice(&block_buffer[0..bytes_from_block]);
            position += bytes_from_block;
        }
    }
}

// Superblock struct for the ext2 file system
#[repr(C, packed)]
struct Superblock {
    _unused: u32,
    inodes_count: u32,
    blocks_count: u32,
    _unused2: u32,
    free_blocks_count: u32,
    free_inodes_count: u32,
    _unused3: u32,
    _unused4: u32,
    first_data_block: u32,
    log_block_size: u32,
    _unused5: u32,
    blocks_per_group: u32,
    _unused6: [u32; 8],
}

// Read the superblock from the storage device
fn read_superblock(storage: &Storage) -> Superblock {
    let mut superblock = Superblock {
        _unused: 0,
        inodes_count: 0,
        blocks_count: 0,
        _unused2: 0,
        free_blocks_count: 0,
        free_inodes_count: 0,
        _unused3: 0,
        _unused4: 0,
        first_data_block: 0,
        log_block_size: 0,
        _unused5: 0,
        blocks_per_group: 0,
        _unused6: [0; 8],
    };

    let superblock_buffer = &mut [0; EXT2_BLOCK_SIZE];
    storage.read_block(EXT2_SUPERBLOCK_OFFSET / EXT2_BLOCK_SIZE,superblock_buffer);
    unsafe {
        let superblock_ptr = superblock_buffer.as_mut Superblock;
        superblock = *superblock_ptr;
    }

    superblock
}

// Inode struct for the ext2 file system
#[repr(C)]
struct Inode {
    mode: u16,
    uid: u16,
    size: u32,
    access_time: u32,
    create_time: u32,
    modify_time: u32,
    delete_time: u32,
    gid: u16,
    links_count: u16,
    blocks_count: u32,
    flags: u32,
    osd1: u32,
    block_pointers: [u32; 15],
    generation: u32,
    extended_attributes: u32,
    size_high: u32,
    _unused: u32,
}

// Find the inode for a given file path
fn find_inode(path: &str, storage: &Storage, block_group_size: &usize) -> Inode {
    let path_parts = path.split('/');
    let mut current_inode = read_inode(2, storage, block_group_size); /* Root directory inode */

    for part in path_parts {
        if part.is_empty() {
            continue;
        }

        let directory_entries = read_directory_entries(&current_inode, storage);
        let directory_entry = directory_entries
            .iter()
            .find(|entry| entry.name == part)
            .expect("File not found");

        current_inode = read_inode(directory_entry.inode, storage, block_group_size);
    }

    current_inode
}

// Read the inode from the storage device
fn read_inode(inode_number: u32, storage: &Storage, block_group_size: &usize) -> Inode {
    let inode_size = size_of::<Inode>();
    let block_size = *block_group_size;
    let block_number = (inode_number - 1) / (block_size as u32 / inode_size as u32);
    let inode_offset = ((inode_number - 1) % (block_size as u32 / inode_size as u32)) as usize * inode_size;
    let block_offset = block_number as usize * block_size;
    let mut block_buffer = [0; EXT2_BLOCK_SIZE];
    storage.read_block(EXT2_BLOCK_GROUP_OFFSET + block_offset as u64, &mut block_buffer);
    let inode_buffer = &mut [0; 128];
    inode_buffer.copy_from_slice(&block_buffer[inode_offset..inode_offset + inode_size]);
    unsafe {
        let inode_ptr = inode_buffer.as_mut_ptr() as *mut Inode;
        *inode_ptr
    }
}

let inode_buffer = &mut [0; 128];
inode_buffer.copy_from_slice(&block_buffer[inode_offset..inode_offset + inode_size]);
unsafe {
    let inode_ptr = inode_buffer.as_mut_ptr() as *mut Inode;
    *inode_ptr
}

// Directory entry struct for the ext2 file system
#[repr(C)]
struct DirectoryEntry {
    inode: u32,
    size: u16,
    name_len: u8,
    file_type: u8,
    name: [u8; 256],
}

// Read directory entries from a directory inode
fn read_directory_entries(inode: &Inode, storage: &Storage) -> Vec<DirectoryEntry> {
    let mut directory_entries = Vec::new();
    let mut block_buffer = [0; EXT2_BLOCK_SIZE];
    for block_index in &inode.block_pointers {
        if *block_index == 0 {
            break;
        }
        storage.read_block(*block_index as u64, &mut block_buffer);
        let mut position = 0;
        while position < EXT2_BLOCK_SIZE {
            let directory_entry_buffer = &mut [0; 264];
            directory_entry_buffer.copy_from_slice(&block_buffer[position..position + 264]);
            let directory_entry = unsafe { *(directory_entry_buffer as *const DirectoryEntry) };
            directory_entries.push(*directory_entry);
            position += directory_entry.size as usize;
        }
    }

    directory_entries
}

// Read a file from the storage device
pub fn read_file(path: &str, storage: &Storage, block_group_size: usize) -> String {
    let inode = find_inode(path, storage, &block_group_size);
    let mut content = String::new();
    let mut block_buffer = [0; EXT2_BLOCK_SIZE];
    for block_index in &inode.block_pointers {
        if *block_index == 0 {
            break;
        }
        storage.read_block(*block_index as u64, &mut block_buffer);
        content.push_str(str::from_utf8(&block_buffer).unwrap());
    }

    content
}

// Write a file to the storage device
pub fn write_file(path: &str, content: &str, storage: &mut Storage, block_group_size: usize) {
    let mut parts = path.rsplitn(2, '/');
    let file_name = parts.next().unwrap();
    let directory_path = parts.next().unwrap_or("");
    let directory_inode = find_inode(directory_path, storage, &block_group_size);
    let directory_entries = read_directory_entries(&directory_inode, storage);
    let mut existing_entry: Option<&DirectoryEntry> = None;
    for entry in directory_entries.iter() {
        let name = str::from_utf8(&entry.name[..entry.name_len as usize]).unwrap();
        if name == file_name {
            existing_entry = Some(entry);
            break;
        }
    }

    if let Some(entry) = existing_entry {
        let inode = read_inode(entry.inode, storage, &block_group_size);
        let mut block_buffer = [0; EXT2_BLOCK_SIZE];
        let mut block_index = 0;
        for block_pointer in inode.block_pointers.iter() {
            if *block_pointer == 0 {
                break;
            }
            block_index += 1;
            storage.read_block(*block_pointer as u64, &mut block_buffer);
            let mut position = 0;
            while position < EXT2_BLOCK_SIZE {
                let size = min(EXT2_BLOCK_SIZE - position, content.len());
                block_buffer[position..position + size].copy_from_slice(&content[..size].as_bytes());
                content = &content[size..];
                position += size;
                if content.is_empty() {
                    break;
                }
            }
            storage.write_block(*block_pointer as u64, &block_buffer);
            block_buffer = [0; EXT2_BLOCK_SIZE];
        }
        if !content.is_empty() {
            let new_block = storage.allocate_block();
            block_index += 1;
            inode.block_pointers[block_index] = new_block as u32;
            storage.write_inode(entry.inode, &inode, &block_group_size);
            let mut position = 0;
            while position < EXT2_BLOCK_SIZE {
                let size = min(EXT2_BLOCK_SIZE - position, content.len());
                block_buffer[position..position + size].copy_from_slice(&content[..size].as_bytes());
                content = &content[size..];
                position += size;
                if content.is_empty() {
                    break;
                }
            }
            storage.write_block(new_block as u64, &block_buffer);
        }
    } else {
        let mut new_entry = DirectoryEntry {
            inode: 0,
            size: 0,
            name_len: 0,
            file_type: 0,
            name: [0; 256],
        };
        let new_inode = storage.allocate_inode();
        let mut inode = Inode {
            mode: 0o100644,
            uid: 0,
            size: 0,
            atime: 0,
            ctime: 0,
            mtime: 0,
            dtime: 0,
            gid: 0,
            links_count: 1,
            block_pointers: [0; 15],
            flags: 0,
            osd1: 0,
            generation: 0,
            file_acl: 0,
            size_high: 0,
            fragment_block: 0,
            osd2: [0; 2],
        };
        let new_block = storage.allocate_block();

        let mut position = 0;
        while position < new_entry.name.len() {
            let byte = file_name.as_bytes()[position];
            new_entry.name[position] = byte;
            if byte == 0 {
                break;
            }
            position += 1;
        }
        new_entry.name_len = position as u8;
        new_entry.inode = new_inode;
        new_entry.size = (8 + new_entry.name_len as u32 + 3) & !3;
        new_entry.file_type = 1; // regular file

        inode.block_pointers[0] = new_block as u32;
        inode.size = content.len() as u32;
        let mut position = 0;
        while position < EXT2_BLOCK_SIZE {
            let size = min(EXT2_BLOCK_SIZE - position, content.len());
            block_buffer[position..position + size].copy_from_slice(&content[..size].as_bytes());
            content = &content[size..];
            position += size;
            if content.is_empty() {
                break;
            }
        }
        storage.write_block(new_block as u64, &block_buffer);
        storage.write_inode(new_inode, &inode, &block_group_size);

        directory_entries.push(new_entry);
        let mut directory_buffer = [0; EXT2_BLOCK_SIZE];
        let mut position = 0;
        for entry in directory_entries.iter() {
            directory_buffer[position..position + 8].copy_from_slice(&entry.inode.to_le_bytes());
            directory_buffer[position + 4..position + 8].copy_from_slice(&entry.size.to_le_bytes());
            directory_buffer[position + 8..position + 9].copy_from_slice(&[entry.name_len]);
            directory_buffer[position + 9..position + 10].copy_from_slice(&[entry.file_type]);
            directory_buffer[position + 10..position + 8 + entry.name_len as usize].copy_from_slice(&entry.name[..entry.name_len as usize]);
            position += (8 + entry.name_len as usize + 3) & !3;
        }
        storage.write_block(directory_inode.block_pointers[0] as u64, &directory_buffer);
    }
}

// Allocate a block and write it with zeros
fn allocate_block(storage: &mut Storage) -> u32 {
    let block = storage.allocate_block();
    let mut buffer = [0; EXT2_BLOCK_SIZE];
    storage.write_block(block as u64, &buffer);
    block
}

// Find the inode corresponding to a file or directory
fn find_inode(path: &str, storage: &Storage, block_group_size: &usize) -> Inode {
    let mut inode = read_inode(EXT2_ROOT_INODE, storage, block_group_size);
    let mut components = path.split('/').filter(|c| !c.is_empty());
    while let Some(component) = components.next() {
        let directory_entries = read_directory_entries(&inode, storage);
        let mut found = false;
        for entry in directory_entries.iter() {
            let name = str::from_utf8(&entry.name[..entry.name_len
                as usize])
                .unwrap();
            if name == component {
                inode = read_inode(entry.inode, storage, block_group_size);
                found = true;
                break;
            }
        }
        if !found {
            panic!("File not found: {}", path);
        }
    }

    inode
}

// Read an inode from disk
fn read_inode(inode_number: u32, storage: &Storage, block_group_size: &usize) -> Inode {
    let block_group = (inode_number - 1) / (*block_group_size as u32);
    let offset = (inode_number - 1) % (*block_group_size as u32);
    let block_group_desc = storage.read_block_group_descriptor(block_group);
    let block_size = 1024 << block_group_desc.log_block_size;
    let mut buffer = vec![0; INODE_SIZE];
    let inode_table_block = block_group_desc.inode_table_address + (offset as u64 * INODE_SIZE as u64 / block_size);
    let inode_table_offset = (offset as usize * INODE_SIZE) % block_size as usize;
    storage.read(inode_table_block as usize * block_size as usize + inode_table_offset, &mut buffer);

    let mut inode = Inode::default();
    let mut cursor = Cursor::new(&buffer[..]);
    cursor.read_exact(&mut inode.mode.to_le_bytes()).unwrap();
    cursor.read_exact(&mut inode.uid.to_le_bytes()).unwrap();
    cursor.read_exact(&mut inode.size.to_le_bytes()).unwrap();
    cursor.read_exact(&mut inode.atime.to_le_bytes()).unwrap();
    cursor.read_exact(&mut inode.ctime.to_le_bytes()).unwrap();
    cursor.read_exact(&mut inode.mtime.to_le_bytes()).unwrap();
    cursor.read_exact(&mut inode.dtime.to_le_bytes()).unwrap();
    cursor.read_exact(&mut inode.gid.to_le_bytes()).unwrap();
    cursor.read_exact(&mut inode.links_count.to_le_bytes()).unwrap();
    cursor.read_exact(&mut inode.block_pointers[..]).unwrap();
    cursor.read_exact(&mut inode.flags.to_le_bytes()).unwrap();
    cursor.read_exact(&mut inode.osd1.to_le_bytes()).unwrap();
    cursor.read_exact(&mut inode.generation.to_le_bytes()).unwrap();
    cursor.read_exact(&mut inode.file_acl.to_le_bytes()).unwrap();
    cursor.read_exact(&mut inode.size_high.to_le_bytes()).unwrap();
    cursor.read_exact(&mut inode.fragment_block.to_le_bytes()).unwrap();
    cursor.read_exact(&mut inode.osd2[..]).unwrap();

    inode
}

// Read the directory entries from a directory inode
fn read_directory_entries(inode: &Inode, storage: &Storage) -> Vec<DirectoryEntry> {
    let mut directory_entries = Vec::new();
    let mut directory_buffer = [0; EXT2_BLOCK_SIZE];
    let block_size = EXT2_BLOCK_SIZE as u64;
    let mut position = 0;
    while position < inode.size {
        let block = position / block_size;
        let block_offset = position % block_size;
        storage.read_block(inode.block_pointers[block as usize] as u64, &mut directory_buffer);
        while position < inode.size && (position % block_size) + 8 <= block_size {
            let mut entry = DirectoryEntry::default();
            entry.inode = u32::from_le_bytes([
                directory_buffer[position],
                directory_buffer[position + 1],
                directory_buffer[position + 2],
                directory_buffer[position + 3],
            ]);
            entry.size = u32::from_le_bytes([
                                                directory_buffer[position + 4],
                                                directory_buffer[position + 5],
                                                directory_buffer[position + 6],
                                                directory_buffer[position + 7],
                                            ]);
            entry.name_len = directory_buffer[position + 8];
            entry.file_type = directory_buffer[position + 7];
            let name_len = entry.name_len as usize;
            let mut name_buffer = vec![0; name_len];
            for i in 0..name_len {
                name_buffer[i] = directory_buffer[position + 8 + i];
            }
            entry.name = String::from_utf8(name_buffer).unwrap();

            if entry.inode != 0 {
                directory_entries.push(entry);
            }

            position += entry.size as u64;
        }
    }

    directory_entries
}
